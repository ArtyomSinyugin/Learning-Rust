pub fn calculate (integers: String)-> i32 {
    let mut answer: i32 = 0;
    'outer: for noun in 0..100 {
               
        for verb in 0..100 {
            let mut input: Vec<i32> = integers
            .split(',')
            .flat_map(|i| i
                            .trim()
                            .parse()
                            )
            .collect();
            
            input[1] = noun;
            input[2] = verb;
            let mut i: usize = 0;
            while i <= input.len() {
                let (a, b, c) = (input[i+1] as usize, input[i+2] as usize, input[i+3] as usize);
                match input[i] {
                    1 => input[c] = input[a] + input [b],
                    2 => input[c] = input[a] * input [b],
                    99 => {               
                        break;
                    }
                    _ => {
                        println!("Ошибка программы");
                        println!("Noun is: {}, verb is: {}", input[1], input[2]);
                        println!("Program number is: {}", input[0]);
                    }
                }
                i = i + 4;
                if input[0] == 19690720 {
                    answer = 100 * input[1] + input[2];
                    break 'outer;
                }   
            }
        }
    }
    answer
}