use std::collections::HashMap;
use petgraph::graph::{DiGraph, NodeIndex};

use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Error, ErrorKind, Write};
use petgraph::prelude::EdgeRef;

pub type Graph = DiGraph<String, String>;

pub struct Automaton {
    pub graph: Graph,
    pub states : Vec<String>,
    pub alphabet: Vec<String>,
    pub nodes_mapping: HashMap<String, NodeIndex>,
    pub start_state: String,
    pub accepting_states: Vec<String>,
}

impl Automaton {
    pub fn new() -> Automaton {
        Automaton {
            graph: Default::default(),
            states: vec![],
            alphabet: vec![],
            nodes_mapping: HashMap::new(),
            start_state: String::new(),
            accepting_states: vec![],
        }
    }

    pub fn populate_from_file(&mut self, file_name: &String) -> Result<(), Error> {
        println!("Reading file: {}\n", file_name);

        let file = match File::open(file_name) {
            Ok(file) => { file },
            Err(why) => {
                return Err(why)
            },
        };

        let mut reader = BufReader::new(file);

        match self.read_definition(&mut reader) {
            Ok(_) => {}
            Err(_) => {
                return Err(Error::new(ErrorKind::Other, "unable to read definition"));
            }
        }
        match self.read_nodes(&mut reader) {
            Ok(_) => {}
            Err(_) => {
                return Err(Error::new(ErrorKind::Other, "unable to read definition"));
            }
        }

        Ok(())
    }

    fn read_definition(&mut self, reader: &mut BufReader<File>) -> Result<(), Error> {
        let mut line: String = String::new();
        let mut _lines_read = match reader.read_line(&mut line) {
            Ok(n) => n,
            Err(why) => return Err(why),
        };

        let alphabet : Vec<String> = line.trim().split(' ').map(|s| s.to_string()).collect();

        println!("alphabet: {:?}", alphabet);

        self.alphabet = alphabet;
        line.clear();

        _lines_read = match reader.read_line(&mut line) {
            Ok(n) => n,
            Err(why) => return Err(why),
        };

        self.states = line.trim().split(' ').map(|s| s.to_string()).collect();
        line.clear();

        println!("states: {:?}", self.states);

        for state in &self.states {
            let node_index = self.graph.add_node(state.clone());
            println!("Adding state: {}", state);
            self.nodes_mapping.insert(state.clone(), node_index);
        }

        _lines_read = match reader.read_line(&mut line) {
            Ok(n) => n,
            Err(why) => return Err(why),
        };

        self.start_state = line.clone().trim().to_string();
        line.clear();

        _lines_read = match reader.read_line(&mut line) {
            Ok(n) => n,
            Err(why) => return Err(why),
        };

        self.accepting_states = line.trim().split(' ').map(|s| s.to_string()).collect();

        Ok(())
    }

    fn read_nodes(&mut self, reader : &mut BufReader<File>) -> Result<(), Error>  {
        let mut line_number : usize = 0;

        for line in reader.lines() {
            line_number += 1;
            match line {
                Ok(line) => {
                    let parameters: Vec<String> = line.trim().split(' ').map(|s| s.to_string()).collect();
                    let node_name : &String = &parameters[0];
                    let edge: &String = &parameters[1];
                    let destination: &String = &parameters[2];

                    self.graph.add_edge(self.nodes_mapping[node_name]
                                        , self.nodes_mapping[destination]
                                        , edge.to_string());
                }
                Err(_) => {
                    println!("Error reading file on line: {}", line_number);
                }
            };
        }

        Ok(())
    }

    pub fn write_on_file(self, file_name: &String) -> Result<(), Error> {
        println!("\nWriting results on file: {}", file_name);

        let file = File::create(file_name)?;
        let mut writer = BufWriter::new(file);

        let alphabet_line = self.alphabet.join(" ") + "\n";

        writer.write(alphabet_line.as_bytes())?;

        let states_line = self.states.join(" ") + "\n";

        writer.write(states_line.as_bytes())?;
        writer.write((self.start_state.clone() + "\n").as_bytes())?;

        let accepting_states_line = self.accepting_states.join(" ") + "\n";
        writer.write(accepting_states_line.as_bytes())?;

        for node_index in self.graph.node_indices() {
            let node : &String = &self.graph[node_index];

            for edge in self.graph.edges(node_index) {
                let destination : &String = &self.graph[edge.target()];
                let edge_val : &String = edge.weight();

                let line: String = node.to_string() + " " + edge_val + " " + destination + "\n";

                match writer.write(line.as_ref()) {
                    Ok(_) => {}
                    Err(why) => return Err(why),
                }
            }
        }

        Ok(())
    }
}