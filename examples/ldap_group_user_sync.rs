use fleetdm_api_client::{
    FleetClient,
    models::user::{CreateUserRequest, UpdateUserRequest, User},
};
use ldap3::{LdapConnAsync, Scope, SearchEntry};
use std::collections::{HashMap, HashSet};
use std::env;
use std::error::Error;
use std::io::{Error as IoError, ErrorKind};
use std::time::Duration;

#[derive(Debug, Clone)]
struct GroupRoleMapping {
    group_dn: String,
    role: String,
}

#[derive(Debug, Clone)]
struct LdapConfig {
    url: String,
    bind_dn: String,
    bind_password: String,
    group_member_attribute: String,
    group_role_mappings: Vec<GroupRoleMapping>,
    user_dn_template: Option<String>,
    user_search_base_dn: Option<String>,
    user_lookup_filter_template: Option<String>,
    user_email_attribute: String,
}

#[derive(Debug, Clone)]
struct SyncConfig {
    fleet_url: String,
    fleet_token: String,
    cleanup_unmapped_sso: bool,
    fleet_users_per_page: u32,
    sync_interval_seconds: Option<u64>,
}

fn invalid_input(message: impl Into<String>) -> Box<dyn Error> {
    Box::new(IoError::new(ErrorKind::InvalidInput, message.into()))
}

fn normalize_email(email: &str) -> String {
    email.trim().to_ascii_lowercase()
}

fn role_rank(role: &str) -> u8 {
    match role {
        "admin" => 3,
        "maintainer" => 2,
        "observer" => 1,
        _ => 0,
    }
}

fn display_name_from_email(email: &str) -> String {
    let local_part = email.split('@').next().unwrap_or("LDAP User");
    let mut words = Vec::new();

    for part in local_part.split(['.', '_', '-']) {
        if part.is_empty() {
            continue;
        }

        let mut chars = part.chars();
        let Some(first) = chars.next() else {
            continue;
        };
        let mut word = first.to_uppercase().collect::<String>();
        word.push_str(chars.as_str());
        words.push(word);
    }

    if words.is_empty() {
        "LDAP User".to_string()
    } else {
        words.join(" ")
    }
}

fn env_flag(name: &str) -> bool {
    env::var(name)
        .map(|value| {
            let normalized = value.trim().to_ascii_lowercase();
            matches!(normalized.as_str(), "1" | "true" | "yes" | "y" | "on")
        })
        .unwrap_or(false)
}

fn required_env(name: &str) -> Result<String, Box<dyn Error>> {
    env::var(name).map_err(|_| invalid_input(format!("{name} must be set")))
}

fn parse_optional_u64_env(name: &str) -> Result<Option<u64>, Box<dyn Error>> {
    match env::var(name) {
        Ok(value) => {
            let parsed = value.trim().parse::<u64>().map_err(|error| {
                invalid_input(format!("{name} must be a positive integer: {error}"))
            })?;
            if parsed == 0 {
                return Err(invalid_input(format!("{name} must be greater than 0")));
            }
            Ok(Some(parsed))
        }
        Err(_) => Ok(None),
    }
}

fn parse_u32_env_with_default(name: &str, default: u32) -> Result<u32, Box<dyn Error>> {
    match env::var(name) {
        Ok(value) => {
            let parsed = value.trim().parse::<u32>().map_err(|error| {
                invalid_input(format!("{name} must be a positive integer: {error}"))
            })?;
            if parsed == 0 {
                return Err(invalid_input(format!("{name} must be greater than 0")));
            }
            Ok(parsed)
        }
        Err(_) => Ok(default),
    }
}

fn template_username(template: &str, username: &str) -> String {
    template.replace("{username}", username)
}

fn escape_ldap_filter_value(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '\0' => escaped.push_str("\\00"),
            '(' => escaped.push_str("\\28"),
            ')' => escaped.push_str("\\29"),
            '*' => escaped.push_str("\\2a"),
            '\\' => escaped.push_str("\\5c"),
            _ => escaped.push(character),
        }
    }
    escaped
}

fn escape_ldap_dn_value(value: &str) -> String {
    let characters: Vec<char> = value.chars().collect();
    let mut escaped = String::with_capacity(value.len());
    for (index, character) in characters.iter().copied().enumerate() {
        if character == '\0' {
            escaped.push_str("\\00");
            continue;
        }

        let at_start = index == 0;
        let at_end = index + 1 == characters.len();
        let requires_escape = matches!(character, ',' | '+' | '"' | '\\' | '<' | '>' | ';' | '=')
            || (at_start && matches!(character, ' ' | '#'))
            || (at_end && character == ' ');
        if requires_escape {
            escaped.push('\\');
        }
        escaped.push(character);
    }
    escaped
}

