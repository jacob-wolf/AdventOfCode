use advent_of_code_2025::{read_file, Part, Which};
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

pub fn p8(choice: Which, part: Part) {
    let file_data: String = read_file(8, choice, None);
    let now = std::time::SystemTime::now();
    match part {
        Part::One => part1(&file_data, choice),
        Part::Two => part2(&file_data),
    };
    match now.elapsed() {
        Ok(elapsed) => println!("Runtime: {} microseconds", elapsed.as_micros()),
        _ => panic!(),
    }
}

fn dist(p1: &Vec<usize>, p2: &Vec<usize>) -> usize {
    let dx = p2[0] as isize - p1[0] as isize;
    let dy = p2[1] as isize - p1[1] as isize;
    let dz = p2[2] as isize - p1[2] as isize;

    return { dx * dx + dy * dy + dz * dz } as usize;
}

fn part1(data: &str, choice: Which) {
    let coords = data
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num_str| num_str.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    // for each box create a connection to its nearest neighbor
    // store a set of edges shortest edges for each node
    let mut edges: Vec<(usize, usize, usize)> = vec![];

    for idx in 0..coords.len() {
        for jdx in idx + 1..coords.len() {
            let d = dist(&coords[idx], &coords[jdx]);
            edges.push((idx, jdx, d));
        }
    }

    edges.sort_by(|a, b| a.2.cmp(&b.2));

    let mut circuits: Vec<HashSet<usize>> = vec![];
    let max_connections = match choice {
        Which::Test => 10,
        Which::Real => 1000,
    };
    let mut counter = 0;

    while counter < max_connections {
        // find the shortest path that's not taken
        // check every existing circuit if either end of  this edge is in the circuit if so add the other end else create new circuit
        let mut found_circuit = false;
        let edge = edges[counter];
        let left = edge.0;
        let right = edge.1;
        for (c_idx, circuit) in circuits.iter().enumerate() {
            if circuit.contains(&left) || circuit.contains(&right) {
                //println!("Found {left} or {right} in {circuit:?}");
                let mut new_circuit = circuit.clone();
                new_circuit.insert(left);
                new_circuit.insert(right);
                circuits[c_idx] = new_circuit;
                found_circuit = true;
                break;
            }
        }
        if !found_circuit {
            let mut new_circuit = HashSet::new();
            new_circuit.insert(left);
            new_circuit.insert(right);
            circuits.push(new_circuit);
        }

        counter += 1;
    }

    // need to combine overlaps
    let mut found_combined_pair = true;
    while found_combined_pair {
        found_combined_pair = false;
        for idx in 0..circuits.len() {
            let circ_1: &HashSet<usize> = &circuits[idx];
            for jdx in idx + 1..circuits.len() {
                let circ_2: &HashSet<usize> = &circuits[jdx];
                // check circuit idx and circuit jdx for overlap
                let mut combo = circ_1.clone();
                combo.extend(circ_2.clone());
                if circ_1.len() + circ_2.len() > combo.len() {
                    found_combined_pair = true;
                    // these sets are the same
                    circuits[idx] = combo;
                    circuits.remove(jdx);
                    break;
                }
            }
            if found_combined_pair {
                break;
            }
        }
    }

    let mut sizes = circuits
        .iter()
        .map(|circ| circ.len())
        .collect::<Vec<usize>>();
    sizes.sort();
    let res = sizes.iter().rev().take(3).product::<usize>();
    println!("P1: {res}");
}

#[derive(Clone, Debug, Hash)]
struct Node {
    label: usize,
    edges: Vec<Edge>,
}

#[derive(Clone, Debug, Hash)]
struct Edge {
    l: usize,
    r: usize,
    d: usize,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
    fn ne(&self, other: &Self) -> bool {
        self.label != other.label
    }
}

impl Eq for Node {}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        ({ self.l == other.l && self.r == other.r }) || ({ self.l == other.r && self.r == other.l })
    }
    fn ne(&self, other: &Self) -> bool {
        ({ self.l != other.l && self.l != other.r }) || ({ self.r != other.l && self.r != other.r })
    }
}

impl Eq for Edge {}

fn part2(data: &str) {
    // going to need a different strategy to scale better
    // still find all the shortest links
    let coords = data
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num_str| num_str.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    let mut nodes: HashMap<usize, Node> = HashMap::new();
    for i in 0..coords.len() {
        nodes.insert(
            i,
            Node {
                label: i,
                edges: vec![],
            },
        );
    }
    let mut edges: HashSet<Edge> = HashSet::new();

    for idx in 0..coords.len() {
        for jdx in idx + 1..coords.len() {
            let d = dist(&coords[idx], &coords[jdx]);
            edges.insert(Edge {
                d: d,
                l: idx,
                r: jdx,
            });
        }
    }

    let mut edges_list = edges.into_iter().collect::<Vec<Edge>>();

    edges_list.sort_by(|a, b| a.d.cmp(&b.d));

    let mut added_edges: HashSet<Edge> = HashSet::new();
    // need to add edges to the graph 1 by 1 until all nodes are linked
    // add an edge then bfs the graph
    let mut final_edge: Option<Edge> = None;

    for edge in edges_list {
        // add the edge and update the node's edge lists
        added_edges.insert(edge.clone());

        let l = edge.l;
        let mut l_node = nodes.get(&l).unwrap().clone();
        let mut new_l_edges = l_node.edges;
        new_l_edges.push(edge.clone());
        l_node.edges = new_l_edges;
        nodes.insert(l, l_node);

        let r = edge.r;
        let mut r_node = nodes.get(&r).unwrap().clone();
        let mut new_r_edges = r_node.edges;
        new_r_edges.push(edge.clone());
        r_node.edges = new_r_edges;
        nodes.insert(r, r_node.clone());

        // check the bfs of the graph now to determine if we've got all the nodes
        let mut nodes_to_check: VecDeque<usize> = VecDeque::new();
        let mut visited: HashSet<usize> = HashSet::new();
        nodes_to_check.push_back(r);
        while !nodes_to_check.is_empty() {
            let curr_node_idx = nodes_to_check.pop_front().unwrap();
            if visited.contains(&curr_node_idx) {
                continue;
            }
            visited.insert(curr_node_idx);

            let curr_node = nodes.get(&curr_node_idx).unwrap();

            for edge in curr_node.edges.clone() {
                // check which end of the edge to add
                let node_to_add = if curr_node.label == edge.l {
                    edge.r
                } else {
                    edge.l
                };
                // add the node to the list of nodes to check
                nodes_to_check.push_back(node_to_add);
            }
        }
        if visited.len() == coords.len() {
            final_edge = Some(edge.clone());
            break;
        }
    }

    // now multiply x coords of final edge to get res
    if let Some(edge) = final_edge {
        let res = coords[edge.l][0] * coords[edge.r][0];
        println!("P2: {res}");
    } else {
        panic!("No Final Edge!")
    }
}
