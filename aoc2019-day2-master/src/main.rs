use std::
        {fs::File, io::Read};
use day2::calculate;
  
fn main() {
    let mut integers = String::new();
    let _f = File::open("input.txt")
        .unwrap()
        .read_to_string(&mut integers);

    let answer = calculate(integers);
    println!("{}", answer);
    println!("Завершено");
}

