use crate::projects::domain::repo::Repo;
use crate::Result;

pub trait ProjectSource {
    fn find_all(&self) -> Result<Vec<Repo>>;
}
