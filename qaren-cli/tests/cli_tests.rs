//! Integration tests for the Qaren CLI.
//!
//! Tests the binary end-to-end using `assert_cmd`.
//! Exit code semantics (default behaviour):
//!   0 = files are identical
//!   1 = files have differences
//!   2 = real error (missing file, invalid args, etc.)

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Get a Command for the qaren binary.
fn qaren_cmd() -> Command {
    Command::cargo_bin("qaren").expect("binary should exist")
}

/// Helper: create a temp file with content and return its path string.
fn create_temp_file(dir: &TempDir, name: &str, content: &str) -> std::path::PathBuf {
    let path = dir.path().join(name);
    fs::write(&path, content).expect("write temp file");
    path
}

// ─── No-args and help ──────────────────────────────────────────────

#[test]
fn test_no_args_shows_usage() {
    qaren_cmd()
        .assert()
        .failure() // clap exits 2 on no args
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn test_help_flag() {
    qaren_cmd()
        .arg("--help")
        .assert()
        .success() // code 0
        .stdout(predicate::str::contains("qaren"));
}

#[test]
fn test_version_flag() {
    qaren_cmd()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("qaren"));
}

// ─── kv: Identical files → exit 0 ─────────────────────────────────

#[test]
fn test_kv_identical_files_exit_0() {
    let tmp = TempDir::new().unwrap();
    let f1 = create_temp_file(&tmp, "a.env", "KEY=value\nDB=host\n");
    let f2 = create_temp_file(&tmp, "b.env", "KEY=value\nDB=host\n");

    qaren_cmd()
        .args(["kv", &f1.display().to_string(), &f2.display().to_string()])
        .assert()
        .code(0)
        .stdout(predicate::str::contains("identical"));
}

// Legacy alias `kvp` still works
#[test]
fn test_kvp_alias_still_works() {
    let tmp = TempDir::new().unwrap();
    let f1 = create_temp_file(&tmp, "a.env", "KEY=value\n");
    let f2 = create_temp_file(&tmp, "b.env", "KEY=value\n");

    qaren_cmd()
        .args(["kvp", &f1.display().to_string(), &f2.display().to_string()])
        .assert()
        .code(0);
}

// ─── kv: Different files → exit 1 (default POSIX mode) ────────────

#[test]
fn test_kv_different_files_exit_1_by_default() {
    let tmp = TempDir::new().unwrap();
    let f1 = create_temp_file(&tmp, "a.env", "KEY=value1\n");
    let f2 = create_temp_file(&tmp, "b.env", "KEY=value2\n");

    qaren_cmd()
        .args(["kv", &f1.display().to_string(), &f2.display().to_string()])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("Modified"));
}

// ─── kv: Missing file → exit 2 ────────────────────────────────────

#[test]
fn test_kv_missing_file_exit_2() {
    let tmp = TempDir::new().unwrap();
    let f1 = create_temp_file(&tmp, "a.env", "KEY=value\n");

    qaren_cmd()
        .args([
            "kv",
            &f1.display().to_string(),
            "nonexistent_file.env",
        ])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("not found").or(predicate::str::contains("error")));
}

// ─── kv: Missing keys detection (real filename in output) ──────────

#[test]
fn test_kv_missing_keys_shown_with_real_filename() {
    let tmp = TempDir::new().unwrap();
    let f1 = create_temp_file(&tmp, "a.env", "A=1\nB=2\nC=3\n");
    let f2 = create_temp_file(&tmp, "b.env", "A=1\n");

    qaren_cmd()
        .args(["kv", &f1.display().to_string(), &f2.display().to_string()])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("Only in").and(predicate::str::contains("a.env")));
}

// ─── kv: Custom delimiter ─────────────────────────────────────────

#[test]
fn test_kv_custom_delimiter() {
    let tmp = TempDir::new().unwrap();
    let f1 = create_temp_file(&tmp, "a.txt", "KEY: value1\n");
    let f2 = create_temp_file(&tmp, "b.txt", "KEY: value2\n");

    qaren_cmd()
        .args([
            "kv",
            &f1.display().to_string(),
            &f2.display().to_string(),
            "-d",
            ":",
        ])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("Modified"));
}

// ─── kv: Per-file delimiter (cross-format comparison) ─────────────

#[test]
fn test_kv_per_file_delimiter() {
    let tmp = TempDir::new().unwrap();
    // file1 uses '=' format, file2 uses ': ' format
    let f1 = create_temp_file(&tmp, "prod.env", "DB=postgres://host:5432/db\nKEY=secret\n");
    let f2 = create_temp_file(&tmp, "staging.yaml", "DB: postgres://host:5432/db\nKEY: secret\n");

    qaren_cmd()
        .args([
            "kv",
            &f1.display().to_string(),
            &f2.display().to_string(),
            "--d1", "=",
            "--d2", ":",
        ])
        .assert()
        .code(0)
        .stdout(predicate::str::contains("identical"));
}

