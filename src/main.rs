mod day03;

fn main() {
    let start = std::time::Instant::now();
    day03::combined();
    println!("Finished in {:?}", start.elapsed());
}
