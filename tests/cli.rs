// Copyright 2017-2023 Daniel P. Clark & other abrute Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! End-to-end tests that drive the compiled `abrute` binary against fixtures
//! encrypted with the `aescry` crate (the same crate abrute now uses to
//! decrypt), so the suite no longer depends on an external `aescrypt` binary.
//!
//! Every scenario lives in a single `#[test]` so the binary — which briefly
//! binds the JSON reporter's TCP port — is only ever launched one process at a
//! time.  Each scenario runs in its own temp directory so the resume file and
//! decrypted output never leak between cases.

use aescry::aescrypt::{Encryptor, Iterations};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

// A tiny iteration count keeps brute-forcing fast in the unoptimized test
// build; abrute reads the count back from the file header when it verifies.
const TEST_ITERATIONS: u32 = 5;

fn abrute(dir: &Path) -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_abrute"));
    // Isolate the `.abrute` resume file and decrypted output per scenario.
    cmd.current_dir(dir);
    cmd
}

fn scratch_dir(tag: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let mut dir = std::env::temp_dir();
    dir.push(format!("abrute-cli-{}-{}", tag, nanos));
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn write_fixture(dir: &Path, name: &str, password: &str, plaintext: &[u8]) {
    let stream = Encryptor::new(password)
        .unwrap()
        .iterations(Iterations::new(TEST_ITERATIONS).unwrap())
        .encrypt(plaintext)
        .unwrap();
    fs::write(dir.join(name), stream).unwrap();
}

// A password-protected zip fixture (password "4321") whose single entry,
// `zipped.file`, holds a short known plaintext.  Embedding it keeps the `-z`
// scenario self-contained rather than depending on a checked-out data folder.
const EXAMPLE_ZIP: &[u8] = include_bytes!("fixtures/example.zip");

#[test]
fn cli_end_to_end() {
    let plaintext = b"Hello World!\n";

    // 1) A password inside the search space is found and the file decrypted.
    {
        let dir = scratch_dir("found");
        write_fixture(&dir, "example.file.aes", "4321", plaintext);

        let status = abrute(&dir)
            .args(&["4", "1234", "--", "example.file.aes"])
            .status()
            .unwrap();

        assert!(status.success(), "expected success exit, got {:?}", status);
        assert_eq!(
            fs::read(dir.join("example.file")).unwrap(),
            plaintext,
            "decrypted output should match the original plaintext"
        );
        fs::remove_dir_all(&dir).ok();
    }

    // 2) The --start option begins the search partway through the space and
    //    still reaches a password that lives beyond the starting point.
    {
        let dir = scratch_dir("start");
        write_fixture(&dir, "example.file.aes", "4321", plaintext);

        let status = abrute(&dir)
            .args(&["4", "1234", "-s", "2222", "--", "example.file.aes"])
            .status()
            .unwrap();

        assert!(
            status.success(),
            "expected success with --start, got {:?}",
            status
        );
        assert_eq!(fs::read(dir.join("example.file")).unwrap(), plaintext);
        fs::remove_dir_all(&dir).ok();
    }

    // 3) A password outside the search space is reported as not found, and no
    //    plaintext is written.
    {
        let dir = scratch_dir("notfound");
        write_fixture(&dir, "example.file.aes", "4321", plaintext);

        let status = abrute(&dir)
            .args(&["2", "12", "--", "example.file.aes"])
            .status()
            .unwrap();

        assert!(
            !status.success(),
            "expected failure exit for a missing password"
        );
        assert!(
            !dir.join("example.file").exists(),
            "no plaintext file should be written when the password is not found"
        );
        fs::remove_dir_all(&dir).ok();
    }

    // 4) A file that is not AES Crypt data fails fast without writing output.
    {
        let dir = scratch_dir("notaes");
        fs::write(
            dir.join("plain.file.aes"),
            b"this is not an AES Crypt stream",
        )
        .unwrap();

        let status = abrute(&dir)
            .args(&["4", "1234", "--", "plain.file.aes"])
            .status()
            .unwrap();

        assert!(
            !status.success(),
            "expected failure for a non-AES-Crypt file"
        );
        assert!(
            !dir.join("plain.file").exists(),
            "no plaintext file should be written for a non-AES-Crypt input"
        );
        fs::remove_dir_all(&dir).ok();
    }

    // 5) A missing target file fails cleanly (non-zero exit, no panic).
    {
        let dir = scratch_dir("missing");
        let output = abrute(&dir)
            .args(&["4", "1234", "--", "does-not-exist.aes"])
            .output()
            .unwrap();

        assert!(
            !output.status.success(),
            "expected failure for a missing file"
        );
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            !stderr.contains("panicked"),
            "the binary should not panic on a missing file: {}",
            stderr
        );
        fs::remove_dir_all(&dir).ok();
    }

    // 6) The --adjacent limit still finds a password whose digits never repeat
    //    a neighbouring character class.  "4321" qualifies for every setting,
    //    so 0 (no adjacent repeats), 1 and 2 all recover the plaintext.
    for adjacent in &["0", "1", "2"] {
        let dir = scratch_dir(&format!("adjacent-{}", adjacent));
        write_fixture(&dir, "example.file.aes", "4321", plaintext);

        let status = abrute(&dir)
            .args(&["4", "1234", "-a", adjacent, "--", "example.file.aes"])
            .status()
            .unwrap();

        assert!(
            status.success(),
            "expected success with --adjacent {}, got {:?}",
            adjacent,
            status
        );
        assert_eq!(fs::read(dir.join("example.file")).unwrap(), plaintext);
        fs::remove_dir_all(&dir).ok();
    }

    // 7) The --zip path drives `unzip` and recovers the archived file.  The
    //    fixture's password is "4321"; abrute searches the "1234" space.
    {
        let dir = scratch_dir("zip");
        fs::write(dir.join("example.zip"), EXAMPLE_ZIP).unwrap();

        let status = abrute(&dir)
            .args(&["4", "1234", "-z", "--", "example.zip"])
            .status()
            .unwrap();

        assert!(
            status.success(),
            "expected success on the zip path, got {:?}",
            status
        );
        let recovered = dir.join("zipped.file");
        assert!(
            recovered.is_file(),
            "the archived file should be extracted next to the cwd"
        );
        assert!(
            !fs::read(&recovered).unwrap().is_empty(),
            "the extracted file should not be empty"
        );
        fs::remove_dir_all(&dir).ok();
    }
}