fn parse_group_dns_env(name: &str) -> Result<Vec<String>, Box<dyn Error>> {
    match env::var(name) {
        Ok(value) => {
            let trimmed = value.trim();
            if trimmed.starts_with('[') {
                let parsed: Vec<String> = serde_json::from_str(trimmed).map_err(|error| {
                    invalid_input(format!(
                        "{name} must be a valid JSON array of strings: {error}"
                    ))
                })?;
                return Ok(parsed
                    .into_iter()
                    .map(|group_dn| group_dn.trim().to_string())
                    .filter(|group_dn| !group_dn.is_empty())
                    .collect());
            }

            Ok(trimmed
                .split(',')
                .map(|raw| raw.trim().to_string())
                .filter(|group_dn| !group_dn.is_empty())
                .collect())
        }
        Err(env::VarError::NotPresent) => Ok(Vec::new()),
        Err(error) => Err(invalid_input(format!("failed reading {name}: {error}"))),
    }
}

fn load_group_role_mappings() -> Result<Vec<GroupRoleMapping>, Box<dyn Error>> {
    let mut group_role_mappings = Vec::new();
    for (role, env_key) in [
        ("admin", "LDAP_GROUPS_FOR_ADMIN"),
        ("maintainer", "LDAP_GROUPS_FOR_MAINTAINER"),
        ("observer", "LDAP_GROUPS_FOR_OBSERVER"),
    ] {
        let group_dns = parse_group_dns_env(env_key)?;
        for group_dn in group_dns {
            group_role_mappings.push(GroupRoleMapping {
                group_dn,
                role: role.to_string(),
            });
        }
    }

    if group_role_mappings.is_empty() {
        return Err(invalid_input(
            "set one or more of LDAP_GROUPS_FOR_ADMIN, LDAP_GROUPS_FOR_MAINTAINER, LDAP_GROUPS_FOR_OBSERVER",
        ));
    }

    Ok(group_role_mappings)
}

fn load_sync_config() -> Result<SyncConfig, Box<dyn Error>> {
    Ok(SyncConfig {
        fleet_url: required_env("FLEET_URL")?,
        fleet_token: required_env("FLEET_TOKEN")?,
        cleanup_unmapped_sso: env_flag("LDAP_SYNC_CLEANUP_SSO"),
        fleet_users_per_page: parse_u32_env_with_default("FLEET_USERS_PER_PAGE", 500)?,
        sync_interval_seconds: parse_optional_u64_env("SYNC_INTERVAL_SECONDS")?,
    })
}

fn load_ldap_config() -> Result<LdapConfig, Box<dyn Error>> {
    let group_role_mappings = load_group_role_mappings()?;

    let user_dn_template = env::var("LDAP_USER_DN_TEMPLATE").ok();
    let user_search_base_dn = env::var("LDAP_USER_SEARCH_BASE_DN").ok();
    let user_lookup_filter_template = env::var("LDAP_USER_LOOKUP_FILTER_TEMPLATE").ok();

    if user_dn_template.is_none() && user_search_base_dn.is_none() {
        return Err(invalid_input(
            "set LDAP_USER_DN_TEMPLATE or LDAP_USER_SEARCH_BASE_DN for user lookup",
        ));
    }
    if user_dn_template.is_none() && user_lookup_filter_template.is_none() {
        return Err(invalid_input(
            "LDAP_USER_LOOKUP_FILTER_TEMPLATE must be set when LDAP_USER_DN_TEMPLATE is not set",
        ));
    }

    Ok(LdapConfig {
        url: required_env("LDAP_URL")?,
        bind_dn: required_env("LDAP_BIND_DN")?,
        bind_password: required_env("LDAP_BIND_PASSWORD")?,
        group_member_attribute: required_env("LDAP_GROUP_MEMBER_ATTRIBUTE")?,
        group_role_mappings,
        user_dn_template,
        user_search_base_dn,
        user_lookup_filter_template,
        user_email_attribute: required_env("LDAP_USER_EMAIL_ATTRIBUTE")?,
    })
}

