mod automaton;
mod hopcroft;

use std::env;

use crate::automaton::Automaton;
use crate::hopcroft::hopcroft_reduction;

static DEFAULT_READ_FILENAME: &str = "nodes.txt";
static DEFAULT_WRITE_FILENAME: &str = "results.txt";

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut read_file_name = DEFAULT_READ_FILENAME.to_string();
    let mut save_file_name = DEFAULT_WRITE_FILENAME.to_string();

    if args.len() > 1 {
        read_file_name = args[1].clone().to_string();
    }

    if args.len() > 2 {
        save_file_name = args[2].clone().to_string();
    }

    let mut dfa : Automaton = Automaton::new();

    match dfa.populate_from_file(&read_file_name) {
        Ok(_) => {},
        Err(error) => {
            println!("Error while populating DFA from file: {}", error);
        }
    }

    let minimized_dfa = hopcroft_reduction(&dfa);

    match minimized_dfa.write_on_file(&save_file_name) {
        Ok(_) => {},
        Err(error) => {
            println!("Error while writing DFA to file: {}", error);
        }
    }
}
