# Requirements Document: Qaren Configuration Comparison Tool

## Introduction

Qaren (قارن - "compare" in Arabic) is a blazingly fast, secure, offline CLI and GUI tool built in Rust for semantic and literal comparison of configuration files. Phase 1 focuses on key-value pair comparison with advanced parsing capabilities to handle real-world infrastructure tool outputs (AWS SSM, PM2, etc.), secret masking, and remediation features to generate missing configuration patches.

The tool addresses critical DevOps pain points: comparing production vs staging environments, detecting configuration drift, handling complex URLs with multiple delimiters, processing quoted outputs from cloud services, and generating actionable patches for CI/CD pipelines.

## Glossary

- **Qaren_CLI**: The command-line interface component of the Qaren tool
- **Qaren_GUI**: The native graphical user interface component of the Qaren tool
- **Parser**: The component responsible for reading and interpreting configuration file formats
- **Diff_Engine**: The component that performs comparison operations between parsed configurations
- **KV_Pair**: A key-value pair extracted from a configuration file (e.g., `DATABASE_URL=postgres://host:5432/db`)
- **Literal_Diff**: Line-by-line text comparison mode that preserves order
- **Semantic_Diff**: Order-agnostic comparison mode that focuses on key-value semantics
- **Delimiter**: The character separating keys from values (default `=`, configurable to `:` or others)
- **Safe_Splitting**: Parsing algorithm that splits only at the FIRST occurrence of the delimiter
- **Quote_Stripping**: Removal of surrounding quotation marks (`""` or `''`) from keys and values
- **Secret_Masking**: Hiding sensitive values for keys containing security-related keywords
- **Patch_File**: An output file containing only missing keys from a comparison
- **Exit_Code**: Numeric status returned by CLI (0=identical, 1=different, 2=error)
- **Comment_Line**: A line starting with `#` or `//` that should be ignored during parsing
- **Empty_Line**: A line containing only whitespace that should be ignored during parsing

## Requirements

### Requirement 1: Literal File Comparison

**User Story:** As a DevOps engineer, I want to perform line-by-line comparison of general configuration files, so that I can detect exact textual differences between versions.

#### Acceptance Criteria

1. WHEN the user executes `qaren diff file1 file2`, THE Qaren_CLI SHALL perform a line-by-line comparison preserving order
2. THE Diff_Engine SHALL output additions in green color
3. THE Diff_Engine SHALL output deletions in red color
4. THE Diff_Engine SHALL output modifications in yellow color
5. WHEN the files are identical, THE Qaren_CLI SHALL return Exit_Code 0
6. WHEN the files differ, THE Qaren_CLI SHALL return Exit_Code 1
7. WHEN an error occurs during comparison, THE Qaren_CLI SHALL return Exit_Code 2

### Requirement 2: Semantic Key-Value Pair Comparison

**User Story:** As a DevOps engineer, I want to compare environment variable files semantically, so that I can detect missing or changed configurations regardless of line order.

#### Acceptance Criteria

1. WHEN the user executes `qaren kvp file1 file2`, THE Qaren_CLI SHALL perform order-agnostic key-value comparison
2. THE Diff_Engine SHALL identify keys present in file1 but missing in file2
3. THE Diff_Engine SHALL identify keys present in file2 but missing in file1
4. THE Diff_Engine SHALL identify keys present in both files with different values
5. THE Diff_Engine SHALL identify keys present in both files with identical values
6. THE Diff_Engine SHALL ignore the order of KV_Pairs when determining equivalence
7. FOR ALL valid KV_Pair configurations, comparing file1 to file2 and then file2 to file1 SHALL produce inverse difference reports

### Requirement 3: Custom Delimiter Support

**User Story:** As a DevOps engineer, I want to specify custom delimiters for key-value separation, so that I can parse outputs from different infrastructure tools like PM2 that use colons instead of equals signs.

#### Acceptance Criteria

1. WHERE the user specifies `-d <delimiter>` or `--delimiter <delimiter>`, THE Parser SHALL use the specified delimiter instead of the default `=`
2. WHEN no delimiter is specified, THE Parser SHALL use `=` as the default delimiter
3. THE Parser SHALL support single-character delimiters including `=`, `:`, and space
4. WHEN a delimiter is specified, THE Parser SHALL apply it consistently to all lines in both files
5. WHEN an invalid delimiter is provided, THE Qaren_CLI SHALL return Exit_Code 2 with a descriptive error message

