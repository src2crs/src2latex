/// Represents a package used in a document and provides methods to create
/// LaTeX code for the package.
pub struct Package {
    name: String,
    options: Vec<String>,
}

impl Package {
    /// Create a new `Package` with the given name.
    pub fn new<N: Into<String>>(name: N) -> Self {
        Self {
            name: name.into(),
            options: Vec::new(),
        }
    }

    /// Add an option to the package.
    pub fn add_option<O: Into<String>>(&mut self, option: O) {
        self.options.push(option.into());
    }

    /// Return self's options as a string.
    /// This is a helper method for inserting the options into the package.
    pub fn options(&self) -> String {
        if self.options.is_empty() {
            return String::new();
        }
        format!("[{}]", self.options.join(","))
    }
}

impl std::fmt::Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\\usepackage{}{{{}}}", self.options(), self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::Package;

    #[test]
    fn test_package() {
        let mut pkg = Package::new("listings");
        assert_eq!(format!("{}", pkg), "\\usepackage{listings}");

        pkg.add_option("utf8");
        assert_eq!(format!("{}", pkg), "\\usepackage[utf8]{listings}");

        pkg.add_option("xcolor");
        assert_eq!(format!("{}", pkg), "\\usepackage[utf8,xcolor]{listings}");
    }
}
