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

    let non_final_states: HashSet<NodeIndex>
        = all_nodes.difference(&final_states).cloned().collect();

    let mut sets: LinkedList<HashSet<NodeIndex>> = LinkedList::new();
    sets.push_front(non_final_states);
    sets.push_back(final_states);

    let mut worklist = sets.clone();

    while let Some(set) = worklist.pop_front() {
        for symbol in &automaton.alphabet {
            let (set1, set2_option)
                = split(&set, automaton, symbol);

            if let Some(set2) = set2_option {
                // Remove the original set from 'sets' after processing it
                // Tried putting it separate method, just messed up order no matter what
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

    Automaton::create_from_sets(&sets, automaton)
}

fn split(set: &HashSet<NodeIndex>, automaton: &Automaton, symbol: &String) -> (HashSet<NodeIndex>, Option<HashSet<NodeIndex>>) {
    let mut set1: HashSet<NodeIndex> = HashSet::new();
    let mut set2: HashSet<NodeIndex> = HashSet::new();

    let transition_map = differentiate_set(set, automaton, symbol);

    if transition_map.len() == 1 {
        return (set.clone(), None);
    }

    let mut first = true;
    for group in transition_map.values() {
        if first {
            set1.extend(group.iter().cloned());
            first = false;
        } else {
            set2.extend(group.iter().cloned());
        }
    }

    (set1, Some(set2))
}

fn differentiate_set(set: &HashSet<NodeIndex>, automaton: &Automaton, symbol: &String) -> HashMap<Option<NodeIndex>, HashSet<NodeIndex>> {
    let mut transition_map: HashMap<Option<NodeIndex>, HashSet<NodeIndex>> = HashMap::new();

    for &node in set {
        let mut found_transition: Option<NodeIndex> = None;

        for edge in automaton.graph.edges(node) {
            if edge.weight() == symbol {
                found_transition = Some(edge.target());
                break;
            }
        }

        transition_map.entry(found_transition)
            .or_insert_with(HashSet::new)
            .insert(node);
    }

    transition_map
}
