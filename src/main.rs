use databases::create_new_user;
use userid::get_usercount;

use clap::Parser;

mod databases;
mod userid;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    masterkey: String,
}

fn main() -> Result<(), rusqlite::Error> {
    let cli = Cli::parse();
    let masterkey = cli.masterkey;
    for _ in 0..10 {
        println!("Created user {}", create_new_user(masterkey.clone())?);
    }
    let count = get_usercount(masterkey)?;
    println!("Hello, world! and {} users!", count);
    Ok(())
}
