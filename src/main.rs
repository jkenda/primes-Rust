fn is_prime(list:&[u32], num:u32) -> bool
{
    for prime in list.iter()
    {
        if num % prime == 0
        {
            return false;
        }
    }
    true
}

fn find_primes(limit:u32) -> Vec<u32>
{
    let mut list = vec![2, 3];
    for num in list[list.len()-1] + 2 .. limit
    {
        if is_prime(&list, num)
        {
            list.push(num);
        }
    }
    list
}

fn main() {
    println!("Primes:\n{:?}\nThere are {} primes.", find_primes(1000000));
}