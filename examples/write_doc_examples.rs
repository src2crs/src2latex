use src2latex::tex::Document;

fn main() {
    // Path to the output directory.
    let out_dir_base = &std::path::PathBuf::from("examples/out");
    let out_dir = &out_dir_base.join("tex");

    // Create the output directory if it doesn't exist
    // and put a .gitingore file in the base directory.
    std::fs::create_dir_all(out_dir).unwrap();
    std::fs::write(out_dir_base.join(".gitignore"), "*").unwrap();

    // Write the document examples to the output directory.
    macro_rules! write_test_docs {
        ($($name:ident),+ $(,)?) => {$(
            Document::$name()
                .write_to_file(out_dir.join(concat!(stringify!($name), ".tex")))
                .unwrap();
        )+};
    }

    write_test_docs!(article, scrartcl, report, scrrprt, book, scrbook);

    write_test_docs!(
        scrartcl_with_src2listings,
        scrartcl_with_src2report,
        scrartcl_with_src2report_and_a4paper
    );
}
