use std::path::Path;

/// A LaTeX Document.
pub struct Document {
    class: String,
}

/// Constructors and basic access.
impl Document {
    /// Create a new `Document` with the given class.
    pub fn new(cls: String) -> Self {
        Self { class: cls }
    }
}

/// Methods for exporting the document.
impl Document {
    /// Return the source code of the document.
    pub fn to_string(&self) -> String {
        format!("{}\n\n{}\n", self.preamble(), self.body())
    }

    /// Write the document to a file.
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        std::fs::write(path.as_ref(), self.to_string())
    }
}

/// Methods to access different parts of the document and return them as strings.
impl Document {
    /// Get the `\documentclass` line of the document.
    pub fn class(&self) -> String {
        format!("\\documentclass{{{}}}", self.class)
    }

    /// Get the complete preamble of the document as a string.
    pub fn preamble(&self) -> String {
        format!("{}", self.class())
    }

    /// Get the body of the document as a string.
    pub fn body(&self) -> String {
        "\\begin{document}\n\\end{document}".to_string()
    }
}
