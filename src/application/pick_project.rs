use crate::projects::domain::project::Project;
use crate::ui::picker::port::ProjectPicker;
use crate::Result;

pub fn execute<'a>(
    open: &'a [Project],
    closed: &'a [Project],
    picker: &dyn ProjectPicker,
    query: &str,
) -> Result<Option<&'a Project>> {
    let open_refs: Vec<&Project> = open.iter().collect();
    let closed_refs: Vec<&Project> = closed.iter().collect();
    let all: Vec<&Project> = open.iter().chain(closed.iter()).collect();
    let Some(idx) = picker.pick(&open_refs, &closed_refs, query)? else {
        return Ok(None);
    };
    Ok(all.get(idx).copied())
}