async fn fetch_group_usernames(
    ldap: &mut ldap3::Ldap,
    group_dn: &str,
    member_attribute: &str,
) -> Result<Vec<String>, Box<dyn Error>> {
    let (entries, _result) = ldap
        .search(
            group_dn,
            Scope::Base,
            "(objectClass=*)",
            vec![member_attribute],
        )
        .await?
        .success()?;

    let Some(entry) = entries.into_iter().next() else {
        return Ok(Vec::new());
    };

    let entry = SearchEntry::construct(entry);
    Ok(entry
        .attrs
        .get(member_attribute)
        .cloned()
        .unwrap_or_default())
}

async fn lookup_email_for_username(
    ldap: &mut ldap3::Ldap,
    ldap_config: &LdapConfig,
    username: &str,
) -> Result<Option<String>, Box<dyn Error>> {
    let (entries, _result) = if let Some(dn_template) = &ldap_config.user_dn_template {
        let user_dn = template_username(dn_template, &escape_ldap_dn_value(username));
        ldap.search(
            &user_dn,
            Scope::Base,
            "(objectClass=*)",
            vec![ldap_config.user_email_attribute.as_str()],
        )
        .await?
        .success()?
    } else {
        let search_base = ldap_config.user_search_base_dn.as_deref().ok_or_else(|| {
            invalid_input(
                "LDAP_USER_SEARCH_BASE_DN must be set when LDAP_USER_DN_TEMPLATE is not set",
            )
        })?;
        let filter_template = ldap_config
            .user_lookup_filter_template
            .as_deref()
            .ok_or_else(|| {
                invalid_input(
                    "LDAP_USER_LOOKUP_FILTER_TEMPLATE must be set when LDAP_USER_DN_TEMPLATE is not set",
                )
            })?;
        let filter = template_username(filter_template, &escape_ldap_filter_value(username));
        ldap.search(
            search_base,
            Scope::Subtree,
            &filter,
            vec![ldap_config.user_email_attribute.as_str()],
        )
        .await?
        .success()?
    };

    let Some(entry) = entries.into_iter().next() else {
        return Ok(None);
    };

    let entry = SearchEntry::construct(entry);
    let email = entry
        .attrs
        .get(&ldap_config.user_email_attribute)
        .and_then(|values| values.first())
        .map(|value| normalize_email(value));

    Ok(email)
}

async fn desired_users_from_ldap(
    ldap: &mut ldap3::Ldap,
    ldap_config: &LdapConfig,
) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut desired: HashMap<String, String> = HashMap::new();
    let mut username_to_email_cache: HashMap<String, Option<String>> = HashMap::new();

    for mapping in &ldap_config.group_role_mappings {
        println!(
            "LDAP group {} => Fleet role {}",
            mapping.group_dn, mapping.role
        );
        let usernames =
            fetch_group_usernames(ldap, &mapping.group_dn, &ldap_config.group_member_attribute)
                .await?;

        for raw_username in usernames {
            let username = raw_username.trim().to_string();
            if username.is_empty() {
                continue;
            }

            let email = if let Some(cached) = username_to_email_cache.get(&username) {
                cached.clone()
            } else {
                let looked_up = lookup_email_for_username(ldap, ldap_config, &username).await?;
                username_to_email_cache.insert(username.clone(), looked_up.clone());
                looked_up
            };

            let Some(email) = email else {
                eprintln!(
                    "Skipping username '{}' from group '{}': no '{}' attribute found",
                    username, mapping.group_dn, ldap_config.user_email_attribute
                );
                continue;
            };

            let should_replace = desired
                .get(&email)
                .map(|existing_role| role_rank(&mapping.role) > role_rank(existing_role))
                .unwrap_or(true);

            if should_replace {
                desired.insert(email, mapping.role.clone());
            }
        }
    }

    Ok(desired)
}

async fn fetch_all_fleet_users(
    client: &FleetClient,
    per_page: u32,
) -> Result<Vec<User>, Box<dyn Error>> {
    let mut users = Vec::new();
    let mut page = 1u32;

    loop {
        let response = client
            .users()
            .list()
            .page(page)
            .per_page(per_page)
            .send()
            .await?;
        let current_page_users = response.users();
        if current_page_users.is_empty() {
            break;
        }

        users.extend(current_page_users.iter().cloned());
        if current_page_users.len() < per_page as usize {
            break;
        }

        page += 1;
    }

    Ok(users)
}

