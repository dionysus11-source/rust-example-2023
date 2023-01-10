fn main() {
    let mut primes = [0;100];
    get_primes(&mut primes);
    println!("{:?}",primes);
}

fn get_primes(primes:&mut[usize;100]){
    let mut i :usize= 2;
    let mut count=0;
    while count <100{
        if is_prime(i){
            primes[count] = i;
            count +=1;
        }
        i+=1;
    }
}

fn is_prime(number:usize)-> bool{
    for i in 2..number{
        if number % i == 0{
            return false
        }
    }
    return true

}
