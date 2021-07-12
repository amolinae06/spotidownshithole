use std::path::PathBuf;
use async_std::channel::{Receiver, Sender, bounded};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::fs::File;
use tokio::task;
use serde::{Serialize, Deserialize};

use crate::downloader::{Download, Downloader, DownloaderConfig};
use crate::error::SpotifyError;
use crate::spotify::Spotify;


#[derive(Debug, Clone)]
pub struct Backend {
    tx: Sender<BackendMessage>,
    rx: Receiver<BackendResponse>
}

impl Backend {
    async fn new(settings: Settings, spotify: Spotify) -> Backend {
        let downloader = Downloader::new(settings.downloader.clone(), spotify.clone());
        let (tx, rx_0) = bounded(1);
        let (tx_1, rx) = bounded(1);
        task::spawn(async move {
            worker_thread(settings, spotify, downloader, rx_0, tx_1).await;
        });
        Backend { tx, rx }
    }

    /// Try loading saved settings
    pub async fn try_load() -> Result<Backend, SpotifyError> {
        let settings = Settings::load().await?;
        let spotify = settings.get_spotify().await?;
        Ok(Backend::new(settings, spotify).await)
    }

    /// Login and save settings
    pub async fn login(username: String, password: String, client_id: String, client_secret: String) -> Result<Backend, SpotifyError> {
        let spotify = Spotify::new(&username, &password, &client_id, &client_secret).await?;
        let settings = Settings::new(&username, &password, &client_id, &client_secret).ok_or(SpotifyError::Error("Failed creating settings!".into()))?;
        settings.save().await?;
        Ok(Backend::new(settings, spotify).await)
    }

    /// Add URI to queue
    pub async fn add_uri(self, uri: String) {
        self.tx.send(BackendMessage::AddUri(uri)).await.ok();
    }

    /// Get all downloads
    pub async fn get_downloads(self) -> Vec<Download> {
        self.tx.send(BackendMessage::GetDownloads).await.ok();
        if let BackendResponse::Downloads(d) = self.rx.recv().await.unwrap() {
            return d;
        }
        unreachable!();
    }

    /// Get config
    pub async fn get_config(self) -> DownloaderConfig {
        self.tx.send(BackendMessage::GetConfig).await.ok();
        if let BackendResponse::Config(d) = self.rx.recv().await.unwrap() {
            return d;
        }
        unreachable!();
    }

    pub async fn save_config(self, config: DownloaderConfig) {
        self.tx.send(BackendMessage::SetConfig(config)).await.ok();
    }
}

/// Async backend thread
async fn worker_thread(mut settings: Settings, _spotify: Spotify, mut downloader: Downloader, rx: Receiver<BackendMessage>, tx: Sender<BackendResponse>) {
    while let Ok(msg) = rx.recv().await {
        match msg {
            BackendMessage::AddUri(uri) => {
                downloader.add_uri(&uri).await.ok();
            }
            BackendMessage::GetDownloads => {
                tx.send(BackendResponse::Downloads(downloader.get_downloads().await)).await.ok();
            }
            BackendMessage::GetConfig => {
                tx.send(BackendResponse::Config(settings.downloader.clone())).await.ok();
            }
            BackendMessage::SetConfig(c) => {
                downloader.set_config(c.clone()).await;
                settings.downloader = c;
                settings.save().await.ok();
            }
            
        }
    }
}


enum BackendMessage {
    AddUri(String),
    GetDownloads,
    GetConfig,
    SetConfig(DownloaderConfig)
}

enum BackendResponse {
    Downloads(Vec<Download>),
    Config(DownloaderConfig)
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub username: String,
    pub password: String,
    pub client_id: String,
    pub client_secret: String,
    pub downloader: DownloaderConfig
}

impl Settings {
    /// Create new instance with authentication
    pub fn new(username: &str, password: &str, client_id: &str, client_secret: &str) -> Option<Settings> {
        Some(Settings {
            username: username.to_string(),
            password: password.to_string(),
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            downloader: DownloaderConfig::default()?
        })
    }

    /// Get spotify client
    pub async fn get_spotify(&self) -> Result<Spotify, SpotifyError> {
        Spotify::new(&self.username, &self.password, &self.client_id, &self.client_secret).await
    }

    /// Get config path
    pub fn get_folder() -> Option<PathBuf> {
        Some(dirs::config_dir()?.join("spotidown"))
    }

    /// Get path to settings file
    pub fn get_path() -> Option<PathBuf> {
        Some(Settings::get_folder()?.join("settings.json"))
    }

    /// Save settings to file
    pub async fn save(&self) -> Result<(), SpotifyError> {
        // Create dir
        let path = Settings::get_path().ok_or(SpotifyError::Error("No settings folder!".into()))?;
        tokio::fs::create_dir_all(path.parent().unwrap()).await?;
        let data = serde_json::to_string_pretty(self)?;
        let mut file = File::create(path).await?;
        file.write_all(data.as_bytes()).await?;
        Ok(())
    }

    /// Load settings from file
    pub async fn load() -> Result<Settings, SpotifyError> {
        let path = Settings::get_path().ok_or(SpotifyError::Error("No settings folder!".into()))?;
        let mut file = File::open(path).await?;
        let mut buf = String::new();
        file.read_to_string(&mut buf).await?;
        Ok(serde_json::from_str(&buf)?)
    }
}