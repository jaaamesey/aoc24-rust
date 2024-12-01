mod day01;

fn main() {
    let start = std::time::Instant::now();
    // day01::part1();
    day01::part2();
    println!("Finished in {:?}", start.elapsed());
}
