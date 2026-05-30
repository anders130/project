pub mod bat;
pub mod glow;

pub trait MarkdownRenderer {
    fn render(&self, path: &str, width: u16);
}
