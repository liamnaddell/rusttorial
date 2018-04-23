extern crate num_bigint;
extern crate num_traits;

use self::num_bigint::BigUint;
use self::num_traits::{Zero,One};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_factorialn_same() {
        assert_eq!(factorialn(5,0), factorial(5));
    }
    #[test]
    fn test_factorialn_compute() {
        assert_eq!(factorialn(5,3), t(b"20"));
        assert_eq!(factorialn(10,3),t(b"604800"));
    }
    #[test]
    fn test_factorialn_stop_bigger_than_num() {
        assert_eq!(factorialn(3,10), t(b"0"));
    }
    #[test]
    fn test_factorial_large() {
        assert_eq!(factorial(20), t(b"2432902008176640000"));
    }
    #[test]
    fn test_factorial_zero() {
        assert_eq!(factorial(0), t(b"1"));
    }
    #[test]
    fn test_c() {
        assert_eq!(c(6,2),t(b"15"));
        assert_eq!(c(6,4),t(b"15"));
        assert_eq!(c(8,3),t(b"56"));
        assert_eq!(c(8,5),t(b"56"));
        assert_eq!(c(8,0),t(b"1"));
        assert_eq!(c(26,2),t(b"325"));
    }
    #[test]
    fn test_plustorialn() {
        assert_eq!(plustorialn(10,0),t(b"55"));
        assert_eq!(plustorialn(5,0),plustorial(5));
    }
    fn t(num: &[u8]) -> BigUint {
        BigUint::parse_bytes(num, 10).unwrap()
    }
}

pub fn factorialn(mut num: u64, stop: u64) -> BigUint {

    let mut new_num: BigUint = One::one();
    if stop >= num {
        new_num = Zero::zero();
    } 

    while num >= stop+1 {
        if num == stop+1 {
            new_num=new_num*num;
            num = num-1;
        } else {
            new_num=new_num*num;
            num = num -1;
        }
    }
    return new_num;
}

pub fn factorial(num: u64) -> BigUint {
    factorialn(num, Zero::zero())
}

pub fn c(top:u64,bottom:u64) -> BigUint {
    if bottom != Zero::zero() {
        let retnum: BigUint = factorialn(top,top-bottom)/factorial(bottom);
        return retnum;
    } else {
        return One::one();
    }
}

pub fn plustorialn(num: u64, stop: u64) -> BigUint {
    /*
    let mut new_num: BigUint = Zero::zero();
    if stop >= num {
        new_num = Zero::zero();
    }
    while num >= stop+1 {
        if num == stop+1 {
            new_num=new_num+num;
            num = num-1;
        } else {
            new_num=new_num+num;
            num = num -1
        }
    }
    new_num*/
    plustorial(num)-plustorial(stop)
}

pub fn plustorial(num: u64) -> BigUint {
    //(n(n+1))/2
    let t: u64 = 2;
    let two = BigUint::from(t);
    let o: u64 = 1;
    let one: BigUint = BigUint::from(o);

    let big_num = &BigUint::from(num);
    let n_plus_one: BigUint = big_num+one;

    let ntn: BigUint = n_plus_one*big_num;
    let nd2: BigUint = ntn/two;
    nd2
}
