#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();

    let mut app = tide::new();

    app.with(ContentTypeMiddleware::new("image/gif"));
    app.at("/*").serve_file("public/fumo.mp4")?;

    app.listen(format!(
        "{}:{}",
        std::env::var("APP_HOST").unwrap_or("127.0.0.1".into()),
        std::env::var("PORT").unwrap_or("8080".into())
    ))
    .await?;
    Ok(())
}

struct ContentTypeMiddleware(String);

impl ContentTypeMiddleware {
    pub fn new<T: Into<String>>(content_type: T) -> Self {
        Self {
            0: content_type.into(),
        }
    }
}

#[tide::utils::async_trait]
impl tide::Middleware<()> for ContentTypeMiddleware {
    async fn handle(&self, request: tide::Request<()>, next: tide::Next<'_, ()>) -> tide::Result {
        let mut response = next.run(request).await;
        response.append_header("Content-Type", &self.0);
        Ok(response)
    }
}
