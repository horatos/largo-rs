use clap::command;

pub fn build_parser(subcommands: &[String]) -> clap::Command {
    let command = command!();

    let subcommands: Vec<clap::Command> = subcommands
        .into_iter()
        .map(|s| {
            let s = s.clone();
            clap::Command::new(s)
                .arg(clap::Arg::new("file-name"))
                .arg(clap::Arg::new("dry-run").long("dry-run").action(clap::ArgAction::SetTrue))
        })
        .collect();

    command.subcommands(&subcommands)
}

#[cfg(test)]
mod test {
    use super::build_parser;

    fn subcommands() -> Vec<String> {
        vec![String::from("bs"), String::from("pl")]
    }

    #[test]
    fn parse_no_args() {
        let args = vec!["largo"];
        let matches = build_parser(&subcommands())
            .try_get_matches_from(args)
            .unwrap();
        assert!(!matches.args_present());
    }

    #[test]
    fn parse_bs_subcommand() {
        let args = vec!["largo", "bs"];

        let matches = build_parser(&subcommands())
            .try_get_matches_from(&args)
            .unwrap();

        assert!(matches!(matches.subcommand(), Some(("bs", _))));
    }

    #[test]
    fn parse_pl_subcommand() {
        let args = vec!["largo", "pl"];

        let matches = build_parser(&subcommands())
            .try_get_matches_from(&args)
            .unwrap();

        assert!(matches!(matches.subcommand(), Some(("pl", _))));
    }

    #[test]
    fn parse_undefined_subcommand() {
        let args = vec!["largo", "foo"];

        let matches = build_parser(&subcommands()).try_get_matches_from(&args);

        assert!(matches.is_err());
    }

    #[test]
    fn parse_bs_subcommand_with_year() {
        let args = vec!["largo", "bs", "2022"];

        let matches = build_parser(&subcommands())
            .try_get_matches_from(&args)
            .unwrap();

        assert!(matches!(matches.subcommand(), Some(("bs", _))));

        let (_, submatches) = matches.subcommand().unwrap();
        assert_eq!(submatches.get_one("file-name"), Some(&String::from("2022")));
    }
}
