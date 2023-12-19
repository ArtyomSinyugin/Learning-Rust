use std::collections::HashMap;
use std::io;

fn main() {

    let mut map: HashMap<String, String> = HashMap::new(); //создаём карту, где ключ Employers, значение нужно ввести

    loop {

        println!(
            "Введите:
            1 - добавить сотрудников в отдел Marketing
            2 - добавить сотрудников в отдел Sales
            3 - добавить сотрудников в отдел RnD
            4 - вывести на экран сотрудников Marketing
            5 - вывести на экран сотрудников Sales
            6 - вывести на экран сотрудников RnD
            7 - вывести всех сотрудников
            * - вернуться назад"
        );
        let mut name = String::new(); // вводим имя в петле
        
        // println!("{:?}", map);

        loop {
                let mut dep_choice = String::new(); // переменная выбора отдела
                //let mut marketing_all: Vec<String> = Vec::new(); // записываем всех сотрудников для вывода, после петли стираем
                //let mut sales_all: Vec<String> = Vec::new(); // записываем всех сотрудников для вывода, после петли стираем
                //let mut rnd_all: Vec<String> = Vec::new(); // записываем всех сотрудников для вывода, после петли стираем
                let mut employers_alph: Vec<String> = Vec::new(); // записываем всех сотрудников для вывода, !!!после второй петли стираем
                
                io::stdin()
                    .read_line(&mut dep_choice)
                    .expect("Failed to read line");
                
                //match dep_choice.as_str().trim() {
                match dep_choice.trim() {
                        "*" => break,
                        _ => {                
                            let dep_choice: i32 = match dep_choice.trim().parse() {
                            Ok(n) => n,
                            Err(_) => {
                                println!("Введите число от 1 до 7 или *!");
                                continue;
                                }
                            };

                            match dep_choice {
                                1 => {
                                    println!("Введите нового сотрудника отдела Marketing");
                                    io::stdin()
                                        .read_line(&mut name)
                                        .expect("Failed to read line");
                                    map.insert(name, String::from("marketing"));
                                    break;
                                },
                                2 => {
                                    println!("Введите нового сотрудника отдела Sales");
                                    io::stdin()
                                        .read_line(&mut name)
                                        .expect("Failed to read line");
                                    map.insert(name, String::from("sales"));
                                    break;
                                },
                                3 => {
                                    println!("Введите нового сотрудника отдела RnD");
                                    io::stdin()
                                        .read_line(&mut name)
                                        .expect("Failed to read line");
                                    map.insert(name, String::from("RnD"));
                                    break;                                    
                                },
                                4 => {
                                    for (key, value) in &map {
                                        if value.as_str().trim() == "marketing" {
                                            // println!("{}", key);
                                            employers_alph.push(key.clone());
                                        }
                                    }
                                    employers_alph.sort();
                                    println!("Сотрудники отдела маркетинга: {:#?}", employers_alph);
                                    break;                                    
                                },
                                5 => {
                                    for (key, value) in &map {
                                        if value.as_str().trim() == "sales" {
                                            // println!("{}", key);
                                            employers_alph.push(key.clone());
                                        }
                                    }
                                    employers_alph.sort();
                                    println!("Сотрудники отдела продаж: {:#?}", employers_alph);
                                    break;                                    
                                },
                                6 => {
                                    for (key, value) in &map {
                                        if value.as_str().trim() == "RnD" {
                                            // println!("{}", key);
                                            employers_alph.push(key.clone());
                                        }
                                    }
                                    employers_alph.sort();
                                    println!("Сотрудники отдела RnD: {:#?}", employers_alph);
                                    break;                                    
                                },
                                7 => {
                                    for (key, value) in &map {
                                        employers_alph.push(key.clone());
                                    }
                                    employers_alph.sort();
                                    println!("Все сотрудники в алфавитном порядке: {:#?}", employers_alph);
                                    break;                                    
                                },                                                                                                                              
                                _ => {
                                    println!("Введите число от 1 до 7 или *!!!");
                                    continue;
                                }
                            }
                        }
                    }

            } // завершилась первая петля
            
    }  // завершилась вторая петля

}
