mod application;
mod history;
mod launcher;
mod projects;
mod sessions;
mod ui;

pub(crate) type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

use application::config::{
    Config, LauncherBackend, MarkdownBackend, MultiplexerBackend, PickerBackend,
};
use history::adapters::file_store::FileUsageStore;
use launcher::adapters::laio::LaioSessionStarter;
use launcher::ports::launcher::Launcher;
use projects::adapters::filesystem::FilesystemProjectSource;
use sessions::adapters::tmux::TmuxAdapter;
use sessions::ports::multiplexer::Multiplexer;
use ui::picker::fzf::FzfProjectPicker;
use ui::picker::port::ProjectPicker as ProjectPickerTrait;
use ui::picker::tv::TvProjectPicker;
use ui::preview::markdown::bat::BatRenderer;
use ui::preview::markdown::glow::GlowRenderer;
use ui::preview::markdown::MarkdownRenderer;

fn main() {
    if let Err(e) = run() {
        eprintln!("project: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let cfg = Config::from_env()?;

    let multiplexer: Box<dyn Multiplexer> = match cfg.multiplexer_backend {
        MultiplexerBackend::Tmux => Box::new(TmuxAdapter),
    };

    if args.get(1).map(|s| s.as_str()) == Some("__preview") {
        let renderer: Box<dyn MarkdownRenderer> = match cfg.markdown_backend {
            MarkdownBackend::Glow => Box::new(GlowRenderer {
                style: cfg.glow_style,
            }),
            MarkdownBackend::Bat => Box::new(BatRenderer),
        };
        return ui::preview::subprocess::handle(
            &args[2..],
            multiplexer.as_ref(),
            renderer.as_ref(),
        );
    }

    let (edit_mode, query) = application::cli::parse_args(args.iter().skip(1));

    let source = FilesystemProjectSource {
        projects_dir: cfg.projects_dir,
    };
    let usage = FileUsageStore {
        history_path: cfg.history_path,
    };
    let starter: Box<dyn Launcher> = match cfg.launcher_backend {
        LauncherBackend::Laio => Box::new(LaioSessionStarter {
            config_dir: cfg.laio_config_dir,
        }),
    };
    let preview_cmd = std::env::current_exe()
        .ok()
        .map(|p| format!("{} __preview", p.display()));
    let picker: Box<dyn ProjectPickerTrait> = match cfg.picker_backend {
        PickerBackend::Fzf => Box::new(FzfProjectPicker { preview_cmd }),
        PickerBackend::Tv => Box::new(TvProjectPicker { preview_cmd }),
    };

    let (open, closed) =
        application::load_projects::execute(&source, multiplexer.as_ref(), &usage)?;

    let Some(project) =
        application::pick_project::execute(&open, &closed, picker.as_ref(), &query)?
    else {
        return Ok(());
    };

    if edit_mode {
        return application::edit_config::execute(project, starter.as_ref());
    }

    application::open_project::execute(
        project,
        &usage,
        multiplexer.as_ref(),
        starter.as_ref(),
        multiplexer.as_ref(),
    )
}
