use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, author, about, long_about=None)]
struct Args {
    #[arg(required = true)]
    title: Vec<String>,
    #[arg(short='n', default_value_t=false, help = "Do not print newline")]
    omit_newline: bool
}

fn main() {
    let args = Args::parse();
    print!("{}{}", args.title.join(" "), if args.omit_newline { "" } else { "\n" });
}
