fn main() {
    let primes = sieve(15);
    println!("{:?}", primes);
}
fn sieve(limit:usize)->Vec<usize>{
    let mut all_numbers_to_n=vec![true;limit+1];
    all_numbers_to_n[0]=false;
    all_numbers_to_n[1]=false;
    for i in 2..=limit{
        if all_numbers_to_n[i]{
            for multiple in (i*i..=limit).step_by(i){
                all_numbers_to_n[multiple]=false
            }
            
        }
    }
    all_numbers_to_n.iter().enumerate()
        .filter(|&(_, &is_prime)| is_prime)
        .map(|(num, _)| num)
        .collect()
   
}

