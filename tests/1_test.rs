#[cfg(test)]
mod tests {
    use std::env::var;

    use simply_plural::*;

    #[tokio::test]
    async fn test() {
        let api = SPApi::new(
            var("SP_API_KEY").expect("No api key specified!"),
            SPEndpoint::Pretesting,
        )
        .expect("Failed to create SPApi struct");

        let user = api.get_user(None).await.unwrap();

        dbg!(user);
    }
}
