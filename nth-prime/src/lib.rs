
pub fn nth(n: u32) -> u32 {

    let mut array_of_primes = Vec::new();

    let mut counter = 2;
    let mut how_many_primes = 0;

    if n > 0 {
        while how_many_primes < n {
            counter += 1;
            if is_prime(&counter, &array_of_primes) {
                how_many_primes += 1;
                array_of_primes.push(counter);
            }
        }
    }

    return counter as u32
}

fn is_prime(n: &usize, array_of_primes: &Vec<usize>) -> bool {
    for i in array_of_primes {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}