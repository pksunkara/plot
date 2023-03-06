use clap::Parser;

/// Command line utility for stylish interactive charts
#[derive(Debug, Parser)]
#[clap(name = "plotter")]
struct App {
    #[clap(subcommand)]
    cmd: Subcommands,
}

#[derive(Debug, Parser)]
enum Subcommands {}

fn main() {
    let program = App::parse();

    match program.cmd {
        // Subcommands::Pie(x) => x.run(),
    }
}
