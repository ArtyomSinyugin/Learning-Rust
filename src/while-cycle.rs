// задача - вывести через цикл только повторяющиеся в массиве элементы. Сложность в повторяющихся более двух раз элементах массива

fn main() {

    let array: [i32;15] = [1, 2, 2, 2, 3, 4, 5, 5,5,5, 6, 7, 8, 5, 2]; 

    let mut i: usize = 0;
    let mut check = [0; 10];
    while i < array.len() {
        let mut j: usize = i + 1;        
        while j < array.len() {
            if check.contains(&array[i]) {
                break
            } else {
                if array [i] == array[j] {
                    println!("В массиве array число {} повторяется несколько раз", array[i]);
                    check[i] = array[i];
                    break
                }
            }
            j += 1
        }
        i += 1
    }
}
