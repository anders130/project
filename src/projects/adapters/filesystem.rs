use std::path::{Path, PathBuf};

use crate::projects::domain::category::Category;
use crate::projects::domain::project_name::ProjectName;
use crate::projects::domain::repo::Repo;
use crate::projects::ports::project_source::ProjectSource;
use crate::Result;

pub struct FilesystemProjectSource {
    pub projects_dir: PathBuf,
}

impl ProjectSource for FilesystemProjectSource {
    fn find_all(&self) -> Result<Vec<Repo>> {
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
    let Some(dname) = path
        .file_name()
        .and_then(|n| n.to_str())
        .map(str::to_string)
    else {
        return vec![];
    };

    if path.join(".git").is_dir() {
        return vec![Repo {
            name: ProjectName::new(dname),
            category: Category::new(cat),
            path,
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
                name: ProjectName::new(format!("{dname}/{sname}")),
                category: Category::new(cat.clone()),
                path: sp,
            })
        })
        .collect()
}
