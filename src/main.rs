use std::env;
use std::error::Error; 
mod fact;
use std::string::String;

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
    //let args: Vec<String> = env::args().collect();
    if env::args().len() < 3 {
        println!("{}",usage);

    } else {
        //get arg 2 here
        let (arg2, is_ok) = getargi(2);
        let (arg1, is_k) = getarg(1);
        if is_ok && is_k {
            if env::args().len() < 4 { 
                match arg1.as_ref() {
                    "f" => println!("{}",fact::factorial(arg2)),
                    "p" => println!("{}",fact::plustorial(arg2)),
                    _ => println!("{}",usage),

                }
            } else {
                //get args here
                let (arg3, am_ok) = getargi(3);
                if am_ok {
                    match arg1.as_ref() {
                        "fn" => println!("{}",fact::factorialn(arg2, arg3)),
                        "c" => println!("{}",fact::c(arg2, arg3)),
                        "pn" => println!("{}",fact::plustorialn(arg2, arg3)),
                        _ => println!("{}", usage),
                    }
                }
            }
        } 
    }
}

fn t(s: &String) -> Result<u64,std::num::ParseIntError> {
    s.parse::<u64>() 
}

fn getargi(i: usize) -> (u64, bool) {
    let mut ok: bool = true;
    let arg2_option: String = match env::args().nth(i) {
        Some(v) => v,
        None => {
                    println!("{}", "There was an error parsing your arguments");
                    ok = false;
                    String::from("")
                },
    };
    let arg2_err = t(&arg2_option);
    let arg2 = match arg2_err {
        Ok(arg2) => arg2,
        Err(error) => {
            println!("{}", error.description());
            ok = false;
            0
        }
    };
    return (arg2, ok)
}

fn getarg(i: usize) -> (String, bool) {
    let ok: bool = true;
    let arg2: String = match env::args().nth(i) {
        Some(v) => v,
        None => panic!("lol"),
    };
    return (arg2, ok)
}
