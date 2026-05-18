use cert_service_practice::app::Application;

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        dotenvy::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let app = Application::build(&database_url, "0.0.0.0:0")
            .await
            .expect("Failed to build application");

        let address = app.run_http_random_port().await;
        let http_client = reqwest::Client::new();

        TestApp {
            address,
            http_client,
        }
    }

    pub async fn get(&self, path: &str) -> reqwest::Response {
        self.http_client
            .get(format!("{}{}", self.address, path))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post(&self, path: &str, body: serde_json::Value) -> reqwest::Response {
        self.http_client
            .post(format!("{}{}", self.address, path))
            .json(&body)
            .send()
            .await
            .expect("Failed to execute request")
    }
}