### Requirement 4: Safe Splitting Algorithm

**User Story:** As a DevOps engineer, I want the parser to split only at the first delimiter occurrence, so that complex URLs with query parameters are not corrupted during parsing.

#### Acceptance Criteria

1. WHEN parsing a line containing multiple delimiter characters, THE Parser SHALL split at the FIRST occurrence only
2. THE Parser SHALL treat all characters after the first delimiter as part of the value
3. WHEN parsing `URL=https://api.com?id=1&key=value` with delimiter `=`, THE Parser SHALL extract key `URL` and value `https://api.com?id=1&key=value`
4. WHEN parsing `"DATABASE_URL":"postgres://user:pass@host:5432/db"` with delimiter `:`, THE Parser SHALL split at the first `:` after quote stripping
5. FOR ALL lines with N delimiters where N >= 1, THE Parser SHALL produce exactly one KV_Pair with the value containing N-1 delimiters

### Requirement 5: Quote Stripping

**User Story:** As a DevOps engineer, I want to automatically remove surrounding quotes from keys and values, so that I can compare AWS SSM outputs that wrap values in quotes with local files that don't.

#### Acceptance Criteria

1. WHERE the user specifies `--strip-quotes`, THE Parser SHALL remove surrounding double quotes `""` from keys and values
2. WHERE the user specifies `--strip-quotes`, THE Parser SHALL remove surrounding single quotes `''` from keys and values
3. THE Parser SHALL only remove quotes that surround the entire key or value, not quotes within the text
4. WHEN parsing `"API_KEY"="abc123"` with `--strip-quotes`, THE Parser SHALL extract key `API_KEY` and value `abc123`
5. WHEN parsing `NAME='John "The Boss" Doe'` with `--strip-quotes`, THE Parser SHALL extract key `NAME` and value `John "The Boss" Doe`
6. WHEN `--strip-quotes` is not specified, THE Parser SHALL preserve all quotation marks as-is

### Requirement 6: Comment and Empty Line Handling

**User Story:** As a DevOps engineer, I want the parser to automatically ignore comments and empty lines, so that I can compare files with different documentation without false positives.

#### Acceptance Criteria

1. THE Parser SHALL ignore all Comment_Lines starting with `#`
2. THE Parser SHALL ignore all Comment_Lines starting with `//`
3. THE Parser SHALL ignore all Empty_Lines containing only whitespace characters
4. WHEN a line contains a key-value pair followed by a comment, THE Parser SHALL extract the key-value pair and ignore the comment portion
5. THE Parser SHALL treat lines with only whitespace and a comment character as Comment_Lines

### Requirement 7: Missing Configuration Patch Generation

**User Story:** As a DevOps engineer, I want to generate a file containing only the missing keys from a comparison, so that I can quickly remediate configuration gaps in staging or production environments.

#### Acceptance Criteria

1. WHERE the user specifies `--generate-missing <output_file>`, THE Qaren_CLI SHALL create a Patch_File at the specified path
2. THE Patch_File SHALL contain only KV_Pairs present in file1 but missing in file2 (default behavior)
3. THE Patch_File SHALL preserve the original formatting of the missing KV_Pairs from file1
4. THE Patch_File SHALL use the same delimiter as the source files
5. WHEN no keys are missing, THE Qaren_CLI SHALL create an empty Patch_File
6. WHEN the output file path is invalid or unwritable, THE Qaren_CLI SHALL return Exit_Code 2 with a descriptive error message
7. THE Qaren_CLI SHALL create the Patch_File after completing the comparison and displaying results

### Requirement 21: Bidirectional Patch Generation

**User Story:** As a DevOps engineer, I want to generate patch files for both directions (source-to-target and target-to-source), so that I can see what's missing in each file and decide which patches to apply.

#### Acceptance Criteria

