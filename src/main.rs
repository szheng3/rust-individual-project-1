use clap::Parser;

mod lib;

#[derive(Parser)]
#[clap(
version = "1.0",
author = "Shuai Zheng",
about = "Command-line interface for Summarization"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Shuai Zheng")]
    Text {
        #[clap(short, long)]
        input: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Text { input }) => {
            let summarization_model = lib::init_summarization_model();

            let mut inputs = [String::new(); 1];
            inputs[0] = input.to_owned();

            let _output = summarization_model.summarize(&inputs);
            let mut result = String::from(_output.join(" "));
            println!("Below is the summary:");

            println!("{:#?}", result);
        }
        None => {
            println!("No command given");
        }
    }
}