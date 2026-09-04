//! Certificate endpoint tests
//! Tests for certificate authorities (CAs) and certificate templates

use crate::live::common::is_optional_feature_error;
use crate::live_client;
use fleetdm_api_client::models::certificate::{
    CreateCertificateAuthorityRequest, CreateCertificateTemplateRequest, CustomEstProxyConfig,
    CustomScepProxyConfig,
};
use fleetdm_api_client::{FleetClient, FleetError, Result};

/// Test listing certificate authorities (CAs)
/// GET /api/v1/fleet/certificate_authorities
#[tokio::test]
async fn list_certificate_authorities() -> Result<()> {
    let client: FleetClient = live_client!();

    // This endpoint may fail if certificates are not configured or requires premium
    let result = client.certificates().list_ca().await;

    match result {
        Ok(cas) => {
            // If it succeeds, validate the structure
            for ca in cas.certificate_authorities() {
                assert!(ca.id() > 0, "each CA should have a valid id");
                assert!(!ca.name().is_empty(), "each CA should have a name");
                assert!(!ca.ca_type().is_empty(), "each CA should have a type");
            }
        }
        Err(error) if is_optional_feature_error(&error) => return Ok(()),
        Err(error) => return Err(error),
    }

    Ok(())
}

/// Test creating and deleting a custom SCEP certificate authority
/// POST /api/v1/fleet/certificate_authorities
/// DELETE /api/v1/fleet/certificate_authorities/:id
#[tokio::test]
async fn create_and_delete_custom_scep_ca() -> Result<()> {
    let client: FleetClient = live_client!();

    let timestamp = chrono::Utc::now().timestamp();
    let ca_name = format!("test_scep_ca_{}", timestamp);

    // Create a custom SCEP CA
    let create_req = CreateCertificateAuthorityRequest {
        digicert: None,
        ndes_scep_proxy: None,
        custom_scep_proxy: Some(CustomScepProxyConfig {
            name: ca_name.clone(),
            url: "https://scep.example.com/scep".to_string(),
            challenge: "test-challenge-password".to_string(),
        }),
        custom_est_proxy: None,
        hydrant: None,
        smallstep: None,
    };

    let result = client.certificates().create_ca(create_req).await;

    let ca = match result {
        Ok(ca) => ca,
        Err(error) if is_optional_feature_error(&error) => return Ok(()),
        Err(error) => return Err(error),
    };

    assert!(ca.id() > 0, "created CA should have a valid id");
    assert_eq!(ca.name(), ca_name, "CA name should match");
    assert_eq!(ca.ca_type(), "custom_scep_proxy", "CA type should match");

    // Delete the CA
    let delete_result = client.certificates().delete_ca(ca.id()).await;
    assert!(delete_result.is_ok(), "should be able to delete CA");

    Ok(())
}

/// Test creating and deleting a custom EST certificate authority
/// POST /api/v1/fleet/certificate_authorities
/// DELETE /api/v1/fleet/certificate_authorities/:id
#[tokio::test]
async fn create_and_delete_custom_est_ca() -> Result<()> {
    let client: FleetClient = live_client!();

    let timestamp = chrono::Utc::now().timestamp();
    let ca_name = format!("test_est_ca_{}", timestamp);

    // Create a custom EST CA
    let create_req = CreateCertificateAuthorityRequest {
        digicert: None,
        ndes_scep_proxy: None,
        custom_scep_proxy: None,
        custom_est_proxy: Some(CustomEstProxyConfig {
            name: ca_name.clone(),
            url: "https://est.example.com/.well-known/est".to_string(),
            username: "test-user".to_string(),
            password: "test-password".to_string(),
        }),
        hydrant: None,
        smallstep: None,
    };

    let result = client.certificates().create_ca(create_req).await;

    let ca = match result {
        Ok(ca) => ca,
        Err(error) if is_optional_feature_error(&error) => return Ok(()),
        Err(error) => return Err(error),
    };

    assert!(ca.id() > 0, "created CA should have a valid id");
    assert_eq!(ca.name(), ca_name, "CA name should match");
    assert_eq!(ca.ca_type(), "custom_est_proxy", "CA type should match");

    // Delete the CA
    client.certificates().delete_ca(ca.id()).await?;

    Ok(())
}

