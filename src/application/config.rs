use std::path::PathBuf;

use crate::Result;

pub enum MarkdownBackend {
    Glow,
    Bat,
}

pub enum LauncherBackend {
    Laio,
}

pub enum MultiplexerBackend {
    Tmux,
}

pub enum PickerBackend {
    Fzf,
    Tv,
}

pub struct Config {
    pub history_path: PathBuf,
    pub laio_config_dir: PathBuf,
    pub projects_dir: PathBuf,
    pub markdown_backend: MarkdownBackend,
    pub launcher_backend: LauncherBackend,
    pub multiplexer_backend: MultiplexerBackend,
    pub picker_backend: PickerBackend,
    pub glow_style: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let home = PathBuf::from(std::env::var("HOME")?);
        let data_home = std::env::var("XDG_DATA_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| home.join(".local/share"));
        let config_home = std::env::var("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| home.join(".config"));
        let projects_dir = std::env::var("PROJECT_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| home.join("Projects"));

        let markdown_backend = match std::env::var("PROJECT_MARKDOWN_RENDERER")
            .as_deref()
            .unwrap_or("glow")
        {
            "bat" => MarkdownBackend::Bat,
            _ => MarkdownBackend::Glow,
        };

        let launcher_backend = LauncherBackend::Laio;

        let multiplexer_backend = MultiplexerBackend::Tmux;

        let picker_backend = match std::env::var("PROJECT_PICKER").as_deref().unwrap_or("fzf") {
            "tv" => PickerBackend::Tv,
            _ => PickerBackend::Fzf,
        };

        let glow_style = std::env::var("PROJECT_GLOW_STYLE").unwrap_or_else(|_| "auto".into());

        Ok(Config {
            history_path: data_home.join("project/history"),
            laio_config_dir: config_home.join("laio"),
            projects_dir,
            markdown_backend,
            launcher_backend,
            multiplexer_backend,
            picker_backend,
            glow_style,
        })
    }
}
