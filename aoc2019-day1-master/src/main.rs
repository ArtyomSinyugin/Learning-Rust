// считать цифры из файла modules.txt, разделить каждую на 3, округлить до целых чисел в меньшую сторону и вычесть два
// сложить все полученные ответы, вывести на экран

use std::{fs::File, io::{BufReader, BufRead}};
use day1::Mass;

fn main() {
    let file = File::open("modules.txt")
        .unwrap();

    let mass = BufReader::new(file).lines()
        .map(|mass| Mass::new_module(mass.ok()));

    //let mass_without_fuel = mass    
    //    .map(|mass| *mass.fuel_to_go_1().unwrap())
    //    .sum::<f32>();

    let mass_with_fuel = mass    
        .map(|mass| *mass.fuel_to_go_2().unwrap())
        .sum::<f32>();

    //println!("Ответ: {}", mass_without_fuel);
    println!("Ответ: {}", mass_with_fuel);
    println!("Завершено");
}


// -----------------------------------------------------------------------------------
// первая задача, блок if
/*
            let k = i % 3;
            let k = (i - k) / 3 - 2;
            k
*/

// несколько мудрёное решение, но не ищет правильного ответа. Хотя интереснее. 
/*
    // if i % 3 != 0 {                  
        print!("Дано: {i} "); 
        while i >= 6 {
         
                 let k = i % 3;
                 let k = (i - k) / 3 - 2;
                 i = k;
                 vec.push(k);
 
         }   
         println!("{:?}", vec);
         vec.iter().sum()
         
        } else {
         print!("Дано: {i} ");
         let i = i / 3 - 2;
         println!("[{i}]");  
         i          
     } 
*/