fn main() {
    // stack     pilha       last in -> first out
        // ocupa um espaço fixo
        // mais rápido acesso
        // dados tamanho específico

    // heap      fila        first in -> first out
        // ocupa mais espaço
        // mais lento
        // permite diferente tamanhos de dado e tamanhos desconhecidos

    if false {
        exercise_01();
    }
    
    exercise_02();
}

fn exercise_01(){
    let st = String::from("size fixed string");

    let mut st1 = String::from("not fixed string");
    st1.push_str("...");
    println!("{}", st);
    println!("{}", st1);
}

fn exercise_02(){

    //two different variables both with the same value 5
    //because integer are simple values,
    //with a know, fixed size and those "5" values are pushed on stack
    let i1 = 5;
    let i2 = i1;

    println!("i1 => {i1}");
    println!("i2 => {i2}");

    // String has 3 parts (in stack)
    //      a pointer to the memory that holds the content of the string "hi"
    //      length of the string (how much bytes the content of the string "hi" is currently using, in this case 2)
    //      capacity (how much bytes the allocator gave to the string)

    // the content of the string is in a heap (not fixed size)

    // String is a complex data, so like javascript each variable has its own pointer with length and capacity
    // but the pointers goes to the same memory allocated with the content "hi"
    let s1 = String::from("hi");
    let s2 = s1; //looks like a shallow copy, but it's a move (Rust invalidate variable s1)
    
    // when the scope ends or the function "drop" is called
    // all memory on the stack is "cleaned", but when you have => var1 = var2
    // the program will try to clean twice the same "area allocated" (also know as "double free error")
    // the 1st will free the memory *(like it should)
    // the 2nd will try to free the same memory area can lead to memory corruption and this could lead to security vulnerability 

    // println!("s1 => {s1}"); //this line gives an error "value borrowed here after move"
    println!("s2 => {s2}");

    let s3 = s2.clone(); //may be expensive (deep copy)
    println!("s3 => {s3}"); //works fine because the data was deeply copied, not moved like s1 was to s2
}