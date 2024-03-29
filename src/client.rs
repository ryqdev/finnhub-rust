pub struct FinnClient {
    pub api_key: String,
}

impl FinnClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key
        }
    }

    pub fn ping() -> String {
        "pong".to_string()
    }

    pub async fn quote(self, symbol: &str) -> Result<QuoteResponse, reqwest::Error> {
        let client = reqwest::Client::new();
        let url = format!("https://finnhub.io/api/v1/quote?symbol={}&token={}", symbol, self.api_key);
        let response = client
            .get(url)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let parsed_response: QuoteResponse = serde_json::from_str(&response).unwrap();
        Ok(parsed_response)
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct QuoteResponse {
    pub c: f64,
    pub d: f64,
    pub dp: f64,
    pub h: f64,
    pub l: f64,
    pub o: f64,
    pub pc: f64,
    pub t: i64,
}