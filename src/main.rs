mod day14;

fn main() {
    let start = std::time::Instant::now();
    day14::part2();
    println!("Finished in {:?}", start.elapsed());
}
