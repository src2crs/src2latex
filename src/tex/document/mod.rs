mod examples;

use itertools::Itertools;
use std::path::Path;

use super::{DocumentClass, Package};
/// A LaTeX Document.
pub struct Document {
    class: DocumentClass,
    packages: Vec<Package>,
}

/// Constructors and modifiers.
impl Document {
    /// Create a new `Document` with the given class.
    pub fn new<C: Into<String>>(cls: C) -> Self {
        Self {
            class: DocumentClass::new(cls),
            packages: Vec::new(),
        }
    }

    pub fn with_package<P: Into<String>>(mut self, package: P) -> Self {
        self.packages.push(Package::new(package));
        self
    }
}

/// Methods for exporting the document.
impl Document {
    /// Return the source code of the document.
    pub fn to_string(&self) -> String {
        [self.preamble(), self.body()].join("\n\n")
    }

    /// Write the document to a file.
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        std::fs::write(path.as_ref(), format!("{}\n", self.to_string()))
    }
}

/// Methods to access different parts of the document and return them as strings.
impl Document {
    /// Get the `\documentclass` line of the document.
    pub fn class(&self) -> String {
        format!("{}", self.class)
    }

    /// Get the `\usepackage` lines of the document as a string.
    pub fn packages(&self) -> String {
        self.packages.iter().map(|p| format!("{}", p)).join("\n")
    }

    /// Get the complete preamble of the document as a string.
    pub fn preamble(&self) -> String {
        if self.packages.is_empty() {
            return self.class();
        }

        [self.class(), self.packages()].join("\n\n")
    }

    /// Get the body of the document as a string.
    pub fn body(&self) -> String {
        ["\\begin{document}", "\\end{document}"].join("\n")
    }
}
