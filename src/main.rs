use std::{process::Command, str::FromStr};

use clap::{Parser, Subcommand};
use inquire::Text;

use strum::{EnumIter, EnumString};

mod autocomplete;
use autocomplete::command_autocomplete::CommandAutoComplete;

mod commands;
use commands::commit::handle_commit_message;

mod util;
use util::{
    commands::{execute, parse_command},
    text::uppercase,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    name: Option<String>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Subcommands>,
}

#[derive(Subcommand, EnumIter, EnumString)]
enum Subcommands {
    Clone {},
    Init {},
    Add {},
    Mv {},
    Restore {},
    Rm {},
    Bisect {},
    Diff {},
    Grep {},
    Log {},
    Show {},
    Status {},
    Branch {},
    Commit {
        #[arg(short, long)]
        message: Option<String>,

        #[arg(short, long, default_missing_value = "true")]
        all: bool,
    },
    Merge {},
    Rebase {},
    Reset {},
    Switch {},
    Tag {},
    Fetch {},
    Pull {},
    Push {},
}

fn handle_commands(mut cli: Cli) {
    match cli.command {
        Some(Subcommands::Clone {}) => {
            let mut cmd = parse_command("git clone");

            execute(&mut cmd);
        }
        Some(Subcommands::Init {}) => {
            let mut cmd = parse_command("git init");

            execute(&mut cmd);
        }
        Some(Subcommands::Add {}) => {
            let mut cmd = parse_command("git add");

            execute(&mut cmd);
        }
        Some(Subcommands::Mv {}) => {
            let mut cmd = parse_command("git mv");

            execute(&mut cmd);
        }
        Some(Subcommands::Restore {}) => {
            let mut cmd = parse_command("git restore");

            execute(&mut cmd);
        }
        Some(Subcommands::Rm {}) => {
            let mut cmd = parse_command("git rm");

            execute(&mut cmd);
        }
        Some(Subcommands::Bisect {}) => {
            let mut cmd = parse_command("git bisect");

            execute(&mut cmd);
        }
        Some(Subcommands::Diff {}) => {
            let mut cmd = parse_command("git diff");

            execute(&mut cmd);
        }
        Some(Subcommands::Grep {}) => {
            let mut cmd = parse_command("git grep");

            execute(&mut cmd);
        }
        Some(Subcommands::Log {}) => {
            let mut cmd = parse_command("git init");

            execute(&mut cmd);
        }
        Some(Subcommands::Show {}) => {
            let mut cmd = parse_command("git shpw");

            execute(&mut cmd);
        }
        Some(Subcommands::Status {}) => {
            let mut cmd = parse_command("git status");

            execute(&mut cmd);
        }
        Some(Subcommands::Branch {}) => {
            let mut cmd = parse_command("git branch");

            execute(&mut cmd);
        }
        Some(Subcommands::Commit { message, all }) => {
            let msg = handle_commit_message(message);

            match msg {
                Some(m) => {
                    let merge = format!("-m {:?}", m.trim());
                    let mut cmd = Command::new("git");
                    cmd.arg("commit");
                    cmd.arg(merge);
                    if all {
                        cmd.arg("-a");
                    }

                    execute(&mut cmd)
                }
                None => {
                    // Do nothing
                }
            }
        }
        Some(Subcommands::Merge {}) => {
            let mut cmd = parse_command("git merge");

            execute(&mut cmd);
        }
        Some(Subcommands::Rebase {}) => {
            let mut cmd = parse_command("git rebase");

            execute(&mut cmd);
        }
        Some(Subcommands::Reset {}) => {
            let mut cmd = parse_command("git reset");

            execute(&mut cmd);
        }
        Some(Subcommands::Switch {}) => {
            let mut cmd = parse_command("git switch");

            execute(&mut cmd);
        }
        Some(Subcommands::Tag {}) => {
            let mut cmd = parse_command("git tag");

            execute(&mut cmd);
        }
        Some(Subcommands::Fetch {}) => {
            let mut cmd = parse_command("git fetch");

            execute(&mut cmd);
        }
        Some(Subcommands::Pull {}) => {
            let mut cmd = parse_command("git pull");

            execute(&mut cmd);
        }
        Some(Subcommands::Push {}) => {
            let mut cmd = parse_command("git push");

            execute(&mut cmd);
        }
        None => {
            let command_prompt = Text::new("git").with_autocomplete(CommandAutoComplete::new());
            let res = command_prompt.prompt();

            match res {
                Ok(mut str) => {
                    uppercase(&mut str);
                    let cmd = Subcommands::from_str(&str.trim());

                    match cmd {
                        Ok(c) => {
                            cli.command = Some(c);
                            handle_commands(cli);
                        }
                        Err(_) => {
                            handle_commands(cli);
                        }
                    }
                }
                Err(_) => {
                    // Do nothing
                }
            }
        }
    }
}

fn main() {
    let cli = Cli::parse();

    handle_commands(cli);
}
