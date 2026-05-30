use super::repo::Repo;

#[derive(Clone)]
pub struct UsageFrequency(pub usize);

#[derive(Clone)]
pub struct Project {
    pub repo: Repo,
    pub frequency: UsageFrequency,
}
