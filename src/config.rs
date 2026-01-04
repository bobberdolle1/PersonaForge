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
}

fn default_ollama_url() -> String {
    "http://ollama:11434".to_string()
}

fn default_ollama_chat_model() -> String {
    "llama3:latest".to_string()
}

fn default_ollama_embedding_model() -> String {
    "nomic-embed-text".to_string()
}

impl Config {
    pub fn from_env() -> Result<Self, envy::Error> {
        envy::from_env::<Config>()
    }
}
