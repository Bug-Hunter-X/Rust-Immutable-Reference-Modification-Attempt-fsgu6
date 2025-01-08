# Rust Immutable Reference Modification Attempt

This repository demonstrates a common error in Rust: attempting to modify a variable through an immutable reference. Rust's borrow checker prevents this to ensure memory safety and prevent data races.

The `bug.rs` file contains the erroneous code, while `bugSolution.rs` shows the corrected version.