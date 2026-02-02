use databases::create_new_user;
use userid::get_usercount;

mod databases;
mod userid;

fn main() -> Result<(), rusqlite::Error> {
    for _ in 0..10 {
        println!("Created user {}", create_new_user()?);
    }
    let count = get_usercount()?;
    println!("Hello, world! and {} users!", count);
    Ok(())
}
