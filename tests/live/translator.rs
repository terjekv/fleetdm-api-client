//! Translator endpoint tests

use crate::live_client;
use fleetdm_api_client::FleetClient;
use fleetdm_api_client::Result;
use fleetdm_api_client::models::translator::TranslateRequest;

#[tokio::test]
async fn translate_query() -> Result<()> {
    let client: FleetClient = live_client!();

    let translate_req = TranslateRequest {
        payload: fleetdm_api_client::models::translator::TranslatePayload::Query {
            payload: fleetdm_api_client::models::translator::QueryPayload {
                query: "SELECT * FROM system_info".to_string(),
            },
        },
    };

    let result = client.translator().translate(&translate_req).await;

    // Translator may not be available in all Fleet configurations
    match result {
        Ok(response) => {
            assert!(
                !response.translated_query().is_empty(),
                "translation response should have translated content"
            );
        }
        Err(error) if crate::live::common::is_optional_feature_error(&error) => {
            // Translator endpoint may not be available, which is acceptable
        }
        Err(error) => return Err(error),
    }

    Ok(())
}
