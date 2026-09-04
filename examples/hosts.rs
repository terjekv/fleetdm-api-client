use fleetdm_api_client::{FleetClient, models::HostStatus};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example 1: Authenticate with email and password
    let _client = FleetClient::builder("https://fleet.example.com")?
        .login("admin@example.com", "secure-password")
        .await?
        .build();

    // Example 2: Authenticate with API token (more common for automation)
    let client = FleetClient::builder("https://fleet.example.com")?
        .with_token("your-api-token-here")?
        .build();

    // List all online hosts
    println!("Fetching online hosts...");
    let online_hosts = client
        .hosts()
        .list()
        .status(HostStatus::Online)
        .per_page(100)
        .send()
        .await?;

    println!("Found {} online hosts", online_hosts.hosts().len());

    for host in online_hosts.hosts() {
        println!(
            "  - {} ({}) - {} - Last seen: {}",
            host.hostname(),
            host.display_name(),
            host.platform(),
            host.seen_time()
        );
    }

    // Get detailed information about a specific host
    if let Some(first_host) = online_hosts.hosts().first() {
        println!("\nFetching details for host ID {}...", first_host.id());
        let host_details = client.hosts().get(first_host.id()).await?;

        println!("Host details:");
        println!("  UUID: {}", host_details.host().uuid());
        println!("  Computer name: {}", host_details.host().computer_name());
        println!("  Memory: {} bytes", host_details.host().memory());
        println!("  Uptime: {} seconds", host_details.host().uptime());

        if let Some(os_version) = host_details.host().os_version() {
            println!("  OS: {} {}", os_version.name(), os_version.version());
        }

        if let Some(team_name) = host_details.host().team_name() {
            println!("  Team: {}", team_name);
        }
    }

    // List hosts filtered by team
    println!("\nFetching hosts for team ID 1...");
    let team_hosts = client.hosts().list().team_id(1).per_page(50).send().await?;

    println!("Found {} hosts in team", team_hosts.hosts().len());

    // Search for hosts by query
    println!("\nSearching for macOS hosts...");
    let mac_hosts = client.hosts().list().query("macOS").send().await?;

    println!("Found {} macOS hosts", mac_hosts.hosts().len());

    // Request a refetch of host data
    if let Some(host) = mac_hosts.hosts().first() {
        println!("\nRequesting refetch for host {}...", host.hostname());
        client.hosts().refetch(host.id()).await?;
        println!("Refetch requested successfully");
    }

    Ok(())
}
