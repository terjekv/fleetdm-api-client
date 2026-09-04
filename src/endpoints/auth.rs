use crate::client::{FleetClient, FleetClientBuilder, Unauthenticated};
use crate::error::{FleetError, Result};
use crate::models::auth::*;
use crate::paths;

const SSO_SESSION_COOKIE: &str = "__Host-FLEETSSOSESSIONID=";

/// Authentication operations that must be available before a token exists.
pub struct PublicAuthEndpoint<'a> {
    builder: &'a FleetClientBuilder<Unauthenticated>,
}

impl<'a> PublicAuthEndpoint<'a> {
    pub(crate) fn new(builder: &'a FleetClientBuilder<Unauthenticated>) -> Self {
        Self { builder }
    }

    /// Send a password-reset email.
    pub async fn forgot_password(
        &self,
        email: impl Into<String>,
    ) -> Result<ForgotPasswordResponse> {
        let email = email.into();
        if email.trim().is_empty() {
            return Err(FleetError::Validation("email must not be empty".into()));
        }
        let body = ForgotPasswordRequest { email };
        let request = self
            .builder
            .public_request(reqwest::Method::POST, paths::FORGOT_PASSWORD)?
            .json(&body);
        crate::http::send_request(request, self.builder.retry_policy()).await
    }

    /// Reset a password using a password-reset token.
    pub async fn reset_password(
        &self,
        new_password: impl Into<String>,
        new_password_confirmation: impl Into<String>,
        password_reset_token: impl Into<String>,
    ) -> Result<ResetPasswordResponse> {
        let new_password = new_password.into();
        let new_password_confirmation = new_password_confirmation.into();
        let password_reset_token = password_reset_token.into();
        if new_password.is_empty() {
            return Err(FleetError::Validation(
                "new password must not be empty".into(),
            ));
        }
        if new_password != new_password_confirmation {
            return Err(FleetError::Validation(
                "new password and confirmation must match".into(),
            ));
        }
        if password_reset_token.trim().is_empty() {
            return Err(FleetError::Validation(
                "password reset token must not be empty".into(),
            ));
        }
        let body = ResetPasswordRequest {
            new_password,
            new_password_confirmation,
            password_reset_token,
        };
        let request = self
            .builder
            .public_request(reqwest::Method::POST, paths::RESET_PASSWORD)?
            .json(&body);
        crate::http::send_request(request, self.builder.retry_policy()).await
    }

    /// Get the public SSO configuration.
    pub async fn sso_config(&self) -> Result<SSOConfigResponse> {
        let request = self
            .builder
            .public_request(reqwest::Method::GET, paths::SSO)?;
        crate::http::send_request(request, self.builder.retry_policy()).await
    }

    /// Begin service-provider initiated SSO and retain Fleet's session cookie.
    pub async fn initiate_sso(&self, relay_url: impl Into<String>) -> Result<InitiateSSOResponse> {
        let relay_url = relay_url.into();
        if !relay_url.starts_with('/') || relay_url.starts_with("//") {
            return Err(FleetError::Validation(
                "relay_url must be a relative path beginning with '/'".into(),
            ));
        }
        let body = InitiateSSORequest { relay_url };
        let request = self
            .builder
            .public_request(reqwest::Method::POST, paths::SSO)?
            .json(&body);
        let response =
            crate::http::send_prepared_request(request, self.builder.retry_policy()).await?;
        let session_cookie = response
            .headers()
            .get_all(reqwest::header::SET_COOKIE)
            .iter()
            .filter_map(|value| value.to_str().ok())
            .find_map(extract_sso_cookie);
        let mut body: InitiateSSOResponse = crate::http::handle_response(response).await?;
        body.set_session_cookie(session_cookie);
        Ok(body)
    }

