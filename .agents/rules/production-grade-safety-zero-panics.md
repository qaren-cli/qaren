---
trigger: always_on
---

Ban on Panics: The use of .unwrap() and .expect() is strictly forbidden in the core logic and production code.

Graceful Degradation: All functions that can fail must return a Result<T, CustomError>. Use the ? operator for error propagation.

Custom Error Types: Implement a centralized, robust custom error system (using thiserror if necessary) to handle I/O errors, parsing failures, and permission denials. The CLI must exit gracefully with the correct Exit Codes (0, 1, 2) and user-friendly error messages, NEVER a Rust panic trace.

Security First: The Secret Masking middleware must be implemented defensively. Assume all parsed data might contain secrets unless proven otherwise.