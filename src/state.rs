// src/state.rs this is for application state

#[derive(Clone)]
pub struct AppState {
    pub tmdb_api_key: String,
    pub client: reqwest::Client,
}

