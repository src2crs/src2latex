use std::path::PathBuf;

pub trait LatexPackage {
    fn src() -> String;
    fn filename() -> &'static str;

    fn write<P: Into<PathBuf>>(dir: P) -> std::io::Result<()> {
        std::fs::write(dir.into().join(Self::filename()), Self::src())
    }
}
