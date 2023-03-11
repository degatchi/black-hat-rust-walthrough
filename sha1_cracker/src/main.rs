use sha1::Digest;
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

const SHA1_HEX_STRING_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    //  Grabs all imports form `.env`.
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage:");
        println!("sha1_cracker: <wordlist.txt> <sha1_hash>");
        return Ok(());
    }

    let hash_to_crack = args[2].trim();
    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        return Err("sha1 hash is not valid".into());
    }

    // Reads the word_list file.
    let wordlist_file = File::open(&args[1])?;
    let reader = BufReader::new(&wordlist_file);

    // Compute the SHA-1 hash of each password.
    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        let common_password = line.trim();
        if hash_to_crack == &hex::encode(sha1::Sha1::digest(common_password.as_bytes())) {
            println!("Password found: {} - line: {}", &common_password, i);
            return Ok(());
        }
    }
    println!("password not found in word list :(");

    Ok(())
}
