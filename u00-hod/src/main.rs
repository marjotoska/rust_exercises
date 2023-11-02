fn main() {
    //Unit 0 - Work Sheet
    //Modify Hello World to print your name and age, both provided as arguments.
    let name = "Marjo";
    let age: u8 =  24;
    println!("Hello {}, of age {}!", name, age);

    //Rewrite fib to compute the value using a for loop.
    let nr = 7;
    let res = fib_loop(nr);
    println!("\n\nFibonacci number of {} is: {}", nr, res);

    //Implement a function for computing if a number is a prime number: fn is_prime(n: u32) -> bool. % is the modulo operator, which should be helpful.
    let nr = 233;
    let res = is_prime(nr);
    println!("\n\nIs {} prime? {}", nr, res);
}

fn is_prime(n: u32) -> bool{
    if n <= 1 || n % 2 == 0 || n % 3 == 0 {
        return false;
    } else if n == 2 || n == 3 {
        return true;
    }

    let mut i: u32 = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

//prime nr mathematical explanation, paragraph 6: https://en.wikipedia.org/wiki/Primality_test#Simple_methods

fn fib_loop(nr: u8) -> u16 {
    let mut a = 0;
    let mut b = 1;
    let mut c;
    if nr == 0 {
        0
    } else {
        for _ in 2..nr+1{
            c = a + b;
            a = b;
            b = c;
        };
        b
    }
}
/*
fib(0) = 0
    
fib(1) = 1 

fib(2) = 
    a = 0
    b = 1

    after loop step:
    c = a + b; c = 1
    a = b; a = 1
    b = c; b = 1
    return b; b = 1

fib(3) = 
    a = 1
    b = 1

    after loop step:
    c = a + b; c = 2
    a = b; a = 1
    b = c; b = 2
    return b; b = 2

fib(4) = 
    a = 1
    b = 2

    after loop step:
    c = a + b; c = 3
    a = b; a = 2
    b = c; b = 3
    return b; b = 3

fib(5) = 
    a = 2
    b = 3

    after loop step:
    c = a + b; c = 5
    a = b; a = 3
    b = c; b = 5
    return b; b = 5

fib(6) = 
    a = 3
    b = 5

    after loop step:
    c = a + b; c = 8
    a = b; a = 5
    b = c; b = 8
    return b; b = 8

fib(7) = 
    a = 5
    b = 8

    after loop step:
    c = a + b; c = 13
    a = b; a = 8
    b = c; b = 13
    return b; b = 13
*/