async fn run_sync_once(
    sync_config: &SyncConfig,
    ldap_config: &LdapConfig,
) -> Result<(), Box<dyn Error>> {
    let client = FleetClient::builder(&sync_config.fleet_url)?
        .with_token(&sync_config.fleet_token)?
        .build();

    let (conn, mut ldap) = LdapConnAsync::new(&ldap_config.url).await?;
    ldap3::drive!(conn);
    ldap.simple_bind(&ldap_config.bind_dn, &ldap_config.bind_password)
        .await?
        .success()?;

    println!("=== LDAP Group User Pre-population Sync ===\n");

    let desired_roles = desired_users_from_ldap(&mut ldap, ldap_config).await?;
    println!(
        "\nResolved {} LDAP users with valid email addresses.",
        desired_roles.len()
    );

    let all_fleet_users = fetch_all_fleet_users(&client, sync_config.fleet_users_per_page).await?;
    let mut existing_by_email: HashMap<String, (u64, Option<String>, bool)> = HashMap::new();

    for user in &all_fleet_users {
        existing_by_email.insert(
            normalize_email(user.email()),
            (
                user.id(),
                user.global_role().map(|role| role.to_string()),
                user.sso_enabled(),
            ),
        );
    }

    let mut created = 0u32;
    let mut updated = 0u32;

    for (email, desired_role) in &desired_roles {
        match existing_by_email.get(email) {
            Some((user_id, current_role, _sso_enabled)) => {
                if current_role.as_deref() != Some(desired_role.as_str()) {
                    client
                        .users()
                        .update(
                            *user_id,
                            UpdateUserRequest {
                                name: None,
                                email: None,
                                enabled: None,
                                global_role: Some(desired_role.clone()),
                                teams: None,
                            },
                        )
                        .await?;

                    println!("Updated {} => role {}", email, desired_role);
                    updated += 1;
                }
            }
            None => {
                client
                    .users()
                    .create(CreateUserRequest {
                        name: display_name_from_email(email),
                        email: email.clone(),
                        password: None,
                        global_role: Some(desired_role.clone()),
                        sso_enabled: Some(true),
                        api_only: Some(false),
                        teams: None,
                    })
                    .await?;

                println!("Created SSO user {} => role {}", email, desired_role);
                created += 1;
            }
        }
    }

    let mut deleted = 0u32;
    if sync_config.cleanup_unmapped_sso {
        let desired_emails: HashSet<String> = desired_roles.keys().cloned().collect();
        let mut users_to_delete = Vec::new();

        for user in &all_fleet_users {
            let email = normalize_email(user.email());
            if user.sso_enabled() && !desired_emails.contains(&email) {
                users_to_delete.push((user.id(), email));
            }
        }

        for (user_id, email) in users_to_delete {
            client.users().delete(user_id).await?;
            println!("Deleted unmanaged SSO user {}", email);
            deleted += 1;
        }
    } else {
        println!("Cleanup disabled (set LDAP_SYNC_CLEANUP_SSO=true to enable).");
    }

    ldap.unbind().await?;

    println!(
        "\nSync complete: created {}, updated {}, deleted {}.",
        created, updated, deleted
    );

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let sync_config = load_sync_config()?;
    let ldap_config = load_ldap_config()?;

    if let Some(interval_seconds) = sync_config.sync_interval_seconds {
        loop {
            if let Err(error) = run_sync_once(&sync_config, &ldap_config).await {
                eprintln!("Sync failed: {error}");
            }
            println!("Sleeping {} seconds until next sync...", interval_seconds);
            tokio::time::sleep(Duration::from_secs(interval_seconds)).await;
        }
    }

    run_sync_once(&sync_config, &ldap_config).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_usernames_inserted_into_ldap_filters() {
        assert_eq!(
            escape_ldap_filter_value("alice*)(uid=*)"),
            "alice\\2a\\29\\28uid=\\2a\\29"
        );
        assert_eq!(escape_ldap_filter_value("a\\b\0"), "a\\5cb\\00");
    }

    #[test]
    fn escapes_usernames_inserted_into_ldap_distinguished_names() {
        assert_eq!(escape_ldap_dn_value("#admin"), "\\#admin");
        assert_eq!(escape_ldap_dn_value("Doe, John "), "Doe\\, John\\ ");
        assert_eq!(escape_ldap_dn_value("a\\b\0"), "a\\\\b\\00");
    }
}
