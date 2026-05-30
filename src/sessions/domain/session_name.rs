pub fn from_project(display_name: &str) -> String {
    display_name.replace('/', "-").replace('.', "--")
}