/// Test getting a certificate authority by ID
/// GET /api/v1/fleet/certificate_authorities/:id
#[tokio::test]
async fn get_certificate_authority() -> Result<()> {
    let client: FleetClient = live_client!();

    // First, list CAs to get an ID
    let list_result = client.certificates().list_ca().await;

    let cas = match list_result {
        Ok(cas) => cas,
        Err(error) if is_optional_feature_error(&error) => return Ok(()),
        Err(error) => return Err(error),
    };

    if cas.certificate_authorities().is_empty() {
        // No CAs to test with
        return Ok(());
    }

    let ca_id = cas.certificate_authorities()[0].id();

    // Get the CA by ID
    let result = client.certificates().get_ca(ca_id).await;

    match result {
        Ok(ca) => {
            assert_eq!(ca.id(), ca_id, "CA ID should match");
            assert!(!ca.name().is_empty(), "CA should have a name");
        }
        Err(error) if is_optional_feature_error(&error) => return Ok(()),
        Err(error) => return Err(error),
    }

    Ok(())
}

/// Test listing certificate templates
/// GET /api/v1/fleet/certificates
#[tokio::test]
async fn list_certificate_templates() -> Result<()> {
    let client: FleetClient = live_client!();

    let result = client.certificates().list_certificates().await;

    match result {
        Ok(certs) => {
            // Validate structure
            for cert in certs.certificates() {
                assert!(cert.id() > 0, "each certificate should have a valid id");
                assert!(
                    !cert.name().is_empty(),
                    "each certificate should have a name"
                );
                assert!(
                    cert.certificate_authority_id() > 0,
                    "each certificate should have a CA id"
                );
                assert!(
                    !cert.subject_name().is_empty(),
                    "each certificate should have a subject name"
                );
            }
        }
        Err(error) if is_optional_feature_error(&error) => return Ok(()),
        Err(error) => return Err(error),
    }

    Ok(())
}

/// Test creating and deleting a certificate template
/// POST /api/v1/fleet/certificates
/// DELETE /api/v1/fleet/certificates/:id
#[tokio::test]
async fn create_and_delete_certificate_template() -> Result<()> {
    let client: FleetClient = live_client!();

    // First, we need a CA to create a certificate for
    // Try to list CAs first
    let cas_result = client.certificates().list_ca().await;

    let ca_id = match cas_result {
        Ok(cas) => {
            if cas.certificate_authorities().is_empty() {
                // No CAs available, skip test
                return Ok(());
            }
            // Find a custom SCEP CA if available
            cas.certificate_authorities()
                .iter()
                .find(|ca| ca.ca_type() == "custom_scep_proxy")
                .map(|ca| ca.id())
                .unwrap_or_else(|| cas.certificate_authorities()[0].id())
        }
        Err(error) if is_optional_feature_error(&error) => return Ok(()),
        Err(error) => return Err(error),
    };

    let timestamp = chrono::Utc::now().timestamp();
    let cert_name = format!("test_cert_{}", timestamp);

    // Create a certificate template
    let create_req = CreateCertificateTemplateRequest {
        name: cert_name.clone(),
        team_id: None,
        certificate_authority_id: ca_id,
        subject_name: "/CN=$FLEET_VAR_HOST_END_USER_IDP_USERNAME/OU=$FLEET_VAR_HOST_UUID"
            .to_string(),
    };

    let result = client.certificates().create_certificate(create_req).await;

    let cert = match result {
        Ok(cert) => cert,
        Err(error) if is_optional_feature_error(&error) => return Ok(()),
        Err(error) => return Err(error),
    };

    assert!(cert.id() > 0, "created certificate should have a valid id");
    assert_eq!(cert.name(), cert_name, "certificate name should match");

    // Delete the certificate
    client.certificates().delete_certificate(cert.id()).await?;

    Ok(())
}

