use std::env;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

fn find_most_calorifically_dense_elf_in_file(path: String, top: usize) -> std::io::Result<u64> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut elves = Vec::new();
    let mut current_elf = 0;
    for line in reader.lines() {
        let maybe_number = line?;
        if maybe_number.is_empty() {
            elves.push(current_elf);
            current_elf = 0;
        } else {
            current_elf += maybe_number.parse::<u64>().unwrap()
        }
    }

    elves.sort();
    elves.reverse();

    Ok(elves.iter().take(top).sum())
}

fn main() {
    for argument in env::args().skip(1) {
        println!("{:?}", find_most_calorifically_dense_elf_in_file(argument.clone(), 1).unwrap());
        println!("{:?}", find_most_calorifically_dense_elf_in_file(argument.clone(), 3).unwrap());
    }
}
