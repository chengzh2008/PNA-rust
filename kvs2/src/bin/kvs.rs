use clap::{App, AppSettings, Arg, SubCommand};
use kvs::{KvStore, KvsError, Result};
use std::env::current_dir;
use std::process::exit;

fn main() -> Result<()> {
        let matches = App::new(env!("CARGO_PKG_NAME"))
                .version(env!("CARGO_PKG_VERSION"))
                .author(env!("CARGO_PKG_AUTHORS"))
                .about(env!("CARGO_PKG_DESCRIPTION"))
                .setting(AppSettings::DisableHelpSubcommand)
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .setting(AppSettings::VersionlessSubcommands)
                .subcommand(
                        SubCommand::with_name("get")
                                .about("controls testing features")
                                .arg(Arg::with_name("key1").required(true)),
                )
                .subcommand(
                        SubCommand::with_name("set")
                                .about("set key/value")
                                .arg(Arg::with_name("key1").required(true))
                                .arg(Arg::with_name("value1").required(true)),
                )
                .subcommand(
                        SubCommand::with_name("rm")
                                .about("set key/value")
                                .arg(Arg::with_name("key1").required(true)),
                )
                .get_matches();

        match matches.subcommand() {
                ("set", Some(matches)) => {
                        let key = matches.value_of("key1").unwrap();
                        let value = matches.value_of("value1").unwrap();
                        let mut store = KvStore::open(current_dir()?)?;
                        store.set(key.to_string(), value.to_string())
                }
                ("get", Some(matches)) => {
                        let key = matches.value_of("key1").unwrap();
                        let mut store = KvStore::open(current_dir()?)?;
                        if let Some(value) = store.get(key.to_string())? {
                                println!("{}", value);
                                Ok(())
                        } else {
                                println!("Key not found");
                                Ok(())
                        }
                }
                ("rm", Some(matches)) => {
                        let key = matches.value_of("key1").unwrap();
                        let mut store = KvStore::open(current_dir()?)?;
                        match store.remove(key.to_string()) {
                                Ok(()) => Ok(()),
                                Err(KvsError::KeyNotFound) => {
                                        println!("Key not found");
                                        exit(1)
                                }
                                Err(e) => Err(e),
                        }
                }
                _ => unreachable!(),
        }
}
