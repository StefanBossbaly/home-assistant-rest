use homeassistant_rest_rs::Client;

#[tokio::test]
async fn test_good_api_status_async() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = mockito::Server::new();

    let mock_server = server
        .mock("GET", "/api/")
        .match_header("content-type", "application/json")
        .match_header("Authorization", "Bearer test_token")
        .with_body("{\"message\": \"API running.\"}")
        .create_async()
        .await;

    let client = Client::new(server.url().as_str(), "test_token")?;
    let api_status = client.get_api_status().await?;

    assert_eq!(api_status.message, "API running.");

    mock_server.assert_async().await;

    Ok(())
}

#[tokio::test]
async fn test_bad_api_status_async() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = mockito::Server::new();

    let mock_server = server
        .mock("GET", "/api/")
        .match_header("content-type", "application/json")
        .match_header("Authorization", "Bearer test_token")
        .with_body("")
        .create_async()
        .await;

    let client = Client::new(server.url().as_str(), "test_token")?;
    let api_status = client.get_api_status().await;

    assert!(api_status.is_err());

    mock_server.assert_async().await;

    Ok(())
}
