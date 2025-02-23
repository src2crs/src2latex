use super::Document;

/// Basic examples for the `Document` type.
impl Document {
    /// Create a new `Document` with the class `article`.
    pub fn article() -> Self {
        Self::new("article".to_string())
    }

    /// Create a new `Document` with the class `scrartcl`.
    pub fn scrartcl() -> Self {
        Self::new("scrartcl".to_string())
    }

    /// Create a new `Document` with the class `report`.
    pub fn report() -> Self {
        Self::new("report".to_string())
    }

    /// Create a new `Document` with the class `scrrprt`.
    pub fn scrrprt() -> Self {
        Self::new("scrrprt".to_string())
    }

    /// Create a new `Document` with the class `book`.
    pub fn book() -> Self {
        Self::new("book".to_string())
    }

    /// Create a new `Document` with the class `scrbook`.
    pub fn scrbook() -> Self {
        Self::new("scrbook".to_string())
    }
}

/// Examples with packages for the `Document` type.
impl Document {
    /// Create a new `scrartcl` using the `src2listings` package.
    pub fn scrartcl_with_src2listings() -> Self {
        Self::scrartcl()
            .with_package("src2listings")
            .with_option("lang=go")
    }

    /// Create a new `scrartcl` using the `src2report` package.
    pub fn scrartcl_with_src2report() -> Self {
        Self::scrartcl()
            .with_package("src2report")
            .with_option("lang=en")
            .with_option("srclang=go")
    }

    /// Create a new `scrartcl` using the `src2listings` package.
    pub fn scrartcl_with_src2report_and_a4paper() -> Self {
        Self::scrartcl()
            .with_option("a4paper")
            .with_package("src2listings")
            .with_option("lang=go")
    }
}

#[cfg(test)]
mod tests {
    use super::Document;

    macro_rules! test_doc {
        ($name:ident) => {
            #[test]
            fn $name() {
                // Create a temporary directory and file path.
                let tempdir = tempfile::tempdir().expect("Failed to create temporary directory.");
                let tempfile_path = tempdir.path().join(concat!(stringify!($name), ".tex"));

                // Create a new document and write it to the temporary file.
                Document::$name()
                    .write_to_file(&tempfile_path)
                    .expect("Failed to write document to file.");

                // Check that the file contents equal the expected contents.
                // To this end, use the include_str! macro to get the expected contents
                // from the file with the same name as the document.
                assert_eq!(
                    std::fs::read_to_string(&tempfile_path).unwrap(),
                    include_str!(concat!(stringify!($name), ".tex"))
                );
            }
        };
    }

    test_doc!(article);
    test_doc!(scrartcl);
    test_doc!(report);
    test_doc!(scrrprt);
    test_doc!(book);
    test_doc!(scrbook);
    test_doc!(scrartcl_with_src2listings);
    test_doc!(scrartcl_with_src2report);
    test_doc!(scrartcl_with_src2report_and_a4paper);
}
