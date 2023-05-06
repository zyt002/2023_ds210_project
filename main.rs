pub mod read_txt_to_graph;
pub mod centrality_functions;
use read_txt_to_graph::read_txt_to_graph::read_file;
use crate::centrality_functions::centrality_functions::{degree_centrality, eigenvector_centrality_normalized, 
    betweenness_centrality};
use petgraph::graph::NodeIndex;
use petgraph::{Undirected, Graph};

fn main() -> std::io::Result<()> {
    // read data and turn into a graph
    let d = read_file("cleaned1.txt");
    let data = d.unwrap();
    
    // degree_centrality
    let degree_centrality_data = degree_centrality(&data);
    let mut sorted_deg_centrality: Vec<(&NodeIndex, &f64)> = degree_centrality_data.iter().collect();
    sorted_deg_centrality.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    let max_degree_node = *sorted_deg_centrality[0].0;
    let max_degree_centrality_score = *sorted_deg_centrality[0].1;
    let max_deg_station_name = data.node_weight(max_degree_node).unwrap();
    println!("Station ID with highest degree centrality: {}", max_deg_station_name);
    println!("Max degree centrality: {}", max_degree_centrality_score);

    // eigenvector centrality
    let eigenvector_centrality_data = eigenvector_centrality_normalized
        (&data, 50, 0.005);
    let mut sorted_eig_centrality: Vec<(&NodeIndex, &f64)> = eigenvector_centrality_data.iter().collect();
    sorted_eig_centrality.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    let max_eig_node = *sorted_eig_centrality[0].0;
    let max_eig_score = *sorted_eig_centrality[0].1;
    let max_eig_station_name = data.node_weight(max_eig_node).unwrap();
    println!("Station ID with highest eigenvector centrality: {}", max_eig_station_name);
    println!("Max eigenvector centrality: {}", max_eig_score);
    
    // betweeness centrality
    let betweenness_centrality_data = betweenness_centrality(&data);
    let mut sorted_btw_centrality: Vec<(&NodeIndex, &f64)> = betweenness_centrality_data.iter().collect();
    sorted_btw_centrality.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    let max_btw_node = *sorted_btw_centrality[0].0;
    let max_btw_score = *sorted_btw_centrality[0].1;
    let max_btw_station_name = data.node_weight(max_btw_node).unwrap();
    println!("Station ID with highest betweeness centrality: {}", max_btw_station_name);
    println!("Max betweeness centrality: {}", max_btw_score);

    Ok(())
}

// a simple graph used to test the functions
fn generate_graph() -> Graph<String, f64, Undirected> {
    let mut graph = Graph::new_undirected();

    let a = graph.add_node("A".to_string());
    let b = graph.add_node("B".to_string());
    let c = graph.add_node("C".to_string());
    let d = graph.add_node("D".to_string());
    let e = graph.add_node("E".to_string());
    let f = graph.add_node("F".to_string());

    graph.add_edge(a, b, 5.0);
    graph.add_edge(b, c, 1.0);
    graph.add_edge(c, a, 6.0);
    graph.add_edge(d, a, 8.0);
    graph.add_edge(e, c, 10.0);
    graph.add_edge(f, a, 7.0);

    graph
}

#[test]
fn test_degree_centrality() {
    let graph = generate_graph();
    let degree_centrality_data = degree_centrality(&graph);
    let mut sorted_deg_centrality: Vec<(&NodeIndex, &f64)> = degree_centrality_data.iter().collect();
    sorted_deg_centrality.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    let max_degree_centrality_score = *sorted_deg_centrality[0].1;
    assert_eq!(max_degree_centrality_score, 0.8);
}

#[test]
fn test_eigenvector_centrality() {
    let graph = generate_graph();
    let eig_centrality_data = eigenvector_centrality_normalized(&graph, 50, 0.005);
    let mut sorted_eig_centrality: Vec<(&NodeIndex, &f64)> = eig_centrality_data.iter().collect();
    sorted_eig_centrality.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    let max_eig_centrality_score = *sorted_eig_centrality[0].1;
    assert_eq!(max_eig_centrality_score, 1.0);
}

#[test]
fn test_between_centrality() {
    let graph = generate_graph();
    let btw_centrality_data = betweenness_centrality(&graph);
    let mut sorted_btw_centrality: Vec<(&NodeIndex, &f64)> = btw_centrality_data.iter().collect();
    sorted_btw_centrality.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    let max_btw_centrality_score = *sorted_btw_centrality[0].1;
    assert_eq!(max_btw_centrality_score, 0.5319148936170213);
}
