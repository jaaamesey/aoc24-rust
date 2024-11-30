mod day01;

fn main() {
    let start = std::time::Instant::now();
    day01::part1();
    println!("Finished in {:?}", start.elapsed());
}
