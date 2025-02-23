use src2latex::document::Document;

fn main() {
    // Path to the output directory.
    let out_dir_base = &std::path::PathBuf::from("examples/out");
    let out_dir = &out_dir_base.join("tex");

    // Create the output directory if it doesn't exist
    // and put a .gitingore file in the base directory.
    std::fs::create_dir_all(out_dir).unwrap();
    std::fs::write(out_dir_base.join(".gitignore"), "*").unwrap();

    // Write the document examples to the output directory.
    Document::article()
        .write_to_file(out_dir.join("article.tex"))
        .unwrap();
    Document::srcartcl()
        .write_to_file(out_dir.join("srcartcl.tex"))
        .unwrap();
    Document::report()
        .write_to_file(out_dir.join("report.tex"))
        .unwrap();
    Document::srcrprt()
        .write_to_file(out_dir.join("srcrprt.tex"))
        .unwrap();
    Document::book()
        .write_to_file(out_dir.join("book.tex"))
        .unwrap();
    Document::srcbook()
        .write_to_file(out_dir.join("srcbook.tex"))
        .unwrap();
}
