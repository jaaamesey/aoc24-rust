mod day04;

fn main() {
    let start = std::time::Instant::now();
    day04::part1();
    println!("Finished in {:?}", start.elapsed());
}
