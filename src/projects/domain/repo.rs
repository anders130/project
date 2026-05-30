use std::path::PathBuf;

use super::category::Category;
use super::project_name::ProjectName;

#[derive(Debug, Clone)]
pub struct Repo {
    pub name: ProjectName,
    pub path: PathBuf,
    pub category: Category,
}
