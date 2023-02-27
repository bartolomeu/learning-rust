fn main() {
    if false {
        exercise01();
    }
    exercise02();
}

fn exercise01 () {
    let s1 = String::from("hi");
    let len = calculate_length(&s1); // s1 was passed 

    println!("s1 => {s1} // len => {len}");

    fn calculate_length (str : &String) -> usize {
        str.len()
    }
}

fn exercise02(){
    let mut str = String::from("hi"); //var str is Mutable

    // this line will make the next line break, can't use a mutable refence and then any reference & to this value
    // let something = &mut str; 

    change_str(&mut str); //pass str as reference, so I can reuse it at next line and set it as mutable to fnc change_str can change it...
    println!("str => {str}");
    
    //use this var after another refence to the same value breaks the code, but if we use it before que compiler will clean its reference and won't break
    // println!("something => {something}");


    fn change_str (str1: &mut String){
        str1.push_str(", hello world !!");
    }
}