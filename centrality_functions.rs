pub mod centrality_functions {
    use std::collections::HashMap;
    use petgraph::Undirected; //library for graph structure
    use petgraph::graph::{Graph, NodeIndex};
    use petgraph::visit::EdgeRef;

    pub fn degree_centrality(graph: &Graph<String, f64, Undirected>) -> HashMap<NodeIndex, f64> {
        let mut degree_centrality = HashMap::new();
        let num_nodes = graph.node_count() as f64;
        // iterate through each node to calculate the degree centrality
        for node in graph.node_indices() {
            let degree = graph.edges(node).count() as f64;
            let centrality_score = degree / (num_nodes - 1.0);
            degree_centrality.insert(node, centrality_score);
        }
        // Sort the nodes by centrality score  & return hashmap 
        let mut sorted_nodes: Vec<NodeIndex> = degree_centrality.keys().cloned().collect();
        sorted_nodes.sort_by(|&a, &b| degree_centrality[&b].partial_cmp(&degree_centrality[&a]).unwrap());
        let mut sorted_centrality = HashMap::new();
        for node in sorted_nodes {
            sorted_centrality.insert(node, degree_centrality[&node]);
        }
        sorted_centrality // key: node index ; value = centrality score
    }

    pub fn eigenvector_centrality_normalized(graph: &Graph<String, f64, petgraph::Undirected>,
                                      max_iterations: usize, tolerance: f64)
                                      -> HashMap<NodeIndex, f64> {                                 
        let n = graph.node_count();
        let mut centrality: Vec<f64> = vec![0.5; n]; // initialize value to 0.5
        let mut new_centrality: Vec<f64> = vec![0.0; n]; // vector to hold updated centrality scores
        
        let mut result = HashMap::new(); //hashmap to store result
        // Iterate until convergence or maximum iterations reached
        for i in 0..max_iterations {  
            let mut max_diff: f64 = 0.0;
            for j in 0..n {
                let mut sum = 0.0;
                let node = NodeIndex::new(j);
                    for edge in graph.edges(node) {
                        let neighbor = edge.target();
                        let neighbor_degree = graph.edges(neighbor).count();
                        sum += centrality[neighbor.index()] / neighbor_degree as f64;
                        }
                    new_centrality[j] = sum;
                    max_diff = max_diff.max((new_centrality[j] - centrality[j]).abs());
                    }
                // check if the centrality scores have converged
                centrality.swap_with_slice(&mut new_centrality);
                if max_diff < tolerance {
                println!("Converged after {} iterations", i + 1);
                break;
                }
            }
        for (i, &c) in centrality.iter().enumerate() {
        result.insert(NodeIndex::new(i), c);
            }
         // Normalize the centrality scores
        let max_centrality: f64 = result.values().fold(0.0, |a, &b| a.max(b));
        for (_, c) in result.iter_mut() {
        *c /= max_centrality;
        }
        result
    }

    pub fn betweenness_centrality(graph: &Graph<String, f64, Undirected>) -> HashMap<NodeIndex, f64> {
        let mut betweenness_centrality = HashMap::new();
        for node in graph.node_indices() {
            betweenness_centrality.insert(node, 0.0); // initialize the score to 0
        }
        for node in graph.node_indices() {
            let mut stack = Vec::new();
            let mut path_count = HashMap::new();
            let mut distance = HashMap::new();
            // sigma : a hash map to keeps track of the number of shortest paths that pass through a particular node
            let mut sigma = HashMap::new();
            
        // Initialize all variables to a default value
            for v in graph.node_indices() {
                path_count.insert(v, 0);
                distance.insert(v, -1);
                sigma.insert(v, 0);
            }
            sigma.insert(node, 1);
            distance.insert(node, 0);
            let mut queue = Vec::new();
            queue.push(node);
    
            while !queue.is_empty() {
                let v = queue.remove(0);
                stack.push(v);
                for edge in graph.edges_directed(v, petgraph::Outgoing) {
                    let w = edge.target();
                    if distance[&w] < 0 {
                        queue.push(w);
                        distance.insert(w, distance[&v] + 1);
                    }
                    if distance[&w] == distance[&v] + 1 {
                        sigma.insert(w, sigma[&w] + sigma[&v]);
                        path_count.insert(w, path_count[&w] + path_count[&v]);
                    }
                }
            }
            // delta : a hash map to keep track of the sum of fraction of shortest paths that pass through a particular node 
            let mut delta = HashMap::new();
            for v in graph.node_indices() {
                delta.insert(v, 0.0);
            }
            // go through the stack in reverse order to calculate the betweenness centrality for each node
            while let Some(v) = stack.pop() {
                for edge in graph.edges_directed(v, petgraph::Incoming) {
                    let w = edge.source();
                    let c = (sigma[&v] as f64 / sigma[&w] as f64) * (1.0 + delta[&v]);
                    delta.insert(w, delta[&w] + c);
                    if v != node {
                        betweenness_centrality.insert(v, betweenness_centrality[&v] + c);
                    }
                }
            }
        }
        // Normalizing the scores
        let mut sum = 0.0;
        for score in betweenness_centrality.values() {
            sum += score;
        }
        let scaling_factor = 1.0 / sum;
        for (_node, score) in betweenness_centrality.iter_mut() {
            *score = scaling_factor * *score;
        }
        betweenness_centrality
    }
}

