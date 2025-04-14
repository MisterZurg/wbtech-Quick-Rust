use std::str::FromStr;
use bigdecimal::{
    BigDecimal,
    num_bigint::BigInt,
    // num_bigint::BigUint,
};
/*
Note:
- i128  form -2^127 to 2^127 - 1
- u128  from 0 to 2^128 - 1
 */
fn print_info<T>(a: T, b: T)
where
    T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + PartialEq
        + From<i32>
        + std::fmt::Display
        + Clone,
{
    println!("{a} * {b} = {}", a.clone() * b.clone());

    if b != T::from(0) {
        println!("{a} / {b} = {}", a.clone() / b.clone());
    } else {
        println!("Division by zero!");
    }

    println!("{a} + {b} = {}", a.clone() + b.clone());
    println!("{a} - {b} = {}", a.clone() - b.clone());
}

fn main() {
    let a_float = BigDecimal::from_str("1000000000000000000000.123456789").unwrap();
    let b_float = BigDecimal::from_str("2000000000000000000000.987654321").unwrap();

    print_info(a_float, b_float);

    let c_int = BigInt::from_str("-4000000000000000000000").unwrap();
    let d_int = BigInt::from_str("3000000000000000000000").unwrap();

    print_info(c_int, d_int);

    // let e_uint = BigUint::from_str("5000000000000000000000").unwrap();
    // let f_uint = BigUint::from_str("6000000000000000000000").unwrap();
    //
    // print_info(e_uint, f_uint);
}
