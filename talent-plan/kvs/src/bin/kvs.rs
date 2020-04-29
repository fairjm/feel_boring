use clap::{App, Arg, SubCommand};
use std::ops::Index;

fn main() {
    let matchers = App::new("kvs")
        .version("0.1.0")
        .about("simple memory kv store")
        .author("yuki")
        .subcommand(SubCommand::with_name("get")
            .about("get key from store")
            .arg(Arg::with_name("KEY")
                .index(1)
                .required(true)
                .help("get key")))
        .subcommand(SubCommand::with_name("set")
            .about("get key from store")
            .arg(Arg::with_name("KEY")
                .index(1)
                .required(true)
                .help("set key"))
            .arg(Arg::with_name("VALUE")
                .index(2)
                .required(true)
                .help("set value")))
        .get_matches();

    if let Some(matches) = matchers.subcommand_matches("get") {
        if matches.is_present("KEY") {
            panic!("unimplemented")
        }
    }

    if let Some(matches) = matchers.subcommand_matches("set") {
        panic!("unimplemented")
    }
}
