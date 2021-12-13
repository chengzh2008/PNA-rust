use clap::{App, Arg, SubCommand};

fn main() {
  let matches = App::new("My Super Program")
    .version(env!("CARGO_PKG_VERSION"))
    .author("chengzh2008")
    .about("simple key/value in-memory store")
    .subcommand(
      SubCommand::with_name("get")
        .about("controls testing features")
        .arg(
          Arg::with_name("key1")
        )
    )
    .subcommand(
      SubCommand::with_name("set")
        .about("set key/value")
        .arg(
          Arg::with_name("key1")
        )
        .arg(Arg::with_name("value1"))
    )
    .subcommand(
      SubCommand::with_name("rm")
        .about("set key/value")
        .arg(
          Arg::with_name("key1")
        )
    )
    .get_matches();

  match matches.subcommand_name() {
    Some("get") => panic!("unimplemented"),
    Some("set") => panic!("unimplemented"),
    Some("rm") => panic!("unimplemented"),
    Some(&_) => panic!(),
    None => panic!(),
  }
}
