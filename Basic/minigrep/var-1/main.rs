//cargo run -- the poem.txt
use std::env;
use std::process;

use minigrep::Config;  // если подставить вместо "use minigrep::*;", то run() не нужно вызывать таким сложным способом
fn main() {
     
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);   // без этой строчки unwrap_or_else не работает. Работает просто unwrap, но криво
    });
    
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {  // вариант 1 обработки ошибки функции run()
        eprintln!("Application error: {e}");
        process::exit(1);  
    }

    //match run(config) { // вариант 2 обработки ошибки функции run()
    //    Ok(()) => todo!(),
    //    Err(e) => {
    //        println!("Application error: {e}");
    //        process::exit(1);   
    //    }
    //}

    //run(config).unwrap_or_else(|err| {   // вариант 3 обработки ошибки функции run()
    //    println!("Application error: {err}");
    //    process::exit(2);   // без этой строчки unwrap_or_else не работает. Работает просто unwrap, но криво
    //});

}
