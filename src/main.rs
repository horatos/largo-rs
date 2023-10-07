use largo_rs::command::build_parser;

fn main() {
    let _ = build_parser(&[]).get_matches();
}
