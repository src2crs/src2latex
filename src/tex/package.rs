/// Represents a package used in a document and provides methods to create
/// LaTeX code for the package.
pub struct Package {
    name: String,
}

impl Package {
    /// Create a new `Package` with the given name.
    pub fn new<N: Into<String>>(name: N) -> Self {
        Self { name: name.into() }
    }
}

impl std::fmt::Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\\usepackage{{{}}}", self.name)
    }
}
