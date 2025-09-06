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

    hc.do_post("/api/tickets", json!({"title": "New Ticket"}))
        .await?
        .print()
        .await?;

    hc.do_get("/api/tickets").await?.print().await?;
    hc.do_delete("/api/tickets/1").await?.print().await?;

    Ok(())
}
