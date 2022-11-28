fn main() {
    for num in (1..10).rev(){
        println!("{num} !");
    }
    println!("LIFTOFF");

    let varr = false;
    if varr {
        exercise01();
        exercise02();
        exercise03();
        exercise04();
    }
}

fn exercise01(){
    let mut counter =0;
    let result = loop {
        counter +=1;
        if counter == 10{
            break counter*3;
        }
    };
    println!("result is {result}");
}

fn exercise02(){
    let mut count =0;
    'counting: loop{
        println!("count =  {count}");
        let mut remaining = 10;

        loop {
            println!(" - remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count ==2 {
                break 'counting;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("end = {count}");
}

fn exercise03 (){
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -=1;
    }
    println!("LIFTOFF");
}

fn exercise04 (){
    let arr = [10,12,14,16,18,20];
    for elm in arr {
        println!("value of elm is {elm}");
    }
}