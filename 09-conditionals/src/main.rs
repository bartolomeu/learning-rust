fn main() {
    // exercise01(5,7);
    // exercise02();
    // exercise03();
    exercise04();

}


fn exercise04(){
    let f = 1;
    let c = celsius_to_fahrenheit(f);

    println!("{f}F {c}C");
}

fn celsius_to_fahrenheit(celsius:i32) -> i32{
    let fahrenheit = (celsius * 9/5) + 32;
    return fahrenheit;
}

// fn fahrenheit_to_celsius(celsius:i32) -> i32{
// }

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