1. WHERE the user specifies `--direction source-to-target`, THE Qaren_CLI SHALL generate a patch containing keys present in file1 but missing in file2 (default behavior)
2. WHERE the user specifies `--direction target-to-source`, THE Qaren_CLI SHALL generate a patch containing keys present in file2 but missing in file1
3. WHERE the user specifies `--direction bidirectional`, THE Qaren_CLI SHALL generate TWO patch files with suffixes `.source-to-target.env` and `.target-to-source.env`
4. WHEN using `--direction bidirectional`, THE output path SHALL be used as a prefix for both generated files
5. WHEN using `--direction bidirectional` with output path `patches/sync`, THE Qaren_CLI SHALL create `patches/sync.source-to-target.env` and `patches/sync.target-to-source.env`
6. THE `--direction` flag SHALL only be valid when `--generate-missing` is specified
7. WHEN `--direction` is specified without `--generate-missing`, THE Qaren_CLI SHALL return Exit_Code 2 with error message "Error: --direction requires --generate-missing"
8. WHEN no `--direction` is specified, THE Qaren_CLI SHALL default to `source-to-target` behavior for backward compatibility

### Requirement 8: Secret Masking

**User Story:** As a security-conscious DevOps engineer, I want sensitive values to be masked in terminal output, so that secrets are not exposed when sharing comparison results or during screen recordings.

#### Acceptance Criteria

1. THE Qaren_CLI SHALL mask values for keys containing the substring `key` (case-insensitive)
2. THE Qaren_CLI SHALL mask values for keys containing the substring `password` (case-insensitive)
3. THE Qaren_CLI SHALL mask values for keys containing the substring `secret` (case-insensitive)
4. THE Qaren_CLI SHALL mask values for keys containing the substring `token` (case-insensitive)
5. THE Qaren_CLI SHALL mask values for keys containing the substring `auth` (case-insensitive)
6. WHEN masking a value, THE Qaren_CLI SHALL display `***MASKED***` instead of the actual value
7. WHERE the user specifies `--show-secrets`, THE Qaren_CLI SHALL display all values unmasked
8. THE Secret_Masking SHALL apply only to terminal output, not to generated Patch_Files

### Requirement 9: Offline Execution and Memory Safety

**User Story:** As a security-conscious DevOps engineer, I want all processing to occur locally in memory without network access or temporary files, so that sensitive configuration data never leaves my machine.

#### Acceptance Criteria

1. THE Qaren_CLI SHALL perform all parsing operations in RAM without creating temporary files
2. THE Qaren_CLI SHALL perform all comparison operations in RAM without creating temporary files
3. THE Qaren_CLI SHALL not initiate any network connections during execution
4. THE Qaren_CLI SHALL not write to disk except when explicitly generating a Patch_File
5. WHEN processing completes or an error occurs, THE Qaren_CLI SHALL release all allocated memory

### Requirement 10: Native GUI with Drag and Drop

**User Story:** As a non-technical user, I want a graphical interface with drag-and-drop support, so that I can compare configuration files without using the command line.

#### Acceptance Criteria

1. THE Qaren_GUI SHALL provide two drop zones for file1 and file2
2. WHEN a user drags a file onto a drop zone, THE Qaren_GUI SHALL accept the file and display its path
3. WHEN both files are loaded, THE Qaren_GUI SHALL enable a "Compare" button
4. WHEN the user clicks "Compare", THE Qaren_GUI SHALL execute the comparison and display results
5. THE Qaren_GUI SHALL display additions in green color
6. THE Qaren_GUI SHALL display deletions in red color
7. THE Qaren_GUI SHALL display modifications in yellow color
8. THE Qaren_GUI SHALL use a minimalist design with clear visual hierarchy

### Requirement 11: GUI Settings Panel

**User Story:** As a GUI user, I want to configure parsing options through a settings panel, so that I can customize delimiter and quote handling without using command-line flags.

#### Acceptance Criteria

1. THE Qaren_GUI SHALL provide a settings panel accessible from the main interface
2. THE settings panel SHALL include a dropdown menu for selecting common delimiters (`=`, `:`, space)
3. THE settings panel SHALL include a text input field for custom delimiter entry
4. THE settings panel SHALL include a checkbox for enabling quote stripping
5. WHEN the user changes settings, THE Qaren_GUI SHALL apply them to subsequent comparisons
6. THE Qaren_GUI SHALL persist settings between application sessions
7. THE Qaren_GUI SHALL display current settings in the settings panel when opened

### Requirement 12: GUI Export Functionality

