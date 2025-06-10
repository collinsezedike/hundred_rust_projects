fn main() {
    let prime_numbers = get_primes(3000000);
    println!("{:?}", prime_numbers);

    let prime_numbers = sieve_of_erastosthenes(3000000); // Way faster
    println!("{:?}", prime_numbers);
}


fn is_prime(num: u32) -> bool {
    match num {
        0 | 1 => false,
        2 => true,
        _ if num % 2 == 0 => false,
        _ => {
            for i in (3..=num.isqrt()).step_by(2) {
                if num % i == 0 {
                    return false;
                }
            }
            return true;
        }
    }
}


fn get_primes(n: u32) -> Vec<u32> {
    let mut primes = Vec::new();

    if n >= 2 {
        primes.push(2);
    }

    for i in (3..=n).step_by(2) {
        if is_prime(i) {
            primes.push(i);
        }
    }
    return primes;
}


fn sieve_of_erastosthenes(n: u32) -> Vec<usize> {
    let size = (n + 1) as usize;
    let mut primes_bool = vec![true; size];

    if size > 0 { primes_bool[0] = false; }
    if size > 1 { primes_bool[1] = false; }

    for i in 2..=primes_bool.len().isqrt() {
        if primes_bool[i] {
            for j in (i*i..size).step_by(i) {
                primes_bool[j] = false;
            }
        }
    }

    primes_bool.iter()
               .enumerate()
               .filter_map(|(index, &value)| { if value { Some(index) } else { None }})
               .collect()
}