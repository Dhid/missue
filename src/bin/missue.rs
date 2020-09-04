extern crate log;

use log::LevelFilter;
use missue::*;
use std::env::current_dir;
use std::process::exit;
use structopt::StructOpt;
use std::path::PathBuf;
use clap::AppSettings;

const DEFAULT_MISSUE_FOLDER: &str = ".missue";

#[derive(StructOpt, Debug)]
#[structopt(
    name = "missue",
    raw(global_settings = "&[AppSettings::DisableHelpSubcommand,AppSettings::VersionlessSubcommands]"))]

struct Opt {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "add", about = "Add an issue with given options")]
    Add {
        #[structopt(short = "n", long = "name", help = "A string name")]
        name: String,

        #[structopt(
            short = "d",
            long = "description",
            help = "A string description of the issue"
        )]
        description: Option<String>,
        #[structopt(
            short = "s",
            long = "status",
            help = "Sets the status of the issue",
            raw(possible_values = "&Status::variants()")
        )]
        status: Option<Status>,
    },
    #[structopt(name = "get", about = "Get the issue, given the name")]
    Get {
        #[structopt(short = "n", long = "name", help = "A string name")]
        name: String,
    },
    #[structopt(name = "rm", about = "Remove the issue, given the name")]
    Remove {
        #[structopt(short = "n", long = "name", help = "A string name")]
        name: String,
    },
    #[structopt(name = "ls", about = "Lists all the issue")]
    List
}

fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    //info!("missue {}", env!("CARGO_PKG_VERSION"));
    let opt = Opt::from_args();
    if let Err(e) = run(opt) {
        eprintln!("{}", e);
        exit(1);
    }
}

fn run(opt: Opt) -> Result<()> {
    let mut client = MissueClient::new(KvStore::open(get_missue_path())?);
    match opt.command {
        Command::Add {name, description, status } => {
            let mut issue = Issue::new(name);
            &issue.with_description(description);
            &issue.with_status(status);
            client.write_or_update(issue)?;
        },
        Command::Get {name} => {
            let issue = client.get(name)?;
            Printer::std_print(&issue);
        },
        Command::Remove {name} => {
            client.remove(name)?;
        },
        Command::List => {
            let list = client.open()?;
            for issue in list {
                Printer::std_print_string(issue);
            }
        }
    }
    Ok(())
}

fn get_missue_path() -> PathBuf{
    let mut path = current_dir().unwrap();
    path.push(DEFAULT_MISSUE_FOLDER);
    path
}