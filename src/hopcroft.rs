use crate::automaton::Automaton;
use std::collections::{HashMap, HashSet, LinkedList};
use petgraph::graph::{NodeIndex};
use petgraph::visit::EdgeRef;

pub fn hopcroft_reduction(automaton: &Automaton) -> Automaton {
    let all_nodes: HashSet<NodeIndex> = automaton.graph.node_indices().collect();

    let final_states: HashSet<NodeIndex> = all_nodes.iter()
        .filter(|&&node_index| {
            let node_value = &automaton.graph[node_index];
            automaton.accepting_states.contains(node_value)
        }).cloned().collect();

    let non_final_states: HashSet<NodeIndex> = all_nodes.difference(&final_states).cloned().collect();

    let mut sets: LinkedList<HashSet<NodeIndex>> = LinkedList::new();
    sets.push_front(non_final_states);
    sets.push_back(final_states);

    let mut worklist = sets.clone();

    while let Some(set) = worklist.pop_front() {
        for symbol in &automaton.alphabet {
            let (set1, set2_option) = split(&set, automaton, symbol);

            if let Some(set2) = set2_option {
                // Remove the original set from 'sets' after processing it
                let mut new_sets = LinkedList::new();
                while let Some(current_set) = sets.pop_front() {
                    if current_set != set {
                        new_sets.push_back(current_set);
                    }
                }
                sets = new_sets;

                if !sets.contains(&set1) {
                    sets.push_front(set1.clone());
                    worklist.push_back(set1);
                }

                if !sets.contains(&set2) {
                    sets.push_front(set2.clone());
                    worklist.push_back(set2);
                }
            }
        }
    }

    rebuild_the_automaton(&sets, automaton)
}

fn split(set: &HashSet<NodeIndex>, automaton: &Automaton, symbol: &String) -> (HashSet<NodeIndex>, Option<HashSet<NodeIndex>>) {
    let mut set1: HashSet<NodeIndex> = HashSet::new();
    let mut set2: HashSet<NodeIndex> = HashSet::new();

    let mut transition_map: HashMap<Option<NodeIndex>, HashSet<NodeIndex>> = HashMap::new();

    for &node in set {
        let mut found_transition = None;

        for edge in automaton.graph.edges(node) {
            if edge.weight() == symbol {
                found_transition = Some(edge.target());
                break;
            }
        }

        transition_map
            .entry(found_transition)
            .or_insert_with(HashSet::new)
            .insert(node);
    }

    if transition_map.len() == 1 {
        return (set.clone(), None);
    }

    // Split states based on different transition targets
    let mut first = true;
    for group in transition_map.values() {
        if first {
            set1.extend(group.iter().cloned());
            first = false;
        } else {
            set2.extend(group.iter().cloned());
        }
    }

    if set2.is_empty() {
        return (set1, None);
    }

    (set1, Some(set2))
}

fn rebuild_the_automaton(sets: &LinkedList<HashSet<NodeIndex>>, automaton: &Automaton) -> Automaton {
    let mut minimized_automaton = Automaton::new();
    let mut state_counter = 0;
    let mut set_to_state: HashMap<String, HashSet<NodeIndex>> = HashMap::new();

    // Step 1: Add states and map them to minimized automaton
    for set in sets.iter() {
        let new_state_name = format!("q{}", state_counter);
        set_to_state.insert(new_state_name.clone(), set.clone());

        if set.iter().any(|&node| automaton.accepting_states.contains(&automaton.graph[node])) {
            minimized_automaton.accepting_states.push(new_state_name.clone());
        }

        let new_state_index = minimized_automaton.graph.add_node(new_state_name.clone());
        minimized_automaton.nodes_mapping.insert(new_state_name.clone(), new_state_index);
        minimized_automaton.states.push(new_state_name);

        state_counter += 1;
    }

    // Step 2: Set the start state
    if let Some(start_state_node) = automaton.nodes_mapping.get(&automaton.start_state) {
        let start_state_set = sets.iter().find(|set| set.contains(start_state_node));

        if let Some(start_set) = start_state_set {
            let start_state_name = set_to_state.iter()
                .find(|(_, subset)| subset == &start_set)
                .map(|(state_name, _)| state_name.clone())
                .unwrap();

            minimized_automaton.start_state = start_state_name;
        }
    } else {
        println!("Error: Start state '{}' not found in nodes_mapping", automaton.start_state);
    }

    minimized_automaton.alphabet = automaton.alphabet.clone();

    // Step 3: Rebuild transitions based on minimized sets
    let mut added_transitions: HashSet<(String, String, String)> = HashSet::new(); // To avoid duplicate transitions

    for (new_state_name, set) in set_to_state.iter() {
        let new_state_index = minimized_automaton.nodes_mapping[new_state_name];

        for &node in set.iter() {
            for edge in automaton.graph.edges(node) {
                let target_node = edge.target();
                let edge_weight = edge.weight();

                let target_set_name = set_to_state.iter()
                    .find(|(_, subset)| subset.contains(&target_node))
                    .map(|(state_name, _)| state_name)
                    .unwrap();

                // Avoid adding duplicate transitions
                let transition = (new_state_name.clone(), edge_weight.clone(), target_set_name.clone());
                if !added_transitions.contains(&transition) {
                    let target_index = minimized_automaton.nodes_mapping[target_set_name];
                    minimized_automaton.graph.add_edge(new_state_index, target_index, edge_weight.clone());
                    added_transitions.insert(transition);
                }
            }
        }
    }

    minimized_automaton
}

