use std::num::Float;

fn main() {
    let number : u64 = 600851475143; 
    let max : u32 = (number as f64).sqrt() as u32;
    let newmax_range : Vec<u32> =  (2..max+1).collect();
    let newmax : u32 =  *newmax_range.iter().rev().filter(|&x| number % (*x as u64) == 0).next().unwrap();

    let mut unsieved : Vec<u32> = (2..newmax+1).collect();
    let mut high_prime = 1;
    while !unsieved.is_empty() {
        let prime = unsieved.remove(0);
        if number % (prime as u64) == 0 {
            high_prime = prime;
        }
        let mut unsieved2 : Vec<u32> = vec![];
        for &x in unsieved.iter().filter(|&x| (*x as u32) % prime != 0) {
            unsieved2.push(x);
        }
        unsieved = unsieved2;
    }
    println!("{:?}", high_prime);
}
