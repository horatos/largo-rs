use std::path::{Path, PathBuf};

use eyre::{Result, WrapErr};

pub struct LedgerFile {
    ledger_path: PathBuf,
}

impl LedgerFile {
    pub fn find_latest() -> Result<Self> {
        let paths = std::fs::read_dir("book")
            .wrap_err("Failed to read directory `book`")?
            .map(|entry| {
                entry
                    .map(|entry| entry.path())
                    .wrap_err("Failed to read directory entry")
            })
            .collect::<Result<Vec<_>>>();
        let mut paths = paths?;
        paths.sort();

        let ledger_path = paths
            .last()
            .cloned()
            .ok_or_else(|| eyre::eyre!("book directory must not be empty"))?;

        Ok(Self { ledger_path })
    }

    pub fn find_by_name(name: &str) -> Result<Self> {
        let path: PathBuf = ["book", &format!("{name}.ledger")].iter().collect();

        match path.try_exists() {
            Ok(true) => Ok(Self { ledger_path: path }),
            Ok(false) => Err(eyre::eyre!("File not found: {}", path.display())),
            Err(e) => Err(e).wrap_err_with(|| format!("Failed to check file existence: {}", path.display())),
        }
    }

    pub fn path(&self) -> &Path {
        self.ledger_path.as_path()
    }
}
