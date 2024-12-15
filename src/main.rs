mod day15;

fn main() {
    let start = std::time::Instant::now();
    day15::part1();
    println!("Finished in {:?}", start.elapsed());
}
