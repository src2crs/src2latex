/// Represents a document class and provides methods to create
/// LaTeX code for the class.
pub struct DocumentClass {
    name: String,
    options: Vec<String>,
}

impl DocumentClass {
    /// Create a new `Class` with the given name.
    pub fn new<N: Into<String>>(name: N) -> Self {
        Self {
            name: name.into(),
            options: Vec::new(),
        }
    }

    /// Add an option to the class.
    pub fn add_option<O: Into<String>>(&mut self, option: O) {
        self.options.push(option.into());
    }

    /// Return self's options as a string.
    /// This is a helper method for inserting the options into the document.
    pub fn options(&self) -> String {
        if self.options.is_empty() {
            return String::new();
        }
        format!("[{}]", self.options.join(","))
    }
}

impl std::fmt::Display for DocumentClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\\documentclass{}{{{}}}", self.options(), self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::DocumentClass;

    #[test]
    fn test_document_class() {
        let mut cls = DocumentClass::new("article");
        assert_eq!(format!("{}", cls), "\\documentclass{article}");

        cls.add_option("12pt");
        assert_eq!(format!("{}", cls), "\\documentclass[12pt]{article}");

        cls.add_option("a4paper");
        assert_eq!(format!("{}", cls), "\\documentclass[12pt,a4paper]{article}");
    }
}
