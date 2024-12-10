mod day10;

fn main() {
    let start = std::time::Instant::now();
    day10::part1();
    println!("Finished in {:?}", start.elapsed());
}
