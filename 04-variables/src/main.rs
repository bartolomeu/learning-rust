fn main() {

    exercise01();
    println!("---------------");
    
    exercise02();
    println!("---------------");

    
    
}

fn exercise01 (){
    let mut x = 5;
    println!("The value is {x}");
    x = 6;
    println!("The value is {x}");
}

fn exercise02(){
    let x =5;
    let x = x+1;
    println!("The value is {x}");
    {
        let x = x*2;
        println!("The value is {x}");
    }
    println!("The value is {x}");
}