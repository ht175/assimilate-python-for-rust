//A command-line tool to play Marco Polo
use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Noah Gift", about = "A Marco Polo game")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Noah Gift")]
    Marco {
        #[clap(short, long)]
        input: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Marco { input }) => {
            let result = hello::marco_polo(&input);
            println!("{}", result);
        }
        None => println!("No subcommand was used"),
    }
}
