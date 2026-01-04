use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Config {
    #[serde(default = "default_ollama_url")]
    pub ollama_url: String,
    pub teloxide_token: String,
    pub database_url: String,
    pub owner_id: u64,
    #[serde(default = "default_ollama_chat_model")]
    pub ollama_chat_model: String,
    #[serde(default = "default_ollama_embedding_model")]
    pub ollama_embedding_model: String,
    #[serde(default = "default_temperature")]
    pub temperature: f64,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
}

fn default_ollama_url() -> String {
    "http://host.docker.internal:11434".to_string()
}

fn default_ollama_chat_model() -> String {
    "gemini-3-flash-preview:cloud".to_string()
}

fn default_ollama_embedding_model() -> String {
    "nomic-embed-text".to_string()
}

fn default_temperature() -> f64 {
    0.7
}

fn default_max_tokens() -> u32 {
    2048
}

impl Config {
    pub fn from_env() -> Result<Self, envy::Error> {
        envy::from_env::<Config>()
    }
}
