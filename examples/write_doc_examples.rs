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
    macro_rules! write_test_doc {
        ($name:ident) => {
            Document::$name()
                .write_to_file(out_dir.join(concat!(stringify!($name), ".tex")))
                .unwrap();
        };
    }

    write_test_doc!(article);
    write_test_doc!(scrartcl);
    write_test_doc!(report);
    write_test_doc!(scrrprt);
    write_test_doc!(book);
    write_test_doc!(scrbook);

    write_test_doc!(scrartcl_with_src2listings);
    write_test_doc!(scrartcl_with_src2report);
    write_test_doc!(scrartcl_with_src2report_and_a4paper);
}
