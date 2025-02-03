use clap::Parser;

#[derive(Debug, Parser)]
enum Command {
    #[command(visible_alias = "m")]
    Map { function: String },
    #[command(visible_alias = "r")]
    Reduce { acc: String, function: String },
    #[command(visible_alias = "s")]
    Scan { acc: String, function: String },
}

fn main() {
    let cmd = Command::parse();
    println!("{cmd:?}");
}
