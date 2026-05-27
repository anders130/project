mod adapters;
mod domain;
mod ports;

pub(crate) type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

use std::collections::HashMap;
use std::path::PathBuf;

use adapters::colors::*;
use ports::history_store::HistoryStore;
use ports::picker::{PickEntry, PickItem, Picker};
use ports::repo_scanner::RepoScanner;
use ports::session_launcher::SessionLauncher;
use ports::session_manager::SessionManager;

struct Config {
    history_path: PathBuf,
    laio_config_dir: PathBuf,
    projects_dir: PathBuf,
    template_path: PathBuf,
}

impl Config {
    fn from_env() -> Result<Self> {
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
        Ok(Config {
            history_path: data_home.join("project/history"),
            laio_config_dir: config_home.join("laio"),
            template_path: config_home.join("project/template.yaml"),
            projects_dir,
        })
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("project: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.get(1).map(|s| s.as_str()) == Some("__preview") {
        let session_name = args.get(2).map(|s| s.as_str()).unwrap_or("");
        let path = args.get(3).map(|s| s.as_str()).unwrap_or("");
        adapters::renderer::render_preview(session_name, path, &adapters::tmux::TmuxAdapter);
        return Ok(());
    }

    let mut edit_mode = false;
    let mut query = String::new();
    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "-e" | "--edit" => edit_mode = true,
            _ => query = arg.clone(),
        }
    }

    let cfg = Config::from_env()?;
    let tmux = adapters::tmux::TmuxAdapter;
    let history = adapters::history::FileHistoryStore {
        history_path: cfg.history_path,
    };
    let configs = adapters::laio::LaioAdapter {
        config_dir: cfg.laio_config_dir,
        template_path: cfg.template_path,
    };
    let scanner = adapters::filesystem::FilesystemRepoScanner {
        projects_dir: cfg.projects_dir,
    };
    let preview_cmd = std::env::current_exe()
        .map(|p| format!("{} __preview {{2}} {{3}}", p.display()))
        .ok();
    let picker = adapters::fzf::FzfPicker { preview_cmd };

    let repos = scanner.scan()?;
    let sessions = tmux.list_sessions();
    let frequencies = history.frequencies()?;

    let cat_colors = build_category_colors(&repos);

    let projects = domain::use_cases::build_projects(repos, &sessions, &frequencies);
    let (open, closed) = domain::use_cases::sort_projects(projects);

    let open_items = open.iter().map(|p| PickEntry::Item(PickItem {
        display: format_display(&p.repo, &cat_colors),
        session_name: p.repo.session_name.clone(),
        path: p.repo.path.to_string_lossy().into_owned(),
    }));

    let sep = (!open.is_empty() && !closed.is_empty()).then_some(PickEntry::Separator);

    let closed_items = closed.iter().map(|p| PickEntry::Item(PickItem {
        display: format_display(&p.repo, &cat_colors),
        session_name: p.repo.session_name.clone(),
        path: p.repo.path.to_string_lossy().into_owned(),
    }));

    let items: Vec<PickEntry> = open_items.chain(sep).chain(closed_items).collect();

    let Some(sel) = picker.pick(&items, &query)? else {
        return Ok(());
    };

    history.record(&sel.session_name)?;

    let project_dir = std::path::Path::new(&sel.path);

    if edit_mode {
        configs.ensure_config(&sel.session_name, project_dir)?;
        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".into());
        std::process::Command::new(editor)
            .arg(configs.config_path(&sel.session_name))
            .status()?;
        return Ok(());
    }

    eprintln!("Opening '{}'...", sel.session_name);

    if tmux.has_session(&sel.session_name) {
        return attach_or_switch(&tmux, &sel.session_name);
    }

    configs.ensure_config(&sel.session_name, project_dir)?;
    configs.start_session(&sel.session_name)?;
    attach_or_switch(&tmux, &sel.session_name)
}

fn attach_or_switch(tmux: &dyn ports::session_manager::SessionManager, name: &str) -> Result<()> {
    if tmux.is_in_tmux() {
        tmux.switch_client(name)
    } else {
        tmux.attach_session(name)
    }
}

fn build_category_colors(repos: &[domain::entities::Repo]) -> HashMap<String, &'static str> {
    const PALETTE: &[&str] = &[BLU, GRN, MVE, PCH, YEL, SKY];
    let mut cats: Vec<&str> = repos.iter().map(|r| r.category.as_str()).collect();
    cats.sort_unstable();
    cats.dedup();
    cats.into_iter()
        .enumerate()
        .map(|(i, cat)| (cat.to_string(), PALETTE[i % PALETTE.len()]))
        .collect()
}

fn format_display(
    repo: &domain::entities::Repo,
    cat_colors: &HashMap<String, &'static str>,
) -> String {
    let col = cat_colors.get(&repo.category).copied().unwrap_or(DIM);
    let label = repo.category.chars().next().unwrap_or('?').to_ascii_uppercase();

    match repo.display_name.split_once('/') {
        Some((prefix, suffix)) => format!("{col}{label}{RST}  {DIM}{prefix}/{RST}{suffix}"),
        None => format!("{col}{label}{RST}  {}", repo.display_name),
    }
}
