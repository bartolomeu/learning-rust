fn main() {
    // stack     pilha       last in -> first out
        // ocupa um espaço fixo
        // mais rápido acesso
        // dados tamanho específico

    // heap      fila        first in -> first out
        // ocupa mais espaço
        // mais lento
        // permite diferente tamanhos de dado e tamanhos desconhecidos

    exercise_01();
}

fn exercise_01(){
    let st = String::from("size fixed string");

    let mut st1 = String::from("not fixed string");
    st1.push_str("...");
    println!("{}", st);
    println!("{}", st1);
}