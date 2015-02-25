fn main() {
    let sum = (1..1000).fold(0, |sum, x| {
        match (x % 5,x % 3) {
            (_, 0)  => sum + x,
            (0, _)  => sum + x,
            _       => sum,
        }
    });
    println!("{}",sum)
}
