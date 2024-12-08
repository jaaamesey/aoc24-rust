mod day08;

fn main() {
    let start = std::time::Instant::now();
    day08::part1();
    println!("Finished in {:?}", start.elapsed());
}
