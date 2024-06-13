pub fn next_prime(nbr: u64) -> u64{
    fn is_prime(nbr: u64) -> bool{
        if nbr < 2 {
            return false
        }
        for i in 2..n{
            if n % i == 0{
                return false
            }
        }
        true
    }
    if is_prime(nbr){
        return nbr
    }
    let mut next = nbr+1;
    loop {
        if is_prime(next){
            return is_prime(next)
        }
        next += 1;
    }
}