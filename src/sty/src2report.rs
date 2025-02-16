use crate::package::macros::package;
use crate::package::LatexPackage;

pub struct Src2Report;

impl LatexPackage for Src2Report {
    fn src() -> String {
        let src = package!("src2report");
        src.to_string()
    }

    fn filename() -> &'static str {
        "src2report.sty"
    }
}
