use eyre::{Result, WrapErr};

use crate::command;
use crate::ledger_file::LedgerFile;
use crate::manifesto::Manifesto;

pub struct App {
    manifesto: Manifesto,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(App {
            manifesto: Manifesto::load()?,
        })
    }

    pub fn run(&self) -> Result<()> {
        let subcommands = self
            .manifesto
            .largo_subcommands().cloned()
            .collect::<Vec<_>>();
        let mut command = command::build_parser(&subcommands);
        let matches = command.clone().get_matches();

        match matches.subcommand() {
            Some((subcommand, matches)) => self.process_subcommand(subcommand, matches)?,
            None => command.print_help()?,
        }

        Ok(())
    }

    fn process_subcommand(&self, name: &str, matches: &clap::ArgMatches) -> Result<()> {
        let mut args = vec![];

        args.push("-f".to_owned());
        let ledger_file = match matches.get_one::<String>("file-name") {
            Some(file_name) => LedgerFile::find_by_name(file_name)?,
            None => LedgerFile::find_latest()?,
        };
        args.push(ledger_file.path().to_str().unwrap().to_owned());

        let mut ledger_args = self.manifesto.ledger_args(name).unwrap().clone();
        let mut default_options = self.manifesto.ledger_default_options().clone();

        args.append(&mut ledger_args);
        args.append(&mut default_options);

        if matches.get_flag("dry-run") {
            let args = args.as_slice().join(" ");
            println!("{args}");
        } else {
            let mut child = std::process::Command::new(self.manifesto.ledger_bin())
                .args(args)
                .spawn()
                .wrap_err_with(|| format!("Failed to spawn ledger-cli: {}", self.manifesto.ledger_bin()))?;
            child.wait().wrap_err("Failed to wait child process")?;
        }

        Ok(())
    }
}