    /// Complete SSO. Pass the cookie returned by [`InitiateSSOResponse::session_cookie`]
    /// for service-provider initiated login, or `None` for IdP-initiated login.
    pub async fn sso_callback(
        &self,
        saml_response: impl Into<String>,
        session_cookie: Option<&str>,
    ) -> Result<SSOCallbackResponse> {
        let saml_response = saml_response.into();
        if saml_response.trim().is_empty() {
            return Err(FleetError::Validation(
                "SAML response must not be empty".into(),
            ));
        }
        let body = SSOCallbackRequest { saml_response };
        let mut request = self
            .builder
            .public_request(reqwest::Method::POST, paths::SSO_CALLBACK)?
            .json(&body);
        if let Some(cookie) = session_cookie {
            let cookie_value = cookie.strip_prefix(SSO_SESSION_COOKIE);
            if cookie_value.is_none_or(str::is_empty) || cookie.contains(';') {
                return Err(FleetError::Validation(
                    "invalid Fleet SSO session cookie".into(),
                ));
            }
            let header = reqwest::header::HeaderValue::from_str(cookie)
                .map_err(|_| FleetError::Validation("invalid Fleet SSO session cookie".into()))?;
            request = request.header(reqwest::header::COOKIE, header);
        }
        crate::http::send_request(request, self.builder.retry_policy()).await
    }
}

fn extract_sso_cookie(set_cookie: &str) -> Option<String> {
    set_cookie
        .split(';')
        .next()
        .filter(|cookie| {
            cookie
                .strip_prefix(SSO_SESSION_COOKIE)
                .is_some_and(|value| !value.is_empty())
        })
        .map(str::to_owned)
}

/// Authentication operations that require an authenticated client.
pub struct AuthEndpoint<'a> {
    client: &'a FleetClient,
}

impl<'a> AuthEndpoint<'a> {
    pub(crate) fn new(client: &'a FleetClient) -> Self {
        Self { client }
    }

    /// Log out the current user.
    pub async fn logout(&self) -> Result<LogoutResponse> {
        let request = self.client.request(reqwest::Method::POST, paths::LOGOUT)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Change the current user's password.
    pub async fn change_password(
        &self,
        old_password: impl Into<String>,
        new_password: impl Into<String>,
    ) -> Result<ChangePasswordResponse> {
        let old_password = old_password.into();
        let new_password = new_password.into();
        if old_password.is_empty() || new_password.is_empty() {
            return Err(FleetError::Validation(
                "old and new passwords must not be empty".into(),
            ));
        }
        let body = ChangePasswordRequest {
            old_password,
            new_password,
        };
        let request = self
            .client
            .request(reqwest::Method::POST, paths::CHANGE_PASSWORD)?
            .json(&body);
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Perform a password reset required by Fleet after login.
    pub async fn perform_required_password_reset(
        &self,
        new_password: impl Into<String>,
    ) -> Result<PerformRequiredPasswordResetResponse> {
        let new_password = new_password.into();
        if new_password.is_empty() {
            return Err(FleetError::Validation(
                "new password must not be empty".into(),
            ));
        }
        let body = PerformRequiredPasswordResetRequest { new_password };
        let request = self
            .client
            .request(
                reqwest::Method::POST,
                paths::PERFORM_REQUIRED_PASSWORD_RESET,
            )?
            .json(&body);
        crate::http::send_request(request, self.client.retry_policy()).await
    }

    /// Get information about the current authenticated user.
    pub async fn me(&self) -> Result<MeResponse> {
        let request = self.client.request(reqwest::Method::GET, paths::ME)?;
        crate::http::send_request(request, self.client.retry_policy()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_only_the_fleet_sso_cookie_pair() {
        assert_eq!(
            extract_sso_cookie(
                "__Host-FLEETSSOSESSIONID=abc+/=; Path=/; Max-Age=300; HttpOnly; Secure"
            ),
            Some("__Host-FLEETSSOSESSIONID=abc+/=".to_string())
        );
        assert_eq!(extract_sso_cookie("other=value; Path=/"), None);
        assert_eq!(
            extract_sso_cookie("__Host-FLEETSSOSESSIONID=; Path=/"),
            None
        );
    }
}
