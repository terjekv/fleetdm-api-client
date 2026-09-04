# LDAP role synchronization example

`examples/ldap_group_user_sync.rs` demonstrates pre-populating Fleet SSO users
from LDAP group membership. It is an operational example, not a general-purpose
identity synchronization daemon.

The example:

- maps one or more LDAP group DNs to Fleet `admin`, `maintainer`, or `observer`
  roles;
- reads usernames from a configurable LDAP group attribute;
- looks up each user's email address;
- creates missing SSO users and updates changed global roles;
- gives the highest role to users present in multiple mapped groups;
- can optionally delete Fleet SSO users not present in any mapped group;
- can run once or repeatedly on an interval.

## Configuration

Copy the example environment file and replace every placeholder:

```bash
cp examples/ldap_group_user_sync.env.example .env
```

The program reads process environment variables; it does not load `.env` by
itself. Export the variables with your preferred secret-management mechanism
before running:

```bash
cargo run --example ldap_group_user_sync
```

Required Fleet settings:

- `FLEET_URL`: HTTPS Fleet base URL without a path, query, or fragment.
- `FLEET_TOKEN`: least-privilege API token able to list, create, update, and—if
  cleanup is enabled—delete users.
- `FLEET_USERS_PER_PAGE`: optional positive page size; defaults to 500.

Required LDAP settings:

- `LDAP_URL`
- `LDAP_BIND_DN`
- `LDAP_BIND_PASSWORD`
- `LDAP_GROUP_MEMBER_ATTRIBUTE`
- `LDAP_USER_EMAIL_ATTRIBUTE`
- at least one of `LDAP_GROUPS_FOR_ADMIN`, `LDAP_GROUPS_FOR_MAINTAINER`, or
  `LDAP_GROUPS_FOR_OBSERVER`

Group variables accept JSON arrays, which are recommended for full DNs:

```bash
LDAP_GROUPS_FOR_ADMIN='["cn=fleet-admins,ou=groups,dc=example,dc=com"]'
```

Comma-separated values are supported only when individual values do not contain
commas.

## User lookup modes

For direct DN lookup, configure a template:

```bash
LDAP_USER_DN_TEMPLATE="uid={username},ou=people,dc=example,dc=com"
```

For subtree search, omit that variable and configure both:

```bash
LDAP_USER_SEARCH_BASE_DN="ou=people,dc=example,dc=com"
LDAP_USER_LOOKUP_FILTER_TEMPLATE="(uid={username})"
```

The example escapes usernames as LDAP DN values or search-filter assertion
values before substituting `{username}`. The surrounding template remains
administrator-controlled configuration and must be a valid DN or LDAP filter.

## Cleanup and scheduling

Cleanup is disabled by default. Enabling it deletes every SSO-enabled Fleet user
whose normalized email is absent from the mapped LDAP groups:

```bash
LDAP_SYNC_CLEANUP_SSO=true
```

This is intentionally destructive. Validate group queries, email mapping, role
precedence, Fleet permissions, and failure behavior in a test environment first.
Non-SSO users are not selected for cleanup.

Run continuously by setting a positive interval:

```bash
SYNC_INTERVAL_SECONDS=3600
```

An iteration error is logged and the loop continues after the interval. Use an
external supervisor for restarts, health checks, structured logging, and secret
rotation in production.

## Security notes

- Prefer LDAPS and validate the LDAP server trust configuration.
- Keep the LDAP bind account and Fleet token least-privileged.
- Supply secrets through a secret manager rather than a checked-in shell file.
- Review output handling because email addresses and group DNs are logged.
- Test cleanup with `LDAP_SYNC_CLEANUP_SSO` unset before enabling deletion.
