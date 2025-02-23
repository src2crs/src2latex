use super::Document;

/// Examples for the `Document` type.
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
