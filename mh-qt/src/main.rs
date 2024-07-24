extern crate rand;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use rand::Rng;


fn main() -> io::Result<()> {


    let file = File::open("/home/vamsi/scripts/mh-qt/src/qt.txt")?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.trim().is_empty())
        .collect();

    let mut rng = rand::thread_rng();
    let rand_index = rng.gen_range(0..lines.len());

    println!("{}", lines[rand_index]);

    Ok(())
}
