use crate::package::macros::package;
use crate::package::LatexPackage;

pub struct Src2Listing;

impl LatexPackage for Src2Listing {
    fn src() -> String {
        let src = package!("src2listing");
        src.to_string()
    }

    fn filename() -> &'static str {
        "src2listing.sty"
    }
}
