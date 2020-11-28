extern crate ignore;
use std::fs::OpenOptions;
use std::io::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    #[structopt(
        short = "w",
        long = "write",
        parse(try_from_str),
        default_value = "false",
        help = "Create or update .gitignore file"
    )]
    write: bool,
    #[structopt(short, long, parse(try_from_str), required = true)]
    keywords: Vec<String>
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let opt = Opt::from_args();
    if opt.write {
        let content = ignore::generate(opt.keywords).await?;
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(".gitignore")
            .unwrap();

        if let Err(e) = writeln!(file, "{}", content) {
            eprintln!("Couldn't write to .gitignore file: {}", e);
        }        
    } else {
        println!("{}", ignore::generate(opt.keywords).await?);
    }
    Ok(())
}
