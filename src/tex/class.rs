/// Represents a document class and provides methods to create
/// LaTeX code for the class.
pub struct DocumentClass {
    name: String,
}

impl DocumentClass {
    /// Create a new `Class` with the given name.
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl std::fmt::Display for DocumentClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\\documentclass{{{}}}", self.name)
    }
}
