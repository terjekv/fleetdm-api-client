use crate::endpoints::{
    ActivitiesEndpoint, AuthEndpoint, CarvesEndpoint, CertificatesEndpoint, CommandsEndpoint,
    ConditionalAccessEndpoint, ConfigEndpoint, FleetsEndpoint, HostsEndpoint, IntegrationsEndpoint,
    InvitationsEndpoint, LabelsEndpoint, OsSettingsEndpoint, PoliciesEndpoint, PublicAuthEndpoint,
    QueriesEndpoint, RawApiEndpoint, ReportsEndpoint, ScriptsEndpoint, SessionsEndpoint,
    SetupExperienceEndpoint, SoftwareEndpoint, TargetsEndpoint, TeamsEndpoint, TranslatorEndpoint,
    UsersEndpoint, VersionEndpoint, VulnerabilitiesEndpoint,
};
use crate::error::{FleetError, Result};
use crate::models::auth::{LoginRequest, LoginResponse};
use crate::retry::RetryPolicy;
use reqwest::{Client as HttpClient, header};
use std::net::IpAddr;
use std::time::Duration;
use url::Url;

#[derive(serde::Deserialize)]
struct MfaRequiredResponse {
    message: String,
}

/// Marker type for unauthenticated state
pub struct Unauthenticated;

/// Marker type for authenticated state
pub struct Authenticated {
    token: String,
}

/// Builder for creating a FleetDM client with typestate pattern
///
/// The builder enforces authentication at compile-time. You must call
/// either `login()` or `with_token()` before you can build the client.
pub struct FleetClientBuilder<State> {
    base_url: Url,
    http_client: HttpClient,
    retry_policy: Option<RetryPolicy>,
    state: State,
}

impl FleetClientBuilder<Unauthenticated> {
    /// Create a new builder with the FleetDM instance URL
    pub fn new(base_url: impl AsRef<str>) -> Result<Self> {
        let base_url = Url::parse(base_url.as_ref())?;
        validate_base_url(&base_url)?;

        let http_client = HttpClient::builder()
            .timeout(Duration::from_secs(30))
            .redirect(reqwest::redirect::Policy::none())
            .build()?;

        Ok(Self {
            base_url,
            http_client,
            retry_policy: None,
            state: Unauthenticated,
        })
    }

    /// Set a custom timeout for HTTP requests
    pub fn timeout(self, timeout: Duration) -> Result<Self> {
        let http_client = HttpClient::builder()
            .timeout(timeout)
            .redirect(reqwest::redirect::Policy::none())
            .build()?;
        Ok(Self {
            base_url: self.base_url,
            http_client,
            retry_policy: self.retry_policy,
            state: self.state,
        })
    }

    /// Set additional HTTP client configuration.
    ///
    /// The caller is responsible for configuring safe TLS and redirect policies.
    /// The built-in client does not follow redirects so bearer credentials and
    /// request bodies cannot be redirected to another destination.
    pub fn http_client(mut self, client: HttpClient) -> Self {
        self.http_client = client;
        self
    }

