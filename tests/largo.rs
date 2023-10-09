use std::process::Command;

#[test]
fn largo_invoked_without_arguments_should_succeed() {
    let largo = std::fs::canonicalize("target/debug/largo-rs").unwrap();
    let output = Command::new(largo.as_path())
        .current_dir("tests/examples/default")
        .output()
        .unwrap();
    assert!(output.status.success(), "largo-rs failed: {:?}", output);
}

#[test]
fn largo_bs_with_dry_run_should_print_ledger_command() {
    let largo = std::fs::canonicalize("target/debug/largo-rs").unwrap();
    let output = Command::new(largo.as_path())
        .args(["bs", "--dry-run"])
        .current_dir("tests/examples/default")
        .output()
        .unwrap();
    assert!(output.status.success(), "largo-rs failed: {:?}", output);
    assert_eq!(
        String::from_utf8(output.stdout).unwrap().trim_end(),
        "-f book/2023.ledger balance -V ^資産 ^負債 ^純資産 --no-pager --force-color"
    );
}

#[test]
fn largo_bs_2022_with_dry_run_should_print_ledger_command() {
    let largo = std::fs::canonicalize("target/debug/largo-rs").unwrap();
    let output = Command::new(largo.as_path())
        .args(["bs", "2022", "--dry-run"])
        .current_dir("tests/examples/default")
        .output()
        .unwrap();
    assert!(output.status.success(), "largo-rs failed: {:?}", output);
    assert_eq!(
        String::from_utf8(output.stdout).unwrap().trim_end(),
        "-f book/2022.ledger balance -V ^資産 ^負債 ^純資産 --no-pager --force-color"
    );
}
