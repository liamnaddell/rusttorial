#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_factorialn_same() {
        assert_eq!(factorialn(5,0), factorial(5));
    }
    #[test]
    fn test_factorialn_compute() {
        assert_eq!(factorialn(5,3), 20);
        assert_eq!(factorialn(10,3),604800);
    }
    #[test]
    fn test_factorialn_stop_bigger_than_num() {
        assert_eq!(factorialn(3,10), 0);
    }
    #[test]
    fn test_factorial_large() {
        assert_eq!(factorial(20), 2432902008176640000);
    }
    #[test]
    fn test_factorial_zero() {
        assert_eq!(factorial(0), 0);
    }
    #[test]
    fn test_c() {
        assert_eq!(c(6,2),15);
        assert_eq!(c(6,4),15);
        assert_eq!(c(8,3),56);
        assert_eq!(c(8,5),56);
    }
}

pub fn factorialn(mut num: u64, stop: u64) -> u64 {
    let mut new_num: u64 = 1;
    if stop >= num {
        new_num = 0;
    } 
    while num >= stop+1 {
        if num == stop+1 {
            new_num=new_num*num;
            num = num-1;
        } else {
            new_num=new_num*num;
            num = num -1
        }
    }
    return new_num;
}

pub fn factorial(num: u64) -> u64 {
    return factorialn(num, 0);
}

pub fn c(top:u64,bottom:u64) -> u64 {
    let retnum = factorialn(top,top-bottom)/factorial(bottom);
    return retnum;
}
