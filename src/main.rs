use std::fs::DirBuilder;

use dirs::{home_dir};
use clap::{App, Arg, ArgMatches, SubCommand};
use git2::Repository;
use log::info;

fn init_data_dir(force: bool) -> Result<String, failure::Error> {
    let home = home_dir().ok_or_else(|| "Failed to find home dir".into())?;
    let home = home.to_str().ok_or_else(|| "failed to convert home path".into())?;
    let full_path = format!("{}/.rs-vm", home);
    let full_path_exists = std::fs::read_dir(&full_path);
    match (full_path_exists.is_ok(), force) {
        (false, _) => return Err(Box::new("Invalid home dir".to_string())),
        (true, true) =>
    }
    DirBuilder::new().recursive(true).create(&full_path)?;

    Ok(full_path)
}

fn setup_data_dir(dir: String) -> Result<(), Box<dyn std::error::Error>> {
    let plugin_repo_url = "https://github.com/asdf-vm/asdf-plugins";
    info!("cloning plugin repo");
    Repository::clone(plugin_repo_url, dir)?;
    info!("finished cloning plugin repo");

    Ok(())
}

fn main() {
    simple_logger::init().expect("failed to setup logger");
    let matches = App::new("rust version manager")
        .version("1.0")
        .subcommand(
            SubCommand::with_name("init")
                .about("initializes data directory")
                .arg(
                    Arg::with_name("force")
                        .short("f")
                        .help("overwrites data_dir if it already exists"),
                ),
        )
        .get_matches();

    // println!("{:?}", matches);
    if let Some(init_arg) = matches.subcommand_matches("init") {
        let data_dir = init_data_dir(init_arg.value_of("force").is_some()).expect("failed to create data dir");
        setup_data_dir(data_dir).expect("failed to clone plugins repo");
    }
}
