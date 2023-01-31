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

/* use std::io; // !!!Первое решение задачи
fn main() {
    
    let mut sentence = String::new ();
    let massage = String::from("Напишите своё предложение на английском:");
    println!("{massage}");

    io::stdin().read_line(&mut sentence).expect("Ошибка ввода, аварийное завершение");

    let sentence = res_ult(&mut sentence);

    println!("Pig latin: \n{sentence}");
    
    fn res_ult (white: &mut String) -> String {

        let mut res: String = String::new();  // сюда записываем финальный результат

        for word in white.split_whitespace() {   // начинаем первый цикл, делим предложение на слова
            let mut word_vector = Vec::new();
            for i in word.chars() { // начинаем второй цикл, слова переводим в вектор
                    word_vector.push(i)
                } // второй цикл
            //word_vector.push('h');
            match &mut word_vector[0] { // считывает первую букву через match, возвращает
                'a' | 'e' | 'i' | 'o' | 'u' | 'y' | 'A' | 'E' | 'I' | 'O' | 'U' | 'Y' => {
                    word_vector.push('h');word_vector.push('e');word_vector.push('y');word_vector.push(' ');     
                },
                _ => {
                    //let x = word_vector[0];
                    word_vector.push (word_vector[0]);word_vector.push ('e');word_vector.push ('y');word_vector.remove (0);word_vector.push(' ');      
                }
            } 
            let word_return = String::from_iter(word_vector);
            res.push_str(&word_return)
        }    // первый цикл   
        //println!("{res}");       
        res
    } // завершаем функцию
} */