    /// Access operations that are intentionally available before authentication.
    pub fn auth(&self) -> PublicAuthEndpoint<'_> {
        PublicAuthEndpoint::new(self)
    }

    pub(crate) fn public_request(
        &self,
        method: reqwest::Method,
        path: &str,
    ) -> Result<reqwest::RequestBuilder> {
        Ok(self.http_client.request(method, self.base_url.join(path)?))
    }

    pub(crate) fn retry_policy(&self) -> Option<&RetryPolicy> {
        self.retry_policy.as_ref()
    }

    /// Configure retry policy for handling rate limits (HTTP 429)
    ///
    /// When a retry policy is set, the client will automatically retry requests
    /// that encounter rate limiting, using exponential backoff and respecting
    /// server-provided Retry-After headers.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use fleetdm_api_client::{FleetClient, RetryPolicy};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// // Conservative retry policy (good for production)
    /// let client = FleetClient::builder("https://fleet.example.com")?
    ///     .with_retry_policy(RetryPolicy::conservative())
    ///     .with_token("token")
    ///     ?
    ///     .build();
    ///
    /// // Aggressive retry policy (good for testing)
    /// let client = FleetClient::builder("https://fleet.example.com")?
    ///     .with_retry_policy(RetryPolicy::aggressive())
    ///     .with_token("token")
    ///     ?
    ///     .build();
    ///
    /// // Custom retry policy
    /// let client = FleetClient::builder("https://fleet.example.com")?
    ///     .with_retry_policy(RetryPolicy {
    ///         max_retries: 3,
    ///         base_delay: std::time::Duration::from_secs(2),
    ///         max_delay: std::time::Duration::from_secs(30),
    ///         respect_retry_after: true,
    ///         use_jitter: true,
    ///     })
    ///     .with_token("token")
    ///     ?
    ///     .build();
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_retry_policy(mut self, policy: RetryPolicy) -> Self {
        self.retry_policy = Some(policy);
        self
    }

    /// Authenticate with email and password, returning an authenticated builder
    pub async fn login(
        self,
        email: impl Into<String>,
        password: impl Into<String>,
    ) -> Result<FleetClientBuilder<Authenticated>> {
        let login_request = LoginRequest {
            email: email.into(),
            password: password.into(),
        };
        if login_request.email.trim().is_empty() || login_request.password.is_empty() {
            return Err(FleetError::Validation(
                "email and password must not be empty".into(),
            ));
        }
        let request = self
            .public_request(reqwest::Method::POST, crate::paths::LOGIN)?
            .json(&login_request);
        let response = crate::http::send_prepared_request(request, self.retry_policy()).await?;
        if response.status() == reqwest::StatusCode::ACCEPTED {
            let response: MfaRequiredResponse = crate::http::handle_response(response).await?;
            return Err(FleetError::MfaRequired(response.message));
        }
        let login_response: LoginResponse = crate::http::handle_response(response).await?;
        validate_token(&login_response.token).map_err(|_| {
            FleetError::Authentication("Fleet returned an invalid authentication token".into())
        })?;

        Ok(FleetClientBuilder {
            base_url: self.base_url,
            http_client: self.http_client,
            retry_policy: self.retry_policy,
            state: Authenticated {
                token: login_response.token,
            },
        })
    }

    /// Authenticate with an existing API token, returning an authenticated builder.
    ///
    /// Empty tokens, surrounding whitespace, and values that cannot safely be
    /// represented in an HTTP header are rejected before a client is constructed.
    pub fn with_token(self, token: impl Into<String>) -> Result<FleetClientBuilder<Authenticated>> {
        let token = token.into();
        validate_token(&token)?;
        Ok(FleetClientBuilder {
            base_url: self.base_url,
            http_client: self.http_client,
            retry_policy: self.retry_policy,
            state: Authenticated { token },
        })
    }
}

fn validate_base_url(base_url: &Url) -> Result<()> {
    if !base_url.username().is_empty() || base_url.password().is_some() {
        return Err(FleetError::Config(
            "Fleet base URL must not contain credentials".into(),
        ));
    }
    if base_url.query().is_some() || base_url.fragment().is_some() {
        return Err(FleetError::Config(
            "Fleet base URL must not contain a query or fragment".into(),
        ));
    }
    if base_url.path() != "/" {
        return Err(FleetError::Config(
            "Fleet base URL must not contain a path".into(),
        ));
    }

    let is_loopback = match base_url.host_str() {
        Some("localhost") => true,
        Some(host) => host
            .trim_matches(['[', ']'])
            .parse::<IpAddr>()
            .is_ok_and(|address| address.is_loopback()),
        None => false,
    };
    if base_url.scheme() != "https" && !(base_url.scheme() == "http" && is_loopback) {
        return Err(FleetError::Config(
            "Fleet base URL must use HTTPS (HTTP is allowed only for loopback testing)".into(),
        ));
    }
    Ok(())
}

fn validate_token(token: &str) -> Result<()> {
    if token.trim().is_empty() {
        return Err(FleetError::Validation(
            "authentication token must not be empty".into(),
        ));
    }
    if token.trim() != token {
        return Err(FleetError::Validation(
            "authentication token must not have leading or trailing whitespace".into(),
        ));
    }
    header::HeaderValue::from_str(&format!("Bearer {token}")).map_err(|error| {
        FleetError::Validation(format!("invalid authentication token: {error}"))
    })?;
    Ok(())
}