// ─── kv: Auto-detect delimiter ────────────────────────────────────

#[test]
fn test_kv_auto_detect_equals() {
    let tmp = TempDir::new().unwrap();
    let f1 = create_temp_file(&tmp, "a.env", "KEY=value\nDB=host\n");
    let f2 = create_temp_file(&tmp, "b.env", "KEY=value\nDB=host\n");

    qaren_cmd()
        .args(["kv", &f1.display().to_string(), &f2.display().to_string()])
        .assert()
        .code(0)
        .stdout(predicate::str::contains("identical"));
}

#[test]
fn test_kv_auto_detect_colon() {
    let tmp = TempDir::new().unwrap();
    let f1 = create_temp_file(&tmp, "a.yaml", "KEY: value\nDB: host\n");
    let f2 = create_temp_file(&tmp, "b.yaml", "KEY: value\nDB: host\n");

    qaren_cmd()
        .args(["kv", &f1.display().to_string(), &f2.display().to_string()])
        .assert()
        .code(0)
        .stdout(predicate::str::contains("identical"));
}

// ─── kv: Invalid delimiter → exit 2 ──────────────────────────────

#[test]
fn test_kv_invalid_delimiter_exit_2() {
    let tmp = TempDir::new().unwrap();
    let f1 = create_temp_file(&tmp, "a.env", "KEY=value\n");
    let f2 = create_temp_file(&tmp, "b.env", "KEY=value\n");

    qaren_cmd()
        .args([
            "kv",
            &f1.display().to_string(),
            &f2.display().to_string(),
            "-d",
            "==",
        ])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("Invalid delimiter"));
}

// ─── kv: Secret masking ───────────────────────────────────────────

#[test]
fn test_kv_secret_masking() {
    let tmp = TempDir::new().unwrap();
    let f1 = create_temp_file(&tmp, "a.env", "API_KEY=super-secret-123\n");
    let f2 = create_temp_file(&tmp, "b.env", "API_KEY=different-secret\n");

    // Without --show-secrets: values should be masked
    qaren_cmd()
        .args(["kv", &f1.display().to_string(), &f2.display().to_string()])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("***MASKED***"));
}

#[test]
fn test_kv_show_secrets() {
    let tmp = TempDir::new().unwrap();
    let f1 = create_temp_file(&tmp, "a.env", "API_KEY=super-secret-123\n");
    let f2 = create_temp_file(&tmp, "b.env", "API_KEY=different-secret\n");

    // With --show-secrets (-S): actual values should appear
    qaren_cmd()
        .args([
            "kv",
            &f1.display().to_string(),
            &f2.display().to_string(),
            "-S",
        ])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("super-secret-123"));
}

// ─── kv: Ignore case ──────────────────────────────────────────────

#[test]
fn test_kv_ignore_case() {
    let tmp = TempDir::new().unwrap();
    let f1 = create_temp_file(&tmp, "a.env", "KEY=Production\n");
    let f2 = create_temp_file(&tmp, "b.env", "KEY=production\n");

    // Without -i: differences found
    qaren_cmd()
        .args(["kv", &f1.display().to_string(), &f2.display().to_string()])
        .assert()
        .code(1);

    // With -i: identical
    qaren_cmd()
        .args(["kv", &f1.display().to_string(), &f2.display().to_string(), "-i"])
        .assert()
        .code(0)
        .stdout(predicate::str::contains("identical"));
}

// ─── kv: Ignore whitespace ────────────────────────────────────────

#[test]
fn test_kv_ignore_whitespace() {
    let tmp = TempDir::new().unwrap();
    let f1 = create_temp_file(&tmp, "a.env", "KEY=hello   world\n");
    let f2 = create_temp_file(&tmp, "b.env", "KEY=hello world\n");

    qaren_cmd()
        .args(["kv", &f1.display().to_string(), &f2.display().to_string(), "-w"])
        .assert()
        .code(0)
        .stdout(predicate::str::contains("identical"));
}

// ─── kv: Brief mode ───────────────────────────────────────────────

#[test]
fn test_kv_brief_mode() {
    let tmp = TempDir::new().unwrap();
    let f1 = create_temp_file(&tmp, "a.env", "A=1\nB=2\n");
    let f2 = create_temp_file(&tmp, "b.env", "A=1\n");

    qaren_cmd()
        .args(["kv", &f1.display().to_string(), &f2.display().to_string(), "-q"])
        .assert()
        .code(1)
        // Brief mode should show Summary but NOT per-key details
        .stdout(predicate::str::contains("Summary"))
        .stdout(predicate::str::contains("Only in").not());
}

