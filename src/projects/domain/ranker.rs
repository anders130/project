use super::project::Project;

pub fn rank(mut open: Vec<Project>, mut closed: Vec<Project>) -> (Vec<Project>, Vec<Project>) {
    open.sort_by(cmp);
    closed.sort_by(cmp);
    (open, closed)
}

fn cmp(a: &Project, b: &Project) -> std::cmp::Ordering {
    b.frequency.0.cmp(&a.frequency.0).then_with(|| {
        a.repo
            .name
            .display()
            .to_lowercase()
            .cmp(&b.repo.name.display().to_lowercase())
    })
}