impl FleetClientBuilder<Authenticated> {
    /// Build the FleetClient (only available after authentication)
    pub fn build(self) -> FleetClient {
        FleetClient {
            base_url: self.base_url,
            http_client: self.http_client,
            token: self.state.token,
            retry_policy: self.retry_policy,
        }
    }
}

/// Main client for interacting with the FleetDM API
///
/// This client can only be created through the typestate builder pattern,
/// ensuring authentication happens before any API calls.
#[derive(Clone)]
pub struct FleetClient {
    pub(crate) base_url: Url,
    pub(crate) http_client: HttpClient,
    pub(crate) token: String,
    pub(crate) retry_policy: Option<RetryPolicy>,
}

impl FleetClient {
    /// Create a new builder for constructing a FleetClient
    pub fn builder(base_url: impl AsRef<str>) -> Result<FleetClientBuilder<Unauthenticated>> {
        FleetClientBuilder::new(base_url)
    }

    /// Get the hosts endpoint
    pub fn hosts(&self) -> HostsEndpoint {
        HostsEndpoint::new(self.clone())
    }

    /// Get the reports endpoint
    pub fn reports(&self) -> ReportsEndpoint {
        ReportsEndpoint::new(self.clone())
    }

    /// Legacy query routes retained for Fleet versions and callers using the old terminology.
    #[deprecated(note = "use reports() for the current Fleet API")]
    pub fn queries(&self) -> QueriesEndpoint {
        QueriesEndpoint::new(self.clone())
    }

    /// Get the policies endpoint
    pub fn policies(&self) -> PoliciesEndpoint {
        PoliciesEndpoint::new(self.clone())
    }

    /// Get the software endpoint
    pub fn software(&self) -> SoftwareEndpoint {
        SoftwareEndpoint::new(self.clone())
    }

    /// Get the fleets endpoint
    pub fn fleets(&self) -> FleetsEndpoint {
        FleetsEndpoint::new(self.clone())
    }

    /// Legacy team routes retained for Fleet versions and callers using the old terminology.
    #[deprecated(note = "use fleets() for the current Fleet API")]
    pub fn teams(&self) -> TeamsEndpoint {
        TeamsEndpoint::new(self.clone())
    }

    /// Get the users endpoint
    pub fn users(&self) -> UsersEndpoint {
        UsersEndpoint::new(self.clone())
    }

    /// Get the labels endpoint
    pub fn labels(&self) -> LabelsEndpoint {
        LabelsEndpoint::new(self.clone())
    }

    /// Get the scripts endpoint
    pub fn scripts(&self) -> ScriptsEndpoint {
        ScriptsEndpoint::new(self.clone())
    }

    /// Get the activities endpoint
    pub fn activities(&self) -> ActivitiesEndpoint {
        ActivitiesEndpoint::new(self.clone())
    }

    /// Get the version endpoint
    pub fn version(&self) -> VersionEndpoint {
        VersionEndpoint::new(self.clone())
    }

