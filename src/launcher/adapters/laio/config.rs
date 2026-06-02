use std::path::Path;

pub struct SessionConfig {
    pub content: String,
}

impl SessionConfig {
    pub fn from_template(template: &str, name: &str, path: &Path) -> Self {
        Self {
            content: template
                .replace("{name}", &yaml_quote(name))
                .replace("{path}", &yaml_quote(&path.display().to_string())),
        }
    }

    pub fn builtin(name: &str, path: &Path) -> Self {
        Self {
            content: format!(
                "\
name: {name}
path: {dir}

windows:
  - name: code
    panes:
      - commands:
          - command: nvim
            args:
              - .
  - name: shell
    focus: true
    panes:
      - commands: []
",
                name = yaml_quote(name),
                dir = yaml_quote(&path.display().to_string()),
            ),
        }
    }
}

fn yaml_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "''"))
}
