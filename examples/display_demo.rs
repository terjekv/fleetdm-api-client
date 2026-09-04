use fleetdm_api_client::models::host::HostOrderKey;
/// Demonstration of Display support for enums in the FleetDM API client
///
/// This example shows how Display support makes it easier to work with enums
/// by enabling string formatting without manual method calls.
use fleetdm_api_client::models::{HostStatus, OrderDirection};

fn main() {
    println!("=== FleetDM API Client - Display Support Demo ===\n");

    // Display works with status enums
    let statuses = [
        HostStatus::Online,
        HostStatus::Offline,
        HostStatus::New,
        HostStatus::Missing,
    ];
    println!("Host Statuses:");
    for status in &statuses {
        println!("  - Status: {}", status); // Now works with Display!
    }
    println!();

    // Display works with OrderDirection
    println!("Sort Directions:");
    println!("  - Ascending: {}", OrderDirection::Asc);
    println!("  - Descending: {}", OrderDirection::Desc);
    println!();

    // Display works with OrderKey enums
    let order_keys = [
        HostOrderKey::Hostname,
        HostOrderKey::ComputerName,
        HostOrderKey::Status,
        HostOrderKey::CreatedAt,
    ];
    println!("Host Order Keys:");
    for key in &order_keys {
        println!("  - Order key: {}", key); // Now works with Display!
    }
    println!();

    // Display is useful in logging and error messages
    println!("Practical uses:");
    println!("  1. Logging: filter(status: {})", HostStatus::Online);
    println!(
        "  2. Error messages: Unsupported sort: {}",
        HostOrderKey::Hostname
    );
    println!(
        "  3. Config files: direction = \"{}\"",
        OrderDirection::Desc
    );

    // The old as_str() method still works for backwards compatibility
    println!("\nBackwards Compatibility:");
    println!(
        "  - as_str() still works: {}",
        HostOrderKey::Hostname.as_str()
    );
}
