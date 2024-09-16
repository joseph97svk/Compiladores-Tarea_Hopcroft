use std::collections::{HashMap, HashSet, LinkedList};
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

        let alphabet : Vec<String> = line
            .trim()
            .split(' ')
            .map(|s| {
                s.to_string()
            })
            .collect();

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
                    let parameters: Vec<String>
                        = line.trim().split(' ')
                        .map(|s| {
                            s.to_string()
                        })
                        .collect();
                    let node_name : &String = &parameters[0];
                    let edge: &String = &parameters[1];
                    let destination: &String = &parameters[2];

                    self.graph.add_edge(self.nodes_mapping[node_name]
                                        , self.nodes_mapping[destination]
                                        , edge.to_string());
                }
                Err(error) => {
                    println!("Error \"{}\" reading file on line: {}", error, line_number);
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

    pub fn create_from_sets(
        sets: &LinkedList<HashSet<NodeIndex>>
        , automaton: &Automaton
    ) -> Automaton {
        let mut minimized_automaton: Automaton = Automaton::new();
        let mut set_to_state: HashMap<String, HashSet<NodeIndex>> = HashMap::new();

        add_states_to_automaton(automaton, sets, &mut set_to_state, &mut minimized_automaton);

        minimized_automaton.start_state = find_start_state(automaton, sets, &set_to_state);
        minimized_automaton.alphabet = automaton.alphabet.clone();

        reconstruct_transitions(automaton, &set_to_state, &mut minimized_automaton);

        minimized_automaton
    }
}
fn add_states_to_automaton(
    automaton: &Automaton,
    sets: &LinkedList<HashSet<NodeIndex>>,
    set_to_state: &mut HashMap<String, HashSet<NodeIndex>>,
    minimized_automaton: &mut Automaton
) {
    let mut state_counter: i32 = 0;

    for set in sets.iter().rev() {
        let new_state_name: String = format!("q{}", state_counter);
        set_to_state.insert(new_state_name.clone(), set.clone());

        if set.iter().any(|&node| {
            automaton.accepting_states.contains(&automaton.graph[node])
        }) {
            minimized_automaton.accepting_states.push(new_state_name.clone());
        }

        let new_state_index = minimized_automaton.graph.add_node(new_state_name.clone());
        minimized_automaton.nodes_mapping.insert(new_state_name.clone(), new_state_index);
        minimized_automaton.states.push(new_state_name);

        state_counter += 1;
    }
}

fn find_start_state(
    automaton: &Automaton,
    sets: &LinkedList<HashSet<NodeIndex>>,
    set_to_state: &HashMap<String, HashSet<NodeIndex>>
) -> String {
    if let Some(start_state_node)= automaton.nodes_mapping.get(&automaton.start_state) {
        let start_state_set
            = sets.iter().find(|set| {
            set.contains(start_state_node)
        });

        if let Some(start_set) = start_state_set {
            let start_state_name = set_to_state.iter()
                .find(|(_, subset)| {
                    subset == &start_set
                })
                .map(|(state_name, _)| {
                    state_name.clone()
                })
                .unwrap();

            return start_state_name;
        }
    }

    "None".to_string()
}

fn reconstruct_transitions(
    automaton: &Automaton,
    set_to_state: &HashMap<String, HashSet<NodeIndex>>,
    minimized_automaton: &mut Automaton
) {
    let mut added_transitions: HashSet<(String, String, String)> = HashSet::new();

    for (new_state_name, set) in set_to_state.iter() {
        let new_state_index: NodeIndex = minimized_automaton.nodes_mapping[new_state_name];

        let mut transition_map: HashMap<String, String> = HashMap::new();

        for &node in set {
            for edge in automaton.graph.edges(node) {
                let edge_weight = edge.weight().clone();
                let target_node = edge.target();

                let target_set_name = set_to_state.iter()
                    .find(|(_, subset)| {
                        subset.contains(&target_node)
                    })
                    .map(|(state_name, _)| {
                        state_name.clone()
                    })
                    .unwrap();

                if !transition_map.contains_key(&edge_weight) {
                    transition_map.insert(edge_weight.clone(), target_set_name);
                } else {
                    if transition_map[&edge_weight] == *new_state_name
                        && target_set_name != *new_state_name {
                        transition_map.insert(edge_weight.clone(), target_set_name);
                    }
                }
            }
        }

        for (edge_weight, target_set_name) in transition_map {
            let transition
                = (new_state_name.clone(), edge_weight.clone(), target_set_name.clone());

            if !added_transitions.contains(&transition) {
                let target_index
                    = minimized_automaton.nodes_mapping[&target_set_name];
                minimized_automaton.graph.add_edge(
                    new_state_index
                    , target_index
                    , edge_weight.clone());
                added_transitions.insert(transition);
            }
        }
    }
}









