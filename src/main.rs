mod automaton;
mod hopcroft;

use std::env;

use crate::automaton::Automaton;
use crate::hopcroft::hopcroft_reduction;

static DEFAULT_READ_FILENAME: &str = "nodes.txt";
static DEFAULT_WRITE_FILENAME: &str = "results.txt";

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_name: String = if args.len() < 2 {
        DEFAULT_READ_FILENAME.to_string()
    } else {
        args[1].clone()
    };

    let mut dfa : Automaton = Automaton::new();

    match dfa.populate_from_file(&file_name) {
        Ok(_) => {},
        Err(error) => {
            println!("Error while populating DFA from file: {}", error);
        }
    }

    let minimized_dfa = hopcroft_reduction(&dfa);

    match minimized_dfa.write_on_file(&DEFAULT_WRITE_FILENAME.to_string()) {
        Ok(_) => {},
        Err(error) => {
            println!("Error while writing DFA to file: {}", error);
        }
    }
}
