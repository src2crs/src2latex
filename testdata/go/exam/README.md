# Example: Go Exam

This is a fictitious example exam using the Go programming language.
It contains a subdirectory `submissions` with examples of how the
files submitted by students might look like after they have been marked.
These files are used as examples and tests for the `src2report` package.

## File Overview

* There are three subdirectories in the `submissions` directory
for three fictitious students: `alice`, `bob`, and `charlie`.
* Each of these contains three subdirectories for different tasks.
* Each task directory contains a `.go` with the student's solution.
* The files named after the tasks (e.g., `task1.go`).
* The files contain comments as they may have been added when marking the exam.

## Ideas / Usage

This is a very simple example of how a marked exam might look like.
An actual exam would contain more files, like e.g. tests,
the original tasks, and solutions.
Based on these files, the exam would be marked either manually or
using an automated tool (e.g. [`src2exam`](https://github.com/src2crs/src2exam)).

The exam in this directory contains only what is necessary to create a
report from these marked submissions.
Examples of documents using the `src2report` package with this exam
can be found in the [tests](../../../tests/tex/README.md) directory.
