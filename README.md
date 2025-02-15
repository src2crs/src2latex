# Src2LaTeX

A collection of LaTeX resources that are meant to be used
in conjunction with other Src2Crs projects.

Examples for resources in this projects are:

* LaTeX document templates, e.g. for exams,
  exam reports, slides/lecture notes, ...
* Style files for use in these templates or in related documents.
* Rust libraries that allow generating prepopulated LaTeX
  documents using the templates and style files.

## Scope

The purpose of the resources in this project is to
have a common repository for development and testing.

The documents/templates developed here
are mainly meant for internal use by other Src2Crs project.
They are exported via the Rust crate defined here
and other projects (e.g. Src2Crs, which is written in Rust)
use this crate to access the documents.

Other ways of publishing the files, like similar mechanisms
for other languages or even directly to CTAN, may be considered
later, but are out of scope for the moment.

## Content

For a more detailed content overview, see the [README.md](texsrc/sty/README.md) in the `texsrc` directory.
