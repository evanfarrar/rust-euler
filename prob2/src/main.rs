fn main() {
    let mut sum = 0;
    let mut last_fib = 1;
    let mut this_fib = 2;
    let mut next_fib;

    while last_fib < 4_000_000 {
        if this_fib % 2 == 0 {
            sum = sum + this_fib;
        }
        next_fib = this_fib + last_fib;
        last_fib = this_fib;
        this_fib = next_fib;
    }
    println!("{}", sum);
}
