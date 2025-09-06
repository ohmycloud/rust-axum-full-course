#[tokio::test]
async fn quick_dev() -> anyhow::Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello?name=Jen").await?.print().await?;
    hc.do_get("/greet/rakudo_star").await?.print().await?;

    Ok(())
}