**User Story:** As a GUI user, I want to export missing configurations to a file, so that I can share remediation patches with my team without using the command line.

#### Acceptance Criteria

1. WHEN a comparison reveals missing keys, THE Qaren_GUI SHALL display an "Export Missing" button
2. WHEN the user clicks "Export Missing", THE Qaren_GUI SHALL open a file save dialog
3. THE Qaren_GUI SHALL suggest a default filename with `.env.patch` extension
4. WHEN the user confirms the save location, THE Qaren_GUI SHALL write the Patch_File to the specified path
5. WHEN the export succeeds, THE Qaren_GUI SHALL display a success notification
6. WHEN the export fails, THE Qaren_GUI SHALL display an error message with the failure reason

### Requirement 13: Performance and Resource Efficiency

**User Story:** As a DevOps engineer working with large configuration files, I want the tool to execute quickly with minimal resource usage, so that I can integrate it into automated pipelines without performance penalties.

#### Acceptance Criteria

1. THE Qaren_CLI SHALL complete comparison of two 1000-line files within 100 milliseconds on standard hardware
2. THE Qaren_CLI SHALL consume less than 50 MB of RAM when processing files up to 10,000 lines
3. THE Parser SHALL process files in a single pass without re-reading
4. THE Diff_Engine SHALL use efficient data structures for O(n) comparison complexity where n is the number of KV_Pairs
5. THE Qaren_CLI SHALL compile to a single static binary with no external runtime dependencies

### Requirement 14: Error Handling and User Feedback

**User Story:** As a user, I want clear and actionable error messages when something goes wrong, so that I can quickly understand and fix the problem.

#### Acceptance Criteria

1. WHEN a specified file does not exist, THE Qaren_CLI SHALL display an error message indicating which file was not found
2. WHEN a file cannot be read due to permissions, THE Qaren_CLI SHALL display an error message indicating the permission issue
3. WHEN a file contains invalid UTF-8 encoding, THE Qaren_CLI SHALL display an error message indicating the encoding issue
4. WHEN parsing fails on a specific line, THE Qaren_CLI SHALL display the line number and content in the error message
5. WHEN an invalid command or option is provided, THE Qaren_CLI SHALL display usage help with available commands and options
6. THE Qaren_CLI SHALL write all error messages to stderr, not stdout
7. THE Qaren_GUI SHALL display error messages in modal dialogs with clear descriptions

### Requirement 15: Cross-Platform Compatibility

**User Story:** As a DevOps engineer working across different operating systems, I want Qaren to work consistently on Linux, macOS, and Windows, so that I can use the same tool regardless of my environment.

#### Acceptance Criteria

1. THE Qaren_CLI SHALL compile and execute on Linux x86_64 systems
2. THE Qaren_CLI SHALL compile and execute on macOS x86_64 and ARM64 systems
3. THE Qaren_CLI SHALL compile and execute on Windows x86_64 systems
4. THE Qaren_GUI SHALL compile and execute on Linux x86_64 systems with native window management
5. THE Qaren_GUI SHALL compile and execute on macOS x86_64 and ARM64 systems with native window management
6. THE Qaren_GUI SHALL compile and execute on Windows x86_64 systems with native window management
7. THE Qaren_CLI SHALL handle platform-specific line endings (LF on Unix, CRLF on Windows) transparently

### Requirement 16: Command-Line Interface Design

**User Story:** As a DevOps engineer, I want a consistent and intuitive command-line interface, so that I can quickly learn and remember the tool's syntax.

#### Acceptance Criteria

1. THE Qaren_CLI SHALL accept commands in the format `qaren <type> <file1> <file2> [options]`
2. THE Qaren_CLI SHALL support the comparison type `diff` for literal comparison
3. THE Qaren_CLI SHALL support the comparison type `kvp` for key-value pair comparison
4. THE Qaren_CLI SHALL display help text when invoked with `--help` or `-h`
5. THE Qaren_CLI SHALL display version information when invoked with `--version` or `-v`
6. THE Qaren_CLI SHALL support short flags (single dash, single character) and long flags (double dash, full word)
7. WHEN invoked without arguments, THE Qaren_CLI SHALL display usage help

### Requirement 17: Configuration File Format Support

**User Story:** As a DevOps engineer, I want to work with common configuration file formats, so that I can compare environment files from various tools and platforms.

