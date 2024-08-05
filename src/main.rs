
use clap::{Parser, Subcommand};
use inquire::Text;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    name: Option<String>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>
}


#[derive(Subcommand)]
enum Commands {
    Commit {
        #[arg(short, long)]
        message: Option<String>,

        #[arg(short, long)]
        all: Option<bool>
    }
}

fn handle_commit_message(message: Option<String>) -> String{
    let message_prompt = Text::new("What is your commit message?");

    match message {
        None => {
            let msg = message_prompt.prompt();


            match msg {
                Ok(str) =>{
                    if str.len() < 1{
                        return  handle_commit_message(message);
                    }
                    return str;
                },
                Err(_) => {
                    return handle_commit_message(message);
                }
            }
        }
        Some(str) => {
            return str;
        }
    }
}


fn main() {
    let cli = Cli::parse();



    match cli.command {
        Some(Commands::Commit { message, all }) => {
            let msg = handle_commit_message(message);
            

            println!("git commit -m {:?}", msg.trim())
        }
        None => {
            println!("no command given")
        }
    }
}