// ─── kv: Patch generation ─────────────────────────────────────────

#[test]
fn test_kv_generate_patch_source_to_target() {
    let tmp = TempDir::new().unwrap();
    let f1 = create_temp_file(&tmp, "a.env", "A=1\nB=2\n");
    let f2 = create_temp_file(&tmp, "b.env", "A=1\n");
    let patch_path = tmp.path().join("patch.env");

    qaren_cmd()
        .args([
            "kv",
            &f1.display().to_string(),
            &f2.display().to_string(),
            "-g",
            &patch_path.display().to_string(),
        ])
        .assert()
        .code(1)
        .stderr(predicate::str::contains("Patch file created"));

    // Verify patch contains the missing key
    let patch_content = fs::read_to_string(&patch_path).expect("read patch");
    assert!(patch_content.contains("B=2"));
}

#[test]
fn test_kv_generate_patch_bidirectional() {
    let tmp = TempDir::new().unwrap();
    let f1 = create_temp_file(&tmp, "a.env", "ONLY_A=1\n");
    let f2 = create_temp_file(&tmp, "b.env", "ONLY_B=2\n");
    let patch_path = tmp.path().join("sync.env");

    qaren_cmd()
        .args([
            "kv",
            &f1.display().to_string(),
            &f2.display().to_string(),
            "-g",
            &patch_path.display().to_string(),
            "--direction",
            "bidirectional",
        ])
        .assert()
        .code(1)
        .stderr(predicate::str::contains("Patch file created"));

    let s2t = tmp.path().join("sync.source-to-target.env");
    let t2s = tmp.path().join("sync.target-to-source.env");
    assert!(s2t.exists(), "source-to-target patch should be created");
    assert!(t2s.exists(), "target-to-source patch should be created");

    let s2t_content = fs::read_to_string(&s2t).expect("read s2t");
    let t2s_content = fs::read_to_string(&t2s).expect("read t2s");
    assert!(s2t_content.contains("ONLY_A=1"));
    assert!(t2s_content.contains("ONLY_B=2"));
}

// ─── kv: Quote stripping ──────────────────────────────────────────

#[test]
fn test_kv_strip_quotes() {
    let tmp = TempDir::new().unwrap();
    let f1 = create_temp_file(&tmp, "a.env", "KEY=\"value1\"\n");
    let f2 = create_temp_file(&tmp, "b.env", "KEY=value1\n");

    // With -s (--strip-quotes), the files should be identical
    qaren_cmd()
        .args([
            "kv",
            &f1.display().to_string(),
            &f2.display().to_string(),
            "-s",
        ])
        .assert()
        .code(0)
        .stdout(predicate::str::contains("identical"));
}

// ─── diff: Literal comparison ─────────────────────────────────────

#[test]
fn test_diff_identical_files() {
    let tmp = TempDir::new().unwrap();
    let f1 = create_temp_file(&tmp, "a.txt", "line1\nline2\n");
    let f2 = create_temp_file(&tmp, "b.txt", "line1\nline2\n");

    qaren_cmd()
        .args(["diff", &f1.display().to_string(), &f2.display().to_string()])
        .assert()
        .code(0)
        .stdout(predicate::str::contains("identical"));
}

#[test]
fn test_diff_different_files() {
    let tmp = TempDir::new().unwrap();
    let f1 = create_temp_file(&tmp, "a.txt", "line1\nline2\n");
    let f2 = create_temp_file(&tmp, "b.txt", "line1\nchanged\n");

    qaren_cmd()
        .args(["diff", &f1.display().to_string(), &f2.display().to_string()])
        .assert()
        .code(1);
}

// ─── kv: --direction without --generate-missing → exit 2 ──────────

#[test]
fn test_kv_direction_without_generate_missing() {
    let tmp = TempDir::new().unwrap();
    let f1 = create_temp_file(&tmp, "a.env", "KEY=value\n");
    let f2 = create_temp_file(&tmp, "b.env", "KEY=value\n");

    qaren_cmd()
        .args([
            "kv",
            &f1.display().to_string(),
            &f2.display().to_string(),
            "--direction",
            "bidirectional",
        ])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("--generate-missing"));
}

// ─── config command ───────────────────────────────────────────────

#[test]
fn test_config_show_runs_without_error() {
    qaren_cmd()
        .args(["config", "show"])
        .assert()
        .code(0);
}

#[test]
fn test_config_exit_show() {
    qaren_cmd()
        .args(["config", "exit", "show"])
        .assert()
        .code(0)
        .stdout(predicate::str::contains("exit nonzero-on-diff"));
}

#[test]
fn test_config_color_show() {
    qaren_cmd()
        .args(["config", "color", "show"])
        .assert()
        .code(0)
        .stdout(predicate::str::contains("color output"));
}
