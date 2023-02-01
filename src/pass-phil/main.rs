// use std::fs::File; // io::{BufReader, BufRead}};
// use std::io::{BufReader, BufRead};
// use std::io::Read;
use std::fs;
use jvm::*;
fn main() {
    //let mut file = String::new();       //  переделал по-своему. Мой результат: 1.39s и 0.01s, образец: 2.24s и 0.01s
    let content = fs::read_to_string("input.txt").unwrap();
    let iterator = content
        .lines();
/*
     let lines = BufReader                      //   пока не понимаю эту часть
        ::new(input)                            //
        .lines();                               //
    let iterator = lines                        //
        .into_iter()                            //
        .filter_map(|line| line.ok());          //
*/
    let result = iterator
        .map(|s| map_input_to_password(s))
        .filter(|(rule, s)| rule.is_valid(s))
        .count();
    println!("Count: {}", result);
}

