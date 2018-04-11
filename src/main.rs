use std::env;
mod fact;

fn main() {
    let usage = String::from("USAGE: rusttorial f|fn|c num [num2]
    f is a standard factorial function that takes one input, and returns the factorial of that number
    fn is a factorial number that stops the factorial function when it reaches the second number specified ex.
        rusttorial fn 5 2 = 60
        It is basically equivilent to factorial(num1)/factorial(num2).
    c is the choose function ex.
        rusttorial c 4 2 = 6
        it is calculated with
        fn(num1,num1-num2)/f(num2);
    p is the plustorial function 
        plustorial is a factorial with addition instead of multiplication
    pn is the plustorial function that stops at stop ex.
        rusttorial pn 5 3 = 5+4
                             ");
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("{}",usage);

    } else {
        if args.len() < 4 { 
            if args[1] == "f" {
                println!("{}",fact::factorial(t(&args[2])));
            } else if args[1] == "p" {
                println!("{}",fact::plustorial(t(&args[2])));
            } else {
                println!("{}",usage);
            }
        } else {
            if args[1] == "fn" {
                println!("{}",fact::factorialn(t(&args[2]), t(&args[3])));
            }
            if args[1] == "c" {
                println!("{}",fact::c(t(&args[2]), t(&args[3])));
            }
            if args[1] == "pn" {
                println!("{}",fact::plustorialn(t(&args[2]), t(&args[3])));
            }
        }
    }
}

    fn t(s: &String) -> u64 {
        return s.parse::<u64>().unwrap();
    }
