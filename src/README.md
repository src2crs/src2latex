# Sources for Src2LaTeX

This Rust crate contains code that makes the style files and templates
developed in this repository available to Rust projects.

## Todos / Ideas / Future Work

* Add documentation.
* Add macros for generating the `LatexPackage` trait impl (a derive macro would be nice).
* Add functionality for creating templates and populating them with preamble and body.
  * General LaTeX document creation with preamble and body.
  * Add commands for creating content for the packages from this repo.
    * E.g. preambles, examples/explanations, macro invocations.
* Add functionality for building LaTeX documents.
  * I.e. a very simple replacement for `latexmk`, or a wrapper around `pdflatex`/`lualatex`.
  * Goal: Provide build functionality for tools that use this crate.
    This way, the generated LaTeX files can be kept completely in temporary
    directories and only the final PDFs are produced as artifacts.
* Based on Rust based LaTeX generation and compilation, add a cli tool for generating
  LaTeX document templates for Src2LaTeX packages.
  * Also allow setting options via the cli tool?
* Auto generate examples via Rust or cli.
  * The existing examples in `tests/tex` should be kept (see note below).

## Notes

* Be careful not to add functions that would better be defined in client crates.
* For now, the actual style files should be kept in their `texsrc/sty`
  directory, or at least under `texsrc`. The same goes for the examples.
  This simplifies developing and testing them without a rust compiler.
* A simple LaTeX document model can be developed here for now.
  If this evolves into a general purpos model, it should be moved to a separate crate.
