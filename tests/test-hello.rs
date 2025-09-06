use serde_json::json;

#[tokio::test]
async fn quick_dev() -> anyhow::Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello?name=Jen").await?.print().await?;
    hc.do_get("/greet/rakudo_star").await?.print().await?;
    hc.do_post(
        "/api/login",
        json!({"username": "admin", "password": "admin"}),
    )
    .await?
    .print()
    .await?;

    Ok(())
}
