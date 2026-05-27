use std::path::{Path, PathBuf};

use crate::domain::entities::Repo;
use crate::ports::repo_scanner::RepoScanner;
use crate::Result;

pub struct FilesystemRepoScanner {
    pub projects_dir: PathBuf,
}

impl RepoScanner for FilesystemRepoScanner {
    fn scan(&self) -> Result<Vec<Repo>> {
        let Ok(entries) = std::fs::read_dir(&self.projects_dir) else {
            return Ok(vec![]);
        };

        let repos = entries
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.is_dir())
            .flat_map(|dir| {
                let cat = dir
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_string();
                scan_category(&dir, cat)
            })
            .collect();

        Ok(repos)
    }
}

fn scan_category(base: &Path, cat: String) -> Vec<Repo> {
    let Ok(entries) = std::fs::read_dir(base) else {
        return vec![];
    };

    entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .flat_map(move |p| scan_dir(p, cat.clone()))
        .collect()
}

fn scan_dir(path: PathBuf, cat: String) -> Vec<Repo> {
    let Some(dname) = path.file_name().and_then(|n| n.to_str()).map(str::to_string) else {
        return vec![];
    };

    if path.join(".git").is_dir() {
        return vec![Repo {
            session_name: dname.replace('.', "--"),
            display_name: dname,
            path,
            category: cat,
        }];
    }

    let Ok(entries) = std::fs::read_dir(&path) else {
        return vec![];
    };

    entries
        .flatten()
        .map(|e| e.path())
        .filter(|sp| sp.is_dir() && sp.join(".git").is_dir())
        .filter_map(move |sp| {
            let sname = sp.file_name()?.to_str()?.to_string();
            Some(Repo {
                display_name: format!("{dname}/{sname}"),
                session_name: format!("{dname}-{sname}").replace('.', "--"),
                path: sp,
                category: cat.clone(),
            })
        })
        .collect()
}
