use kvs::KvStore;
use std::process;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "kvs", about = "A key-value store CLI")]
enum Command {
    #[structopt(name = "set", about = "Set a key-value pair")]
    Set { key: String, value: Option<String> },
    #[structopt(name = "get", about = "Get the value for a key")]
    Get { key: String },
    #[structopt(name = "rm", about = "remove a key")]
    Remove { key: String },
}

#[derive(Debug, StructOpt)]
#[structopt(name = "kvs", about = "A key-value store CLI")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

fn main() {
    let opt = Opt::from_args();

    // let mut kv_store = KvStore::new();

    match opt.cmd {
        Command::Set { key, value } => {
            match value {
                Some(val) => {
                    // kv_store.set(key, val);
                    eprintln!("unimplemented");
                    process::exit(-1)
                }
                None => {
                    eprintln!("unimplemented");
                    process::exit(-1);
                }
            }
        }
        Command::Get { key } => {
            // kv_store.get(key)
            eprintln!("unimplemented");
            process::exit(-1);
        }
        Command::Remove { key } => {
            // kv_store.remove(key);
            eprintln!("unimplemented");
            process::exit(-1);
        }
    }
}
