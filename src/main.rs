const HELLO: &str = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
const MEMORY_SIZE: usize = 10;

// Brainfuck interpreter
fn main() {


    // new memory array of memory size

    let mut memory = [0u8; MEMORY_SIZE]; // Nouveau tableau de mémoire

    // let instruction pointer
    let mut pi = 0; // Pointeur d'instruction


    // let memory cell pointer
    let mut pm = 0; // Pointeur de cellule mémoire


    // let last stack index array
    let _stack_array = [0u8; MEMORY_SIZE];


    // let output
    let mut output = String::new(); // Sortie


    // output = String from char code


    // tant qu'on a pas finir de lire les instruction
    while pi < HELLO.len() {
        match HELLO.chars().nth(pi).unwrap() {
            '>' => pm += 1,
            '<' => pm -= 1,
            '+' => memory[pm] += 1,
            '-' => memory[pm] -= 1,
            '.' => output.push(memory[pm] as char),
            ',' => memory[pm] = get_input(),
            '[' => {
                if memory[pm] == 0 {
                    // Sauter à l'instruction après le ]
                    let mut loop_counter = 1;
                    while loop_counter > 0 {
                        pi += 1;
                        match HELLO.chars().nth(pi).unwrap() {
                            '[' => loop_counter += 1,
                            ']' => loop_counter -= 1,
                            _ => {}
                        }
                    }
                }
            }
            ']' => {
                if memory[pm] != 0 {
                    // Sauter en arrière à l'instruction après le [
                    let mut loop_counter = 1;
                    while loop_counter > 0 {
                        pi -= 1;
                        match HELLO.chars().nth(pi).unwrap() {
                            ']' => loop_counter += 1,
                            '[' => loop_counter -= 1,
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
        pi += 1;
    }


    send_output(output);
}

fn send_output(output: String) {
    println!("{}", output)
}


fn get_input() -> u8 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.as_bytes()[0]
}