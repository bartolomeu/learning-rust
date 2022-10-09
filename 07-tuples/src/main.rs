fn main() {
    let tuple : (i32, f64, &str) = (500, 6.4, "hey?!?"); //why &str ?!?
    let (v1, v2, v3) = tuple;

    let x0 = tuple.0 ;
    let x1 = tuple.1 ;
    let x2 = tuple.2 ;

    println!("v1 => {v1} or {x0}"); // why "{tuple.0}" doesn't work ?!?
    println!("v2 => {v2} or {x1}"); // why "{tuple.1}" doesn't work ?!?
    println!("v3 => {v3} or {x2}"); // why "{tuple.2}" doesn't work ?!?
}