    /// Get the configuration endpoint
    pub fn config(&self) -> ConfigEndpoint<'_> {
        ConfigEndpoint::new(self)
    }

    /// Get the sessions endpoint
    pub fn sessions(&self) -> SessionsEndpoint<'_> {
        SessionsEndpoint::new(self)
    }

    /// Get generated low-level access to documented authenticated Fleet REST routes.
    pub fn raw_api(&self) -> RawApiEndpoint<'_> {
        RawApiEndpoint::new(self)
    }

    /// Backwards-compatible name for the raw API surface.
    #[deprecated(note = "use raw_api()")]
    pub fn spec_api(&self) -> RawApiEndpoint<'_> {
        self.raw_api()
    }

    /// Get the vulnerabilities endpoint
    pub fn vulnerabilities(&self) -> VulnerabilitiesEndpoint<'_> {
        VulnerabilitiesEndpoint::new(self)
    }

    /// Get the targets endpoint
    pub fn targets(&self) -> TargetsEndpoint<'_> {
        TargetsEndpoint::new(self)
    }

    /// Get the translator endpoint
    pub fn translator(&self) -> TranslatorEndpoint<'_> {
        TranslatorEndpoint::new(self)
    }

    /// Get the file carves endpoint
    pub fn carves(&self) -> CarvesEndpoint<'_> {
        CarvesEndpoint::new(self)
    }

    /// Get the certificates endpoint
    pub fn certificates(&self) -> CertificatesEndpoint<'_> {
        CertificatesEndpoint::new(self)
    }

    /// Get the authentication endpoint
    pub fn auth(&self) -> AuthEndpoint<'_> {
        AuthEndpoint::new(self)
    }

    /// Get the MDM commands endpoint
    pub fn commands(&self) -> CommandsEndpoint<'_> {
        CommandsEndpoint::new(self)
    }

    /// Get the OS settings endpoint
    pub fn os_settings(&self) -> OsSettingsEndpoint<'_> {
        OsSettingsEndpoint::new(self)
    }

    /// Get the conditional access endpoint
    pub fn conditional_access(&self) -> ConditionalAccessEndpoint<'_> {
        ConditionalAccessEndpoint::new(self)
    }

    /// Get the setup experience endpoint
    pub fn setup_experience(&self) -> SetupExperienceEndpoint<'_> {
        SetupExperienceEndpoint::new(self)
    }

    /// Get the integrations endpoint
    pub fn integrations(&self) -> IntegrationsEndpoint<'_> {
        IntegrationsEndpoint::new(self)
    }

    /// Get the invitations endpoint
    pub fn invitations(&self) -> InvitationsEndpoint<'_> {
        InvitationsEndpoint::new(self)
    }

    /// Internal method to create an authenticated request
    pub(crate) fn request(
        &self,
        method: reqwest::Method,
        path: &str,
    ) -> Result<reqwest::RequestBuilder> {
        let url = self.base_url.join(path)?;
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bearer {}", self.token))
                .map_err(|e| FleetError::Config(format!("Invalid token: {}", e)))?,
        );

        Ok(self.http_client.request(method, url).headers(headers))
    }

    /// Get the configured retry policy, if any
    pub(crate) fn retry_policy(&self) -> Option<&RetryPolicy> {
        self.retry_policy.as_ref()
    }

    /// Get the authentication token used by this client
    pub fn token(&self) -> &str {
        &self.token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_requires_authentication() {
        // This test demonstrates that you cannot build without authenticating
        // The following would not compile:
        // let client = FleetClient::builder("https://fleet.example.com").unwrap().build();

        // You must call .login() or .with_token() first
        let builder = FleetClient::builder("https://fleet.example.com").unwrap();
        let authenticated = builder.with_token("test-token").unwrap();
        let _client = authenticated.build();
    }

    #[test]
    fn rejects_insecure_remote_urls_and_embedded_credentials() {
        assert!(matches!(
            FleetClient::builder("http://fleet.example.com"),
            Err(FleetError::Config(_))
        ));
        assert!(matches!(
            FleetClient::builder("https://user:password@fleet.example.com"),
            Err(FleetError::Config(_))
        ));
        assert!(matches!(
            FleetClient::builder("https://fleet.example.com/prefix"),
            Err(FleetError::Config(_))
        ));
        assert!(matches!(
            FleetClient::builder("https://fleet.example.com?token=secret"),
            Err(FleetError::Config(_))
        ));
        FleetClient::builder("http://127.0.0.1:8080").unwrap();
        FleetClient::builder("http://[::1]:8080").unwrap();
    }

    #[test]
    fn rejects_empty_or_malformed_tokens() {
        assert!(matches!(
            FleetClient::builder("https://fleet.example.com")
                .unwrap()
                .with_token("  "),
            Err(FleetError::Validation(_))
        ));
        assert!(matches!(
            FleetClient::builder("https://fleet.example.com")
                .unwrap()
                .with_token("token\nforged-header"),
            Err(FleetError::Validation(_))
        ));
    }
}
