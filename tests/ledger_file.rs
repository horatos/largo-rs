use std::path::Path;

use nix::{sys::wait::waitpid, sys::wait::WaitStatus, unistd::fork, unistd::ForkResult};

use largo_rs::ledger_file::LedgerFile;

#[test]
fn find_latest_ledger_in_default_data_should_success() {
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => {
            let status = waitpid(child, None).unwrap();
            assert_eq!(status, WaitStatus::Exited(child, 0));
        }
        Ok(ForkResult::Child) => {
            std::env::set_current_dir("tests/examples/default").unwrap();

            let ledger_file = LedgerFile::find_latest().unwrap();
            assert_eq!(ledger_file.path(), Path::new("book/2023.ledger"));

            std::process::exit(0);
        }
        Err(e) => {
            panic!("Failed to fork process: {e:?}");
        }
    }
}

#[test]
fn find_ledger_by_name_in_default_data_should_success() {
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => {
            let status = waitpid(child, None).unwrap();
            assert_eq!(status, WaitStatus::Exited(child, 0));
        }
        Ok(ForkResult::Child) => {
            std::env::set_current_dir("tests/examples/default").unwrap();

            let ledger_file = LedgerFile::find_by_name("2022").unwrap();
            assert_eq!(ledger_file.path(), Path::new("book/2022.ledger"));

            std::process::exit(0);
        }
        Err(e) => {
            panic!("Failed to fork process: {e:?}");
        }
    }
}

#[test]
fn find_missing_ledger_by_name_in_default_data_should_fail() {
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => {
            let status = waitpid(child, None).unwrap();
            assert_eq!(status, WaitStatus::Exited(child, 0));
        }
        Ok(ForkResult::Child) => {
            std::env::set_current_dir("tests/examples/default").unwrap();

            let ledger_file = LedgerFile::find_by_name("NOTFOUND");
            assert!(ledger_file.is_err());

            std::process::exit(0);
        }
        Err(e) => {
            panic!("Failed to fork process: {e:?}");
        }
    }
}
