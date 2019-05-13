pub(crate) mod graph;
mod betweenness_centrality;
mod closeness_centrality;
mod shortest_paths;

//-------------------------------Import----------------------------------------------------------------
use std::fs::File;
use std::io::Write;

use graph::{Vertex, Edge};
use super::skeleton_graph_generation::Skeletongraph;
use betweenness_centrality::calculate_betweeness_node_scores;
use closeness_centrality::calculate_closeness_node_scores;

use std::collections::HashSet;
use std::collections::HashMap;

//-------------------------------Enum and Structs-----------------------------------------------------

#[derive(Debug)]
pub enum CentralityVariant {
    Betweenness(SearchingMethod),
    Closeness(SearchingMethod, Weights)
} 
#[derive(Debug)]
pub enum SearchingMethod {
    AStar1To1,
    Disjkstra1ToAll,
    TreepathsShrink
} 

#[derive(Debug)]
pub enum Weights {
    CoverLength 
} 


//------------------------------Publics ----------------------------------------------------------------

pub fn get_most_central_points(skeleton_graph : &Skeletongraph, centrality_type : &CentralityVariant) -> Vec<(f64, f64)>{
    //load disjunct nodes
    let node_set = get_nodes_from_skeleton_edges(&skeleton_graph);
    
    //Convert to graph-structure
    let (mut vertices, position_vertices_mapping) = get_vertices_from_skeleton_graph(&skeleton_graph, node_set);
    let edges = get_edges_from_skeleton_graph(&skeleton_graph, &position_vertices_mapping, &mut vertices);

    //Calculate centrality scores 
    let node_scores = match centrality_type{
        CentralityVariant::Betweenness(method) => calculate_betweeness_node_scores(&skeleton_graph, &position_vertices_mapping, &edges, &vertices, method),
        CentralityVariant::Closeness(searchmethod, weights) => calculate_closeness_node_scores(&skeleton_graph, &position_vertices_mapping, &edges, &vertices, &searchmethod, &weights)
    };

    //Get the highest-ranked vertices
   let best_vetices : HashSet<usize> = get_vertices_with_hightest_score(node_scores);

    //Get corresponding points if is no boundary point
   let best_points : Vec<(f64,f64)> = get_points_from_id(best_vetices, &vertices);
  
   best_points
}

//---

pub fn write_central_points_to_file(most_central_points : &Vec<(f64, f64)>,  file: &mut File){
    let mut writestring = "".to_string(); 
    let point_list_string = point_list_to_string(most_central_points);

    writestring.push_str(&point_list_string);
    writestring.push_str("\n");
    
    //write string to file
    file.write_all(writestring.as_bytes());
}



//Subparts of public method----------------------------------------------------------------------------

fn get_nodes_from_skeleton_edges(skeleton_graph : &Skeletongraph) ->  HashSet<(i64,i64)>{
    let mut node_set = HashSet::new();

    //load all vertices
    for edge in &skeleton_graph.graph_edges{
        let startpoint : (i64, i64) = (edge.0 as i64, edge.1 as i64);
        let endpoint  : (i64, i64)   = (edge.2 as i64, edge.3 as i64);
        node_set.insert(startpoint);
        node_set.insert(endpoint);
    }

    node_set
}

//---

fn get_vertices_from_skeleton_graph(
    skeleton_graph : &Skeletongraph,
    node_set : HashSet<(i64,i64)>) 
    -> 
    (Vec<Vertex>, HashMap<(i64, i64), usize>)
{
    let mut vertices = Vec::new();    
    let mut position_vertices_mapping = HashMap::new();    
 
    //create vertices structs
    let mut vertex_counter : usize = 0;
    for entry in node_set.into_iter(){
        
        //check for endpoints-flag
        let (x,y) = entry;
        let mut is_endpoint = false;
        if skeleton_graph.leafs.contains(&(x as f64,y as f64)){
            is_endpoint = true;
        }

        let vertex = Vertex::new(vertex_counter, entry, is_endpoint);

        position_vertices_mapping.insert(entry, vertex_counter);
        vertices.push(vertex);
        vertex_counter = vertex_counter +1;
    }
    
    (vertices, position_vertices_mapping)
}

//---

fn get_edges_from_skeleton_graph(
    skeleton_graph : &Skeletongraph,
    position_vertices_mapping : &HashMap<(i64, i64), usize>,
    vertices : &mut Vec<Vertex>) 
    -> 
    Vec<Edge>
{
    //add all edges and update vertex
    let mut edges = Vec::new();    
    let mut edge_counter : usize = 0;
    for edge in &skeleton_graph.graph_edges{
        let startpoint : (i64, i64) = (edge.0 as i64, edge.1 as i64);
        let endpoint  : (i64, i64)   = (edge.2 as i64, edge.3 as i64);
        let first_vertex = match position_vertices_mapping.get(&startpoint) {
            Some(value) => *value,
            None => panic!("One Edge conntects a vertex with another, which isn't known."),
        };
        let second_vertex = match position_vertices_mapping.get(&endpoint) {
            Some(value) => *value,
            None => panic!("One Edge conntects a vertex with another, which isn't known."),
        };
        let cost = get_weight((edge.0, edge.1, edge.2, edge.3));
        let edge = Edge{
            id : edge_counter,
            connects : (first_vertex, second_vertex),
            cost : cost
        };
        vertices[first_vertex].add_edge(edge_counter);
        vertices[second_vertex].add_edge(edge_counter);
        edges.push(edge);
        edge_counter = edge_counter + 1;
    }

    edges
}

//---

fn get_vertices_with_hightest_score(node_scores : HashMap<usize, u32>) -> HashSet<usize>{
    let mut best_nodes = HashSet::new(); 
   
    //Collect Node(s) with the most hits
    let mut maxval = 0;
    for (node, score_val) in node_scores{
        if maxval < score_val{
            maxval = score_val;
            best_nodes.clear();
            best_nodes.insert(node);
        }
        else if maxval == score_val{
            best_nodes.insert(node);
        }
    }
    println!("{:?}", best_nodes);
    best_nodes
}

//---

fn get_points_from_id(best_nodes : HashSet<usize>, vertices : &Vec<Vertex> ) -> Vec<(f64, f64)>{
    let mut best_vetices : Vec<(f64, f64)> = Vec::new();
    for vertex_id in best_nodes{
        let current_vertex = &vertices[vertex_id];
        if current_vertex.is_endvertex {
            continue; //put no edge-points to final set
        }
        let (x,y) = current_vertex.position;
        best_vetices.push((x as f64, y as f64));
    }

    best_vetices
}




//------------------------------Privates ---------------------------------------------------------------
fn get_weight((a,b,c,d) : (f64, f64, f64, f64)) -> u64{
    let mut xdiff = c-a;
    let mut y_diff = d-b;
    
    xdiff = xdiff * xdiff;
    y_diff = y_diff * y_diff;

    (xdiff + y_diff).sqrt() as u64
}

//---

fn point_list_to_string(points : &Vec<(f64, f64)>) -> String{
    let mut mutstring = "".to_string();
    for (x,y) in points {
       mutstring.push_str(&format!("{} {} ", x,y ));
    }
    
    mutstring
}