#### Acceptance Criteria

1. THE Parser SHALL correctly parse `.env` file format with `KEY=value` syntax
2. THE Parser SHALL correctly parse AWS SSM parameter outputs with quoted keys and values
3. THE Parser SHALL correctly parse PM2 environment dumps with colon delimiters
4. THE Parser SHALL correctly parse shell export statements in the format `export KEY=value`
5. WHEN parsing shell exports, THE Parser SHALL treat `export` as a prefix to ignore, not part of the key
6. THE Parser SHALL handle keys containing alphanumeric characters, underscores, and hyphens
7. THE Parser SHALL handle values containing any printable characters including spaces and special symbols

### Requirement 18: Comparison Output Format

**User Story:** As a DevOps engineer, I want comparison results in a clear, scannable format, so that I can quickly identify what changed between configurations.

#### Acceptance Criteria

1. THE Qaren_CLI SHALL group comparison results into sections: Missing in file2, Missing in file1, Modified, Identical
2. THE Qaren_CLI SHALL display the section name before listing items in that section
3. THE Qaren_CLI SHALL display each difference with the format `KEY: value1 -> value2` for modifications
4. THE Qaren_CLI SHALL display each missing key with the format `KEY: value` for additions/deletions
5. WHERE Secret_Masking is active, THE Qaren_CLI SHALL display masked values in the comparison output
6. THE Qaren_CLI SHALL display a summary line at the end showing total counts for each category
7. WHEN files are identical, THE Qaren_CLI SHALL display "Files are identical" and exit with code 0

### Requirement 19: Build and Distribution

**User Story:** As a user, I want to download and run Qaren without installing additional dependencies, so that I can start using it immediately.

#### Acceptance Criteria

1. THE Qaren_CLI SHALL compile to a single static binary with no external dependencies
2. THE Qaren_GUI SHALL compile to a single executable with embedded resources
3. THE build process SHALL produce binaries for Linux x86_64, macOS x86_64, macOS ARM64, and Windows x86_64
4. THE binary size SHALL be less than 10 MB for CLI and less than 20 MB for GUI
5. THE Qaren_CLI SHALL not require installation and SHALL execute directly from any directory
6. THE Qaren_GUI SHALL not require installation and SHALL execute directly from any directory

### Requirement 20: Parsing Robustness

**User Story:** As a DevOps engineer, I want the parser to handle malformed or edge-case inputs gracefully, so that I can still get useful results even when configuration files have minor issues.

#### Acceptance Criteria

1. WHEN a line contains no delimiter, THE Parser SHALL skip the line and continue processing
2. WHEN a line contains only a key with no value, THE Parser SHALL treat the value as an empty string
3. WHEN a line contains only a delimiter with no key, THE Parser SHALL skip the line and continue processing
4. WHEN a line contains unbalanced quotes, THE Parser SHALL attempt to parse it as-is without quote stripping
5. WHEN a file is empty, THE Parser SHALL return an empty set of KV_Pairs without error
6. WHEN a file contains only comments and empty lines, THE Parser SHALL return an empty set of KV_Pairs without error
7. THE Parser SHALL continue processing all lines even when individual lines fail to parse

### Requirement 22: GUI Bidirectional Export

**User Story:** As a GUI user, I want to export patches for both directions, so that I can see what's missing in each file without using the command line.

#### Acceptance Criteria

1. WHEN a comparison reveals missing keys in either direction, THE Qaren_GUI SHALL display an "Export Options" dropdown button
2. THE "Export Options" dropdown SHALL include three choices: "Export Source → Target", "Export Target → Source", "Export Both Directions"
3. WHEN the user selects "Export Source → Target", THE Qaren_GUI SHALL export keys present in file1 but missing in file2
4. WHEN the user selects "Export Target → Source", THE Qaren_GUI SHALL export keys present in file2 but missing in file1
5. WHEN the user selects "Export Both Directions", THE Qaren_GUI SHALL open a file save dialog and create two files with `.source-to-target.env` and `.target-to-source.env` suffixes
6. THE Qaren_GUI SHALL display the count of missing keys for each direction in the export options (e.g., "Export Source → Target (5 keys)")
7. WHEN an export succeeds, THE Qaren_GUI SHALL display a success notification showing the file path(s) created
