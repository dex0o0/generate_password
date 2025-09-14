use std::{io};

use rand::{ Rng};
fn grenerate_password(length:usize,use_num:bool,use_symbol:bool)-> String{
    let mut chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
    if use_num{
        chars.push_str("0123456789");
    }
    if use_symbol{
        chars.push_str("!@#$%^&*()_+-=[]{}|;:,.?");
    }
    let mut rng = rand::rng();
    (0..length).map(|_| {
        let idx = rng.random_range(0..chars.len());
        chars.chars().nth(idx).unwrap()
    }).collect()

}
fn main() {
    println!("Please enter length password");
    let mut len = String::new();
    io::stdin().read_line(&mut len).unwrap();
    let length:usize = len.trim().parse().expect("!!!!");
    println!("{}",grenerate_password(length, true, true));
}
