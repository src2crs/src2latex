use src2latex::{
    package::LatexPackage,
    sty::{Src2Listing, Src2Report},
};

fn main() {
    // Path to the output directory.
    let out_dir = &std::path::PathBuf::from("examples/out");

    // Create the output directory if it doesn't exist
    // and put a .gitingore file in it.
    std::fs::create_dir_all(out_dir).unwrap();
    std::fs::write(out_dir.join(".gitignore"), "*").unwrap();

    // Write the package sources to the output directory.
    Src2Listing::write(out_dir).unwrap();
    Src2Report::write(out_dir).unwrap();
}
