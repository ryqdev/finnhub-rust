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

    pub fn quote(self, symbol: &str) -> Result<QuoteResponse, reqwest::Error> {
        let url = format!("https://finnhub.io/api/v1/quote?symbol={}&token={}", symbol, self.api_key);
        let response = reqwest::blocking::get(&url)?.json::<QuoteResponse>()?;
        Ok(response)
    }
}

#[derive(Debug, serde::Deserialize)]
struct QuoteResponse {
    current_price: String,
    change: String,
    percent_change: String,
    high_price: String,
    low_price: String,
    open_price: String,
    previous_close_price: String,
}