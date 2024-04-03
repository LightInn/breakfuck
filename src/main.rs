const HELLO: &str = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
const MEMORY_SIZE: usize = 30000;

// Brainfuck interpreter
fn main() {


    // new memory array of memory size

    let mut memory = [0u8; MEMORY_SIZE]; // Nouveau tableau de mémoire

    // let instruction pointer
    let mut pi = 0; // Pointeur d'instruction


    // let memory cell pointer
    let mut pm = 0; // Pointeur de cellule mémoire


    // let last stack index array
    let mut stack_array = [0u8; MEMORY_SIZE];


    // let output
    let mut output = String::new(); // Sortie


    // output = String from char code


    // tant qu'on a pas finir de lire les instruction
    while pi < HELLO.len() {

        match HELLO.chars().nth(pi).unwrap() {
            '>' =>
        }

    }





    println!("Hello, world!");
}


fn interpret() {
    let end: bool = false;

    while !end {

        match  {  }



    }
}