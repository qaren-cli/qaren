---
trigger: always_on
---

No Unauthorized Crates: You are strictly forbidden from introducing any third-party crates not explicitly approved in the Tech Spec (e.g., stick to clap, egui, similar).

No Regex for Parsing: Do NOT use the regex crate for parsing the Key-Value files. It is too slow and heavy. Use standard Rust string manipulation techniques (e.g., &str slicing, .split_once(), .strip_prefix(), .trim()) to ensure maximum speed and minimal memory footprint.

Memory Efficiency: Avoid unnecessary memory allocations. Pass references (&str, &[T]) instead of cloning strings (String::clone()) whenever possible. Your parsing logic must operate entirely in-memory without creating temporary files.