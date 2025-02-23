use super::Document;

/// Basic examples for the `Document` type.
impl Document {
    /// Create a new `Document` with the class `article`.
    pub fn article() -> Self {
        Self::new("article".to_string())
    }

    /// Create a new `Document` with the class `srcartcl`.
    pub fn srcartcl() -> Self {
        Self::new("article".to_string())
    }

    /// Create a new `Document` with the class `report`.
    pub fn report() -> Self {
        Self::new("report".to_string())
    }

    /// Create a new `Document` with the class `srcrprt`.
    pub fn srcrprt() -> Self {
        Self::new("report".to_string())
    }

    /// Create a new `Document` with the class `book`.
    pub fn book() -> Self {
        Self::new("book".to_string())
    }

    /// Create a new `Document` with the class `srcbook`.
    pub fn srcbook() -> Self {
        Self::new("book".to_string())
    }
}

/// Examples with packages for the `Document` type.
impl Document {
    /// Create a new `srcartcl` using the `src2listings` package.
    pub fn srcartcl_with_src2listings() -> Self {
        Self::srcartcl().with_package("src2listings")
    }

    /// Create a new `srcartcl` using the `src2report` package.
    pub fn srcartcl_with_src2report() -> Self {
        Self::srcartcl().with_package("src2report")
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
    test_doc!(srcartcl);
    test_doc!(report);
    test_doc!(srcrprt);
    test_doc!(book);
    test_doc!(srcbook);
    test_doc!(srcartcl_with_src2listings);
    test_doc!(srcartcl_with_src2report);
}
