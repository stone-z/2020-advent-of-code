use std::io::BufRead;
use std::fs::File;
use std::io::{self, BufReader};
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let filename = "input.txt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut numbers: HashSet<i32> = HashSet::with_capacity(200); // Hardcoded length

    for line in reader.lines(){
        let num = line.unwrap();

        let int: i32 = num.parse().unwrap();
        numbers.insert(int);
    }

    let (opt1, opt2) = find_sum(2020, numbers);

    println!("solution: {} + {} = 2020. {} * {} = {}", opt1, opt2, opt1, opt2, opt1*opt2);

    Ok(())
}

fn find_sum(target: i32, options: HashSet<i32>) -> (i32, i32) {
    for num in options.iter() {
        let diff = target - num;
        if options.contains(&diff) {
            return (*num, diff);
        }
    }
    return (0, 0);
}