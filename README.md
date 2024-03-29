# Finnhub Rust Tool Kit

## Available Api
- [quote](https://finnhub.io/docs/api/quote)

## Example
Get your api key from [finnhub](https://finnhub.io/)

```rust
use finnhub_rust::client::FinnClient;
#[tokio::main]
async fn main() {
    let api_key = "<your api key from finnhub>".to_string();
    let client = FinnClient::new(api_key);
    let resp = client.quote("AAPL").await.unwrap();
    println!("{:?}", resp)
}
```