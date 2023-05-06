pub mod read_txt_to_graph {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use petgraph::Undirected;
    use petgraph::graph::{Graph};

    pub fn read_file(filename: &str) -> std::io::Result<Graph<String, f64, Undirected>> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let mut lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
        lines.sort();
        let mut graph = Graph::<String, f64, Undirected>::default();
        for line in lines {
            let line = line;
            let parts: Vec<_> = line.split(',').map(|p| p.to_string()).collect();
            let departure_name = &parts[0];
            let return_name = &parts[1];
            let distance = match parts[3].parse::<f64>() {
                Ok(d) => d,
                Err(_) => {
                    eprintln!("Invalid distance value: {}", parts[3]);
                    continue;
                }
            };
            let departure_index = match graph.node_indices().find(|i| graph[*i] == *departure_name) {
                Some(i) => i,
                None => {
                    let i = graph.add_node(departure_name.clone());
                    i
                }
            };
            let return_index = match graph.node_indices().find(|i| graph[*i] == *return_name) {
                Some(i) => i,
                None => {
                    let i = graph.add_node(return_name.clone());
                    i
                }
            };
            graph.add_edge(departure_index, return_index, distance);
        }
        return Ok(graph);
    }
}