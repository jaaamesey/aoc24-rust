mod day12;

fn main() {
    let start = std::time::Instant::now();
    day12::part1();
    println!("Finished in {:?}", start.elapsed());
}