/// Test getting a certificate template by ID
/// GET /api/v1/fleet/certificates/:id
#[tokio::test]
async fn get_certificate_template() -> Result<()> {
    let client: FleetClient = live_client!();

    // First, list certificates to get an ID
    let list_result = client.certificates().list_certificates().await;

    let certs = match list_result {
        Ok(certs) => certs,
        Err(error) if is_optional_feature_error(&error) => return Ok(()),
        Err(error) => return Err(error),
    };

    if certs.certificates().is_empty() {
        // No certificates to test with
        return Ok(());
    }

    let cert_id = certs.certificates()[0].id();

    // Get the certificate by ID
    let result = client.certificates().get_certificate(cert_id).await;

    match result {
        Ok(cert) => {
            assert_eq!(cert.id(), cert_id, "certificate ID should match");
            assert!(!cert.name().is_empty(), "certificate should have a name");
        }
        Err(error) if is_optional_feature_error(&error) => return Ok(()),
        Err(error) => return Err(error),
    }

    Ok(())
}

/// Test the full CA lifecycle: create, get, update, delete
#[tokio::test]
async fn full_ca_lifecycle() -> Result<()> {
    let client: FleetClient = live_client!();

    let timestamp = chrono::Utc::now().timestamp();
    let ca_name = format!("test_lifecycle_ca_{}", timestamp);

    // 1. Create CA
    let create_req = CreateCertificateAuthorityRequest {
        digicert: None,
        ndes_scep_proxy: None,
        custom_scep_proxy: Some(CustomScepProxyConfig {
            name: ca_name.clone(),
            url: "https://scep.example.com/scep".to_string(),
            challenge: "initial-challenge".to_string(),
        }),
        custom_est_proxy: None,
        hydrant: None,
        smallstep: None,
    };

    let create_result = client.certificates().create_ca(create_req).await;

    let ca = match create_result {
        Ok(ca) => ca,
        Err(error) if is_optional_feature_error(&error) => return Ok(()),
        Err(error) => return Err(error),
    };

    let ca_id = ca.id();

    // 2. Get CA
    let get_result = client.certificates().get_ca(ca_id).await;
    match get_result {
        Ok(fetched_ca) => {
            assert_eq!(fetched_ca.id(), ca_id, "fetched CA ID should match");
        }
        Err(error) if is_optional_feature_error(&error) => {}
        Err(error) => return Err(error),
    }

    // 3. Update CA (experimental feature, may fail)
    // Skipping update as it's experimental and may not work

    // 4. Delete CA
    client.certificates().delete_ca(ca_id).await?;

    // 5. Verify deletion (get should fail)
    let verify_delete = client.certificates().get_ca(ca_id).await;
    assert!(
        matches!(verify_delete, Err(FleetError::NotFound(_))),
        "deleted CA should return NotFound"
    );

    Ok(())
}

/// Test error handling for non-existent CA
#[tokio::test]
async fn get_nonexistent_ca() -> Result<()> {
    let client: FleetClient = live_client!();

    // Try to get a CA with a very high ID that likely doesn't exist
    let result = client.certificates().get_ca(999999).await;

    match result {
        Err(
            FleetError::NotFound(_)
            | FleetError::BadRequest(_)
            | FleetError::PremiumRequired { .. },
        ) => {}
        Err(error) => return Err(error),
        Ok(_) => panic!("getting a non-existent CA unexpectedly succeeded"),
    }

    Ok(())
}

/// Test error handling for non-existent certificate
#[tokio::test]
async fn get_nonexistent_certificate() -> Result<()> {
    let client: FleetClient = live_client!();

    // Try to get a certificate with a very high ID that likely doesn't exist
    let result = client.certificates().get_certificate(999999).await;

    assert!(
        matches!(result, Err(FleetError::NotFound(_))),
        "getting a non-existent certificate should return NotFound"
    );

    Ok(())
}
