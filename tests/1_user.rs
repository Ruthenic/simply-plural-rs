#[cfg(test)]
mod tests {
    use std::env;

    use rand::prelude::*;
    use simply_plural::*;

    #[tokio::test]
    async fn test() {
        let mut rng = rand::thread_rng();

        let api = SPApi::new(
            env::var("SP_API_KEY").expect("No api key specified!"),
            SPEndpoint::Pretesting,
        )
        .expect("Failed to create SPApi struct!");

        let mut user = api.get_user(None).await.unwrap();
        user.set_username(format!("APITest{}", rng.gen_range(0..i32::MAX)))
            .await
            .expect("Failed to change username!");
        user.is_a_system = !user.is_a_system;

        let field = &mut user.get_fields()[0];
        field.1.name = format!("{}_{}", field.0, rng.gen_range(0..i32::MAX));
        user.set_field(field.0.clone(), field.1.clone());

        user.update().await.expect("Failed to update user!")
    }
}
