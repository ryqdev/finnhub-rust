
pub struct Client {
    pub api_key: String,
}

impl Client {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key
        }
    }

    pub fn ping() -> String{
        "pong".to_string()
    }
}