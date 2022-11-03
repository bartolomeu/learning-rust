use std::i32;

fn main() {
    if false {
        exercise01(5,7);
        exercise02();
        exercise03();
        exercise04();
    }

    exercise05();

}

fn exercise05 (){
    let num = 50;
    let fib = calculate_fibonacci(num);
    println!("fibonacci {num} => {fib}");
}

fn calculate_fibonacci(num: u32) -> u32{
    if (num == 1) ^ (num == 2) {
        return 1;
    }
    let fib1 = calculate_fibonacci(num -1);
    let fib2 = calculate_fibonacci(num -2);
    // println!("fib-1 => {fib1}");
    // println!("fib-2 => {fib2}");
    return fib1 + fib2;
}


fn exercise04(){
    let c : f32 = 20.0;
    let f = celsius_to_fahrenheit(c);

    println!("{c}C {f}F");

    let f1 = 20.0;
    let c1 = fahrenheit_to_celsius(f1);

    println!("{f1}F {c1}C");
}

fn celsius_to_fahrenheit(celsius:f32) -> f32{
    let fahrenheit = (celsius * (9.0/5.0)) + 32.0;
    return fahrenheit;
}

fn fahrenheit_to_celsius(fahrenheit:f32) -> f32{
    let celsius: f32 = (fahrenheit - 32.0) * 5.0/9.0;
    return celsius;
}

fn exercise03 () {
    let arr = (0..4).rev();
    for elm in arr {
        println!("{elm} !");
    }
    println!(" LIFTOFF !! ");
}

fn exercise02 () {
    let arr = [10, 20, 30, 40, 50];
    for elm in arr {
        println!("{elm}");
    }
}

fn exercise01 (counter: i32, break_remain : i32){
    let mut count = 0;

    'counting_loop: loop {
        println!("count => {count}");
        let mut remaining = 10;

        loop {
            println!("  remaining => {remaining}");
            if remaining == break_remain {
                break;
            }

            if count == counter {
                break 'counting_loop;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("end count => {count}")
}
