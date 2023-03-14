//A command-line tool to search wikipedia and summarize the content
//both via subcommands
use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Hugo")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Hugo")]
    Wiki {
        #[clap(short, long)]
        page: String,
    },
    #[clap(version = "1.0", author = "Hugo")]
    Sentimentwiki {
        #[clap(short, long)]
        page: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Wiki { page }) => {
            let content = Week8::get_wiki_content(&page);
            println!("{}", content);
        }
        Some(Commands::Sentimentwiki { page }) => {
            let content = Week8::get_wiki_content(&page);
            Week8::sentiment_content(&content);
        }
        None => println!("No subcommand was used"),
    }
}

