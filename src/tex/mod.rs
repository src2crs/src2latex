mod document;

pub use document::Document;

mod class;
mod package;

pub(crate) use class::DocumentClass;
pub(crate) use package::Package;
