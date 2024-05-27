use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use sha2::{Sha256, Digest};
use std::process::exit;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        exit(1);
    }

    let mode = &args[1];
    if mode != "sha256" {
        println!("sha256 only authorized");
        exit(1)
    }
    let hash = &args[2];

    let passlist: File = File::open("list.txt").unwrap();

    println!("Cracking {}", hash);
    let reader = BufReader::new(passlist);

    for line in reader.lines() {
        let line: String = line.unwrap();
        let password = line.trim().to_owned().into_bytes();

        let passhash = format!("{:x}", Sha256::digest(&password));

        println!("{} == {}", std::str::from_utf8(&password).unwrap(), passhash);

        if &passhash == hash {
            println!("Password value : {}", line);
            exit(0)
        }

    }
    println!("password hash not found")

}
