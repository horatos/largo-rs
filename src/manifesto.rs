use std::collections::HashMap;

use eyre::{Result, WrapErr};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Manifesto {
    project: Project,
    ledger: Ledger,
    #[serde(flatten)]
    commands: Commands,
}

#[derive(Deserialize)]
pub struct Project {
    pub largo: String,
}

#[derive(Deserialize)]
pub struct Ledger {
    pub bin: String,
    #[serde(rename = "default-options")]
    pub default_options: Vec<String>,
}

#[derive(Deserialize)]
pub struct Commands {
    commands: HashMap<String, Vec<String>>,
}

impl Manifesto {
    /// Load the project manifesto file in the current directory
    pub fn load() -> Result<Self> {
        let s = std::fs::read_to_string("Largo.toml")
            .wrap_err("Failed to open or read the file Largo.toml")?;

        Self::load_from_str(s.as_str())
    }

    pub fn load_from_str(s: &str) -> Result<Self> {
        toml::from_str(s).wrap_err("Failed to parse TOML file")
    }

    pub fn largo(&self) -> &str {
        self.project.largo.as_str()
    }

    pub fn largo_subcommands(&self) -> impl Iterator<Item = &String> {
        self.commands.names()
    }

    pub fn ledger_bin(&self) -> &str {
        self.ledger.bin.as_str()
    }

    pub fn ledger_default_options(&self) -> &Vec<String> {
        &self.ledger.default_options
    }

    pub fn ledger_args(&self, subcommand: &str) -> Option<&Vec<String>> {
        self.commands.get(subcommand)
    }
}

impl Commands {
    pub fn get(&self, name: &str) -> Option<&Vec<String>> {
        self.commands.get(name)
    }

    pub fn names(&self) -> impl Iterator<Item = &String> {
        self.commands.keys()
    }
}

#[cfg(test)]
mod test {
    use super::Manifesto;

    const LARGO_TOML: &str = r#"
[project]
largo = "largo-rs"

[ledger]
bin = "/opt/local/bin/ledger"
default-options = ["--no-pager", "--force-color"]

[commands]
bs = ["balance", "-V", "^資産", "^負債", "^純資産"]
pl = ["balance", "^収益", "^費用"]
"#;

    #[test]
    fn load_project_manifesto() {
        let manifesto = Manifesto::load_from_str(LARGO_TOML).unwrap();

        assert_eq!(manifesto.largo(), "largo-rs");
        assert_eq!(manifesto.ledger_bin(), "/opt/local/bin/ledger");
        assert_eq!(
            manifesto.ledger_default_options(),
            &vec![String::from("--no-pager"), String::from("--force-color")]
        );

        let want: Vec<String> = ["balance", "-V", "^資産", "^負債", "^純資産"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect();
        let got = manifesto.ledger_args("bs");
        assert_eq!(got, Some(&want));

        let want: Vec<String> = ["balance", "^収益", "^費用"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect();
        let got = manifesto.ledger_args("pl");
        assert_eq!(got, Some(&want));

        assert_eq!(manifesto.ledger_args("foobar"), None);
    }
}
