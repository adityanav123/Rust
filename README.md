## Minigrep

Minigrep is a small, efficient text search CLI (Command Line Interface) application built using Rust. This project is inspired by and follows the guidelines provided in the Rust-lang book, specifically in the chapter dedicated to an I/O project. More details can be found here: [Rust-lang Book Chapter 12](https://doc.rust-lang.org/book/ch12-00-an-io-project.html).

---

### Execution

To use Minigrep, execute the following command in your terminal:

```bash
$ cargo run <text_to_search> <filename>
```
This command searches for the specified text within the provided file.

### Case Insensitive Search

To perform a case-insensitive search, set the CASE_INSENSITIVE environment variable to true:
```bash
$ export CASE_INSENSITIVE=true
```
To revert to case-sensitive search, unset the CASE_INSENSITIVE variable:
```bash
$ unset CASE_INSENSITIVE
```

### Testing

To run the tests included with Minigrep, use:

```bash
$ cargo test    
```
This ensures that all functionalities are working as expected.

---