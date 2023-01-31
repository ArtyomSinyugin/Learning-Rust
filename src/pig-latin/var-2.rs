//use std::vec; 
//use std::io;
fn main() {   // второе решение задачи
    
    //let mut sentence = String::new ();
    let message = String::from("Some sentence in English");
    let mut pigged_string = String::new();

    for word in message.split_whitespace() {
        let mut pigged_word = String::new();

        let mut chunk = word.chars();  // chunk - не массив, а итератор. Можно явно его собрать в массив: let chunk = message.chars().collect::<Vec<_>>() Или взять следующий элемент итератора, поглотив его: chunk.next()
        println!("Так выглядит итератор: {:?}", chunk);
        let x = chunk.next();
        //println!("{:?} поглотил {:?}", chunk, x);
        
        match x {
            Some ('a' | 'e' | 'i' | 'o' | 'u' | 'y' | 'A' | 'E' | 'I' | 'O' | 'U' | 'Y') => {
                // как то отредактировать слово. добавить к нему "hey"
                pigged_word = format!("{}-hey ", word);
                println! ("Это слово на гласную - {}", pigged_word);
            },
            Some(_) => {
                // как то отредактировать слово. первую букву в конец и добавить "ey"               
                pigged_word = format!("{}-{}ey ", String::from_iter(chunk), String::from_iter(x));
                println! ("Это слово на согласную - {}", pigged_word);
            },
            None => println! ("Нет слов")
        }
        pigged_string.push_str(&pigged_word);
    } // конец цикла
    println!("Ура! Получилось! Читайте: {}", pigged_string);
}
