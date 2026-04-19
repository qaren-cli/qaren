---
trigger: always_on
---

Strict Decoupling: The core diffing engine and parser (qaren-core) must be 100% independent of the CLI (qaren-cli) and GUI (qaren-gui). Do not mix clap or egui logic inside the parsing or diffing functions.

Step-by-Step Execution: Do NOT generate the entire project code at once.

Step 1: Write the core data structures (structs/enums) and the parsing logic, then STOP and ask for my review.

Step 2: Write the diffing engine and secret masking logic, then STOP.

Step 3: Wire up the CLI interface.

Test-Driven Focus: For every core module you implement (especially the custom delimiter and quote-stripping parser), you must write inline unit tests (#[cfg(test)]) covering edge cases (e.g., URLs with multiple delimiters, malformed lines) before moving to the next component.