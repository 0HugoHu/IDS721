//Searches a path for duplicate files
//Compares serial and parallel versions of the program
use clap::Parser;

#[derive(Parser)]
//add extended help
#[clap(
    version = "1.0",
    author = "Noah Gift",
    about = "Compares serial and parallel versions of the program",
    after_help = "Example: parallel parallel --path /src/data/"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    ParallelMd5 {
        #[clap(short, long, default_value = "src/data/")]
        path: String,
    },
    SerialMd5 {
        #[clap(short, long, default_value = "src/data/")]
        path: String,
    },
    ParallelSha256 {
        #[clap(short, long, default_value = "src/data/")]
        path: String,
    },
    SerialSha256 {
        #[clap(short, long, default_value = "src/data/")]
        path: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::ParallelMd5 { path }) => {
            println!("Parallel version of the program");
            let files = week5::walk(&path).unwrap();
            let checksums = week5::checksum_par(files).unwrap();
            for (checksum, files) in checksums {
                if files.len() > 1 {
                    println!("{}:", checksum);
                    for file in files {
                        println!("\t{}", file);
                    }
                }
            }
        }
        Some(Commands::SerialMd5 { path }) => {
            println!("Serial version of the program");
            let files = week5::walk(&path).unwrap();
            let checksums = week5::checksum(files).unwrap();
            for (checksum, files) in checksums {
                if files.len() > 1 {
                    println!("{}:", checksum);
                    for file in files {
                        println!("\t{}", file);
                    }
                }
            }
        }
        Some(Commands::ParallelSha256 { path }) => {
            println!("Parallel version of the program");
            let files = week5::walk(&path).unwrap();
            let checksums = week5::checksum_par_sha(files).unwrap();
            for (checksum, files) in checksums {
                if files.len() > 1 {
                    println!("{}:", checksum);
                    for file in files {
                        println!("\t{}", file);
                    }
                }
            }
        }
        Some(Commands::SerialSha256 { path }) => {
            println!("Serial version of the program");
            let files = week5::walk(&path).unwrap();
            let checksums = week5::checksum_sha(files).unwrap();
            for (checksum, files) in checksums {
                if files.len() > 1 {
                    println!("{}:", checksum);
                    for file in files {
                        println!("\t{}", file);
                    }
                }
            }
        }
        None => println!("No command specified"),
    }
}
