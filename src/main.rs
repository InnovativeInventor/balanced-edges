use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;
use rayon::prelude::*;
use serde_json::Value;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use rand::prelude::*;
use structopt::StructOpt;

// #[derive(Debug, Clone, Copy)]
type Index = usize;
type Pop = usize;

fn parse_dual_graph(filename: &str, pop_col: &str) -> (StableGraph<Pop, ()>, f64) {
    let dual_graph_unparsed = fs::read_to_string(filename).expect("Error loading in file");
    let dual_graph_val: Value = serde_json::from_str(&dual_graph_unparsed).unwrap();
    let dual_graph_parsed = dual_graph_val.as_object().unwrap();

    let mut dual_graph = StableGraph::<Pop, ()>::new();
    let nodes = dual_graph_parsed["nodes"].as_array().unwrap();

    // Parse nodes
    let mut node_pop_map = HashMap::<Index, Pop>::new();
    for node in nodes {
        let node_id: Index = serde_json::from_value(node["id"].clone()).unwrap();
        let population: Pop = serde_json::from_value(node[pop_col].clone()).unwrap();
        node_pop_map.insert(node_id, population);
    }

    // node_pop_map could have gaps in the indices
    for _ in 0..*node_pop_map.keys().max().unwrap() + 1 {
        dual_graph.add_node(0);
    }

    let mut total_pop = 0.0;
    for (node, population) in node_pop_map.iter() {
        dual_graph[NodeIndex::new(*node)] = *population;
        total_pop += *population as f64;
    }

    /*
    for node in dual_graph.clone().node_indices() {
        if dual_graph[node] == usize::MAX {
            println!("node removed: {:?}", node);
            dual_graph.remove_node(node);
        }
    }
    */

    let edges = dual_graph_parsed["adjacency"].as_array().unwrap();
    for (node, edges_per_node) in edges.iter().enumerate() {
        for edge in edges_per_node.as_array().unwrap() {
            dual_graph.add_edge(
                NodeIndex::new(node),
                NodeIndex::new(serde_json::from_value(edge["id"].clone()).unwrap()),
                (),
            );
        }
    }

    (dual_graph, total_pop)
}

fn dfs_paths_rec(
    graph: &StableGraph<Pop, ()>,
    node: NodeIndex,
    node_pop: Pop,
    path: &mut HashSet<NodeIndex>,
    max_pop: Pop,
) -> usize {
    let mut max_path_len = 0;
    if max_pop > node_pop {
        max_path_len = path.len();
        let max_pop = max_pop - node_pop;
        for neighbor in graph.neighbors(node) {
            if !path.contains(&neighbor) {
                let neighbor_pop = *graph.node_weight(neighbor).unwrap();
                if max_pop >= neighbor_pop {
                    path.insert(neighbor);
                    let length = dfs_paths_rec(graph, neighbor, neighbor_pop, path, max_pop);
                    max_path_len = std::cmp::max(max_path_len, length);
                    path.remove(&neighbor);
                }
            }
        }
    } else if max_pop == node_pop {
        max_path_len = path.len(); 
    } else {
        if path.len() != 0 {
            panic!("path len {}, with max pop {}, and node pop {}", path.len(), max_pop, node_pop) // should not happen
        }
    }
    max_path_len
}

fn dfs_paths(graph: &StableGraph<Pop, ()>, node: NodeIndex, max_pop: Pop) -> usize {
    let length = dfs_paths_rec(graph, node, graph[node], &mut HashSet::with_capacity(40), max_pop);
    println!("Max length for node {:?}: {}", node, length);
    length
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    name = "balanced-edges",
    about = "Calculator to determine an upper bound for M for reversible ReCom"
)]
struct Opt {
    #[structopt(short = "f", long = "file", help = "The filename to read the dual graph from")]
    filename: String,

    #[structopt(
        short = "d",
        long = "districts",
        help = "The number of districts"
    )]
    districts: usize,

    #[structopt(
        short = "t",
        long = "tol",
        help = "The population tolerance"
    )]
    tolerance: f64,
}


fn main() {
    let opt = Opt::from_args();
    let (dual_graph, total_pop) = parse_dual_graph(&opt.filename, "TOTPOP20");
    let ideal_pop = total_pop as f64 / opt.districts as f64;

    let length = dual_graph.node_indices().count();
    // let max_path_length = dfs_paths(&dual_graph, NodeIndex::new(5258), (ideal_pop * 0.02) as usize);
    // let mut rng = rand::thread_rng();
    let bar = ProgressBar::new(length as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {wide_bar:cyan/blue} {pos:>7}/{len:7} {eta}")
            .progress_chars("##-"),
    );
    let max_path_length = dual_graph
        .node_indices()
        .collect::<Vec<NodeIndex>>()
        // .choose_multiple(&mut rng, 10)
        // .copied()
        // .collect::<Vec<NodeIndex>>()
        .par_iter()
        .progress_with(bar)
        .map(|node| dfs_paths(&dual_graph, *node, (ideal_pop * opt.tolerance) as usize))
        .max()
        .unwrap();

    println!("Final max path length: {}", max_path_length);
}
