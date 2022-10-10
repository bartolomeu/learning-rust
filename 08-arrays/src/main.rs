use std::io;

fn main() {
    let _arr1 = [3;5]; // == [3, 3, 3, 3, 3]
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Enter array index");

    let mut index = String::new();
    
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index '{index}' is not a number");

    let element = a[index];
    println!("value of index {index} is : {element}")

}
