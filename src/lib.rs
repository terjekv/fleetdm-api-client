//! # FleetDM API Client
//!
//! An unofficial asynchronous Rust client for the Fleet REST API, with typed
//! coverage for common workflows and generated wrappers for the pinned route set.
//!
//! ## Features
//!
//! - **Type-safe authentication**: Uses typestate pattern to ensure authentication at compile-time
//! - **Async/await**: Built on tokio and reqwest for efficient async operations
//! - **Comprehensive error handling**: Rich error types with detailed context and full API response data
//! - **Fluent builders**: Ergonomic query builders for complex API calls with type-safe parameters
//! - **Opt-in retry logic**: Configurable retry policies with exponential backoff for rate limiting (HTTP 429)
//! - **Display support**: Common query enums implement `Display` for ergonomic logging and string formatting
//! - **Optional tracing**: Built-in tracing support for debugging (enable with `tracing` feature)
//!
//! ## Quick Start
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! fleetdm-api-client = "0.0.1"
//! tokio = { version = "1", features = ["full"] }
//! ```
//!
//! ## Authentication
//!
//! There are two ways to authenticate:
//!
//! ### Email and Password Login
//!
//! ```no_run
//! use fleetdm_api_client::FleetClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = FleetClient::builder("https://fleet.example.com")?
//!         .login("admin@example.com", "password")
//!         .await?
//!         .build();
//!     Ok(())
//! }
//! ```
//!
//! ### API Token
//!
//! ```no_run
//! use fleetdm_api_client::FleetClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = FleetClient::builder("https://fleet.example.com")?
//!         .with_token("your-api-token")?
//!         .build();
//!     Ok(())
//! }
//! ```
//!
//! ## Usage Examples
//!
//! ### List Hosts with Filters
//!
//! ```no_run
//! use fleetdm_api_client::FleetClient;
//! use fleetdm_api_client::models::HostStatus;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = FleetClient::builder("https://fleet.example.com")?
//!         .with_token("your-api-token")?
//!         .build();
//!
//!     // List online hosts with pagination
//!     let hosts = client.hosts()
//!         .list()
//!         .status(HostStatus::Online)
//!         .per_page(50)
//!         .send()
//!         .await?;
//!
//!     for host in hosts.hosts() {
//!         println!("{}: {}", host.hostname(), host.status());
//!     }
//!     Ok(())
//! }
//! ```
//!
//! See `examples/hosts.rs` for more examples.
//!
//! ### Report Management
//!
//! ```no_run
//! use fleetdm_api_client::FleetClient;
//! use fleetdm_api_client::models::report::CreateReportRequest;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = FleetClient::builder("https://fleet.example.com")?
//!         .with_token("your-api-token")?
//!         .build();
//!
//!     // Create a new report
//!     let report = client.reports()
//!         .create(CreateReportRequest {
//!             name: "CPU Info".to_string(),
//!             query: "SELECT * FROM cpus;".to_string(),
//!             description: Some("Get CPU information".to_string()),
//!             fleet_id: None,
//!             interval: None,
//!             platform: None,
//!             discard_data: None,
//!             ..Default::default()
//!         })
//!         .await?;
//!
//!     println!("Created report: {}", report.report().name());
//!     Ok(())
//! }
//! ```
//!
//! See `examples/reports.rs` for more examples.
//!
//! ## Retry Policies
//!
//! Opt-in retry with exponential backoff helps handle rate limiting. Configure it at client creation:
//!
//! ```no_run
//! use fleetdm_api_client::{FleetClient, RetryPolicy};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = FleetClient::builder("https://fleet.example.com")?
//!         .with_retry_policy(RetryPolicy::conservative())  // 3 retries, 1-30s delays
//!         .with_token("token")?
//!         .build();
//!     Ok(())
//! }
//! ```
//!
//! Available policies:
//! - `RetryPolicy::conservative()` - 3 retries, 1-30s delays (recommended for production)
//! - `RetryPolicy::aggressive()` - 5 retries, 500ms-60s delays (good for testing)
//! - `RetryPolicy::none()` - Disable retries
//!
//! See `examples/retry.rs` for detailed retry configuration examples.
//!
//! ## Error Handling
//!
//! The client provides detailed error information:
//!
//! ```no_run
//! use fleetdm_api_client::FleetClient;
//! use fleetdm_api_client::error::FleetError;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = FleetClient::builder("https://fleet.example.com")?
//!         .with_token("your-api-token")?
//!         .build();
//!
//!     match client.hosts().get(999).await {
//!         Ok(host) => println!("Found: {}", host.host().hostname()),
//!         Err(FleetError::NotFound(msg)) => println!("Host not found: {}", msg),
//!         Err(FleetError::Api { response, .. }) => {
//!             println!("API error with details: {:?}", response.errors);
//!         }
//!         Err(e) => println!("Error: {}", e),
//!     }
//!     Ok(())
//! }
//! ```
//!
//! ## Display Support for Query Enums
//!
//! Common query and filter enum types implement `Display` for ergonomic formatting:
//!
//! ```
//! use fleetdm_api_client::models::{HostStatus, OrderDirection};
//! use fleetdm_api_client::models::host::HostOrderKey;
//!
//! let status = HostStatus::Online;
//! println!("Status: {}", status);        // Prints: "online"
//!
//! let order = HostOrderKey::Hostname;
//! println!("Sorting by: {}", order);     // Prints: "hostname"
//!
//! let direction = OrderDirection::Desc;
//! println!("Direction: {}", direction);  // Prints: "desc"
//! ```
//!
//! See `examples/display_demo.rs` for more display examples.
//!
//! ## Tracing Support
//!
//! Enable optional request/response logging by adding the `tracing` feature:
//!
//! ```toml
//! [dependencies]
//! fleetdm-api-client = { version = "0.0.1", features = ["tracing"] }
//! tracing = "0.1"
//! tracing-subscriber = "0.3"
//! ```
//!
//! Provides automatic logging of:
//! - Successful responses
//! - Rate limit errors and retry attempts
//! - Authentication failures
//! - API errors with detailed context
//!
//! ## Examples Directory
//!
//! - `auth.rs` and `authentication.rs` - Public and token-based authentication examples
//! - `hosts.rs` - List, filter, and manage hosts
//! - `reports.rs` and `queries.rs` - Current report routes and legacy query terminology
//! - `policies.rs` - Create and manage policies
//! - `teams.rs` - Legacy team management
//! - `users.rs` - User management
//! - `display_demo.rs` - Display trait demonstration
//! - `retry.rs` - Retry policy configuration
//! - And many more in the `examples/` directory
//!
//! ## Compile-Time Safety
//!
//! The client uses Rust's type system to prevent common mistakes:
//!
//! ```compile_fail
//! use fleetdm_api_client::FleetClient;
//!
//! // This will NOT compile - authentication is required!
//! let client = FleetClient::builder("https://fleet.example.com")
//!     .unwrap()
//!     .build();  // ERROR: build() not available without auth
//! ```
//!
//! You must explicitly authenticate:
//!
//! ```no_run
//! # use fleetdm_api_client::FleetClient;
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = FleetClient::builder("https://fleet.example.com")?
//!     .with_token("token")?
//!     .build();  // OK - authentication verified at compile-time
//! # Ok(())
//! # }
//! ```

pub mod client;
pub mod endpoints;
pub mod error;
pub mod http;
pub mod models;
pub mod retry;

// Internal path definitions
mod paths;

// Re-export commonly used types
pub use client::{Authenticated, FleetClient, FleetClientBuilder, Unauthenticated};
pub use error::{FleetError, Result};
pub use retry::RetryPolicy;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_requires_auth() {
        // Verify that authentication is required at compile time
        let builder = FleetClient::builder("https://fleet.example.com").unwrap();

        // This would not compile without authentication:
        // let client = builder.build();

        // Must authenticate first:
        let authenticated = builder.with_token("test-token").unwrap();
        let _client = authenticated.build();
    }
}