/// Argument-validation and robustness guards ported from the old shpec
/// `invalid.test` suite.  None of these reach the search loop, so the binary
/// exits before it would bind the JSON reporter's port — this test is safe to
/// run alongside `cli_end_to_end`.
#[test]
fn cli_rejects_invalid_input() {
    let dir = scratch_dir("invalid");

    // Each case: (args, human description).  Every one must exit non-zero
    // without panicking.
    let cases: &[(&[&str], &str)] = &[
        (&[], "no arguments at all"),
        (&["a:b", "asdf", "--", "missing.aes"], "a non-numeric range"),
        (
            &["4:4", "asdf", "--", "does-not-exist.aes"],
            "a missing target file",
        ),
        (&["asdf", "asdf", "--", "asdf"], "a garbage range"),
        (
            &["4", "asdf", "-a", "g", "--", "asdf"],
            "a non-numeric adjacent value",
        ),
        (
            &["4", "asdf", "-s", "1234", "--", "asdf"],
            "a start longer than the range",
        ),
    ];

    for (args, description) in cases {
        let output = abrute(&dir).args(*args).output().unwrap();
        assert!(
            !output.status.success(),
            "expected a non-zero exit for {}",
            description
        );
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            !stderr.contains("panicked"),
            "the binary should not panic for {}: {}",
            description,
            stderr
        );
    }

    fs::remove_dir_all(&dir).ok();
}
