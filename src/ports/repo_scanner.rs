use crate::domain::entities::Repo;
use crate::Result;

pub trait RepoScanner {
    fn scan(&self) -> Result<Vec<Repo>>;
}
