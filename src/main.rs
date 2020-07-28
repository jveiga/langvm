use std::fs::DirBuilder;

use clap::{App, Arg, ArgMatches, SubCommand};
use dirs::home_dir;
use git2::Repository;
use log::{debug, error, info};
use thiserror::Error as TError;

const PLUGIN_REPO_URL: &'static str = "https://github.com/asdf-vm/asdf-plugins";

#[derive(TError, Debug)]
enum EError {
    MissingHomeDir,
    UnableToCreateDir(std::io::Error),
}

impl std::fmt::Display for EError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, ".....")
    }
}

impl std::convert::From<EError> for std::io::Error {
    fn from(e: EError) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, e)
    }
}

fn init_data_dir() -> Result<String, EError> {
    let home = home_dir().ok_or(EError::MissingHomeDir)?;
    let full_path = format!("{}/.rs-vm", home.to_string_lossy());
    debug!("creating {}", &full_path);
    DirBuilder::new()
        .recursive(true)
        .create(&full_path)
        .map_err(EError::UnableToCreateDir)?;

    Ok(full_path)
}

fn setup_data_dir(dir: String) -> std::io::Result<()> {
    info!("cloning plugin repo");
    Repository::clone(PLUGIN_REPO_URL, dir)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    info!("finished cloning plugin repo");

    Ok(())
}

fn update() -> std::io::Result<()> {
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    simple_logger::init().expect("failed to setup logger");
    let matches = App::new("rust version manager")
        .version("1.0")
        .subcommand(SubCommand::with_name("init").about("initializes data directory"))
        .get_matches();

    if matches.subcommand_matches("init").is_some() {
        let data_dir = init_data_dir()?;
        setup_data_dir(data_dir)?;
    } else if matches.subcommand_matches("update").is_some() {
        update()?;
    } else {
        println!("{}", matches.usage());
    }
    Ok(())
}
