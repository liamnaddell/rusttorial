extern crate num;
extern crate rusttorial;

use rusttorial::{c,factorial,factorialn,plustorial,plustorialn};
use num::bigint::BigUint;
use std::process::exit;
use std::string::String;
use std::ffi::OsString;
use std::env;
use std::error::Error; 


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
    if env::args_os().len() < 3 {
        println!("{}",usage);

    } else {
        //get arg 2 here
        let (arg2, is_ok) = getargi(2);
        let (arg1, is_k) = getarg(1);
        if is_ok && is_k {
            if env::args_os().len() < 4 { 
                match arg1.as_ref() {
                    "f" => p(factorial(arg2)),
                    "p" => p(plustorial(arg2)),
                    _ => println!("{}",usage),

                }
            } else {
                //get args here
                let (arg3, am_ok) = getargi(3);
                if am_ok {
                    match arg1.as_ref() {
                        "fn" => p(factorialn(arg2,arg3)),
                        "c" => { 
                            let ret = c(arg2,arg3).unwrap_or_else(|| {
                                println!("rusttorial: integer overflow. Try specifying values like this: rusttorial c 5 4");
                                exit(1)
                            });
                            p(ret);
                        }
                        "pn" => p(plustorialn(arg2,arg3)),
                        _ => println!("{}", usage),
                    }
                }
            }
        } 
    }
}

fn p(my_int: BigUint) {
    println!("{}",my_int)
}

fn t(s: &String) -> Result<u64,std::num::ParseIntError> {
    s.parse::<u64>() 
}

//get arg integer
fn getargi(i: usize) -> (u64, bool) {
    //are we good?
    let mut ok: bool = true;

    //this is an osString which can contain invalid unicode
    let arg2_option: OsString = match env::args_os().nth(i) {
        Some(v) => v,
        None => {
                    print!("There was an error parsing your arguments");
                    ok = false;
                    OsString::from("")
                },
    };

    //converts OsString to string with losses of invalid unicode 
    let arg2_string: String = os_string_to_string(arg2_option);

    //converts to int
    let arg2_err = t(&arg2_string);
    let arg2 = match arg2_err {
        Ok(arg2) => arg2,
        Err(error) => {
            println!("{}", error.description());
            ok = false;
            0
        }
    };

    //return if ok, and return ints
    return (arg2, ok)
}

//get arg string
fn getarg(i: usize) -> (String, bool) {
    //ok?
    let ok: bool = true;
    //convert 
    let arg2: OsString = match env::args_os().nth(i) {
        Some(v) => v,
        None => panic!("error getting arguments"),
    };
    let arg2_string: String = os_string_to_string(arg2);
    return (arg2_string, ok)
}


fn os_string_to_string(s: OsString) -> String {
    s.to_string_lossy().to_string()
}

