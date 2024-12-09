mod day09;

fn main() {
    let start = std::time::Instant::now();
    day09::part1();
    println!("Finished in {:?}", start.elapsed());
}
