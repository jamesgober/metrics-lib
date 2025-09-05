// Compile-check all Rust code blocks in docs/API.md using doc-comment.
// This treats fenced ```rust blocks as doctests.

// Ensure dev-dependency in Cargo.toml: doc-comment = "0.3"

doc_comment::doctest!("../docs/API.md");
