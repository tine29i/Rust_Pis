pub fn is_prime(n: u64) -> bool {
    for i in 2..n{
        if n % i == 0{
            return false
        }
    }
    true
}

pub fn next_prime (n: u64) -> u64 {
    let mut next_number = n + 1;
    while next_number > 1 {
        if is_prime(next_number){
            return next_number
        }
        next_number+=1;
    }
    0
}