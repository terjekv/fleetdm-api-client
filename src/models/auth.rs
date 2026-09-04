use serde::{Deserialize, Serialize};

// Login (already handled in client.rs but adding models here for completeness)
#[derive(Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginResponse {
    pub token: String,
    #[serde(default)]
    pub user: Option<User>,
}

// Logout
#[derive(Debug, Deserialize)]
pub struct LogoutResponse {}

// Forgot password
#[derive(Debug, Serialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct ForgotPasswordResponse {}

// Change password
#[derive(Serialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Debug, Deserialize)]
pub struct ChangePasswordResponse {
    #[serde(default)]
    pub user: Option<User>,
}

// Reset password
#[derive(Serialize)]
pub struct ResetPasswordRequest {
    pub new_password: String,
    pub new_password_confirmation: String,
    pub password_reset_token: String,
}

#[derive(Debug, Deserialize)]
pub struct ResetPasswordResponse {}

// Perform required password reset
#[derive(Serialize)]
pub struct PerformRequiredPasswordResetRequest {
    pub new_password: String,
}

#[derive(Debug, Deserialize)]
pub struct PerformRequiredPasswordResetResponse {
    #[serde(default)]
    pub user: Option<User>,
}

// Me - Get current user
#[derive(Debug, Deserialize)]
pub struct MeResponse {
    pub user: User,
    #[serde(default)]
    pub available_teams: Vec<AvailableTeam>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub email: String,
    pub name: String,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    pub force_password_reset: bool,
    pub gravatar_url: Option<String>,
    #[serde(default)]
    pub gravatar_url_dark: Option<String>,
    pub sso_enabled: bool,
    #[serde(default)]
    pub mfa_enabled: bool,
    pub global_role: Option<String>,
    #[serde(default)]
    pub teams: Vec<UserTeam>,
    #[serde(default)]
    pub api_only: bool,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

fn default_enabled() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTeam {
    pub id: u64,
    pub name: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailableTeam {
    pub id: u64,
    pub name: String,
}

// SSO Config
#[derive(Debug, Deserialize)]
pub struct SSOConfigResponse {
    pub settings: Option<SSOSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSOSettings {
    pub idp_name: String,
    pub idp_image_url: Option<String>,
    pub sso_enabled: bool,
}

// Initiate SSO
#[derive(Debug, Serialize)]
pub struct InitiateSSORequest {
    pub relay_url: String,
}

#[derive(Deserialize)]
pub struct InitiateSSOResponse {
    pub url: String,
    #[serde(skip)]
    session_cookie: Option<String>,
}

// SSO Callback
#[derive(Serialize)]
pub struct SSOCallbackRequest {
    #[serde(rename = "SAMLResponse")]
    pub saml_response: String,
}

#[derive(Deserialize)]
pub struct SSOCallbackResponse {
    pub token: String,
}

impl InitiateSSOResponse {
    pub(crate) fn set_session_cookie(&mut self, session_cookie: Option<String>) {
        self.session_cookie = session_cookie;
    }

    /// Cookie that must be sent to the callback for service-provider initiated SSO.
    pub fn session_cookie(&self) -> Option<&str> {
        self.session_cookie.as_deref()
    }
}

impl User {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn force_password_reset(&self) -> bool {
        self.force_password_reset
    }

    pub fn gravatar_url(&self) -> Option<&str> {
        self.gravatar_url.as_deref()
    }

    pub fn gravatar_url_dark(&self) -> Option<&str> {
        self.gravatar_url_dark.as_deref()
    }

    pub fn sso_enabled(&self) -> bool {
        self.sso_enabled
    }

    pub fn global_role(&self) -> Option<&str> {
        self.global_role.as_deref()
    }

    pub fn teams(&self) -> &[UserTeam] {
        &self.teams
    }

    pub fn api_only(&self) -> bool {
        self.api_only
    }

    pub fn mfa_enabled(&self) -> bool {
        self.mfa_enabled
    }

    pub fn created_at(&self) -> Option<&str> {
        self.created_at.as_deref()
    }

    pub fn updated_at(&self) -> Option<&str> {
        self.updated_at.as_deref()
    }
}

impl MeResponse {
    pub fn user(&self) -> &User {
        &self.user
    }

    pub fn available_teams(&self) -> &[AvailableTeam] {
        &self.available_teams
    }
}
