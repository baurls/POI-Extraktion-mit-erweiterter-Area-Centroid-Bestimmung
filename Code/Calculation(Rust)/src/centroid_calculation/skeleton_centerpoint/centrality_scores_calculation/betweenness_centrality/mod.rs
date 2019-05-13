pub(crate) mod treepaths;

//-------------------------------Import----------------------------------------------------------------

use super::graph::{Vertex, Edge};
use super::shortest_paths::a_star::{get_shortest_path};
use super::shortest_paths::dijkstra_all_paths::{get_all_shortest_paths};
use treepaths::get_treepath_shrink_scores;
use crate::centroid_calculation::skeleton_centerpoint::skeleton_graph_generation::Skeletongraph;
use super::{SearchingMethod};

use std::collections::HashSet;
use std::collections::HashMap;



//----------------Public API----------------------------------------------------

pub fn calculate_betweeness_node_scores(
    skeleton_graph : &Skeletongraph,
    position_vertices_mapping : &HashMap<(i64, i64), usize>,
    edges : &Vec<Edge>, 
    vertices : &Vec<Vertex>,
    method : &SearchingMethod) 
    -> 
    HashMap<usize, u32>
{
    match method {
        SearchingMethod::AStar1To1 => calculate_a_start_betweeness_node_scores(skeleton_graph, position_vertices_mapping, edges, vertices),
        SearchingMethod::Disjkstra1ToAll => calculate_dijkstra_betweeness_node_scores(skeleton_graph, position_vertices_mapping, edges, vertices),
        SearchingMethod::TreepathsShrink => calculate_treeshrink_betweeness_node_scores(skeleton_graph, position_vertices_mapping, edges, vertices),
     } 

    
}





//----------------HIGH-Level-Methods (private)---------------------------------------------------



fn calculate_a_start_betweeness_node_scores(
    skeleton_graph : &Skeletongraph,
    position_vertices_mapping : &HashMap<(i64, i64), usize>,
    edges : &Vec<Edge>, 
    vertices : &Vec<Vertex>) 
    -> 
    HashMap<usize, u32>
{
    let mut node_scores = HashMap::new();

     for i in 0..skeleton_graph.leafs.len(){
        for j in i+1..skeleton_graph.leafs.len(){
            let (sx, sy) = skeleton_graph.leafs[i];
            let (tx, ty) = skeleton_graph.leafs[j];

            let s = (sx as i64, sy as i64);
            let t = (tx as i64, ty as i64);

            let start_index = match position_vertices_mapping.get(&s){
                Some(index) => *index,
                None => panic!("The start-position has no corresponding vertex.")
            };

            let end_index = match position_vertices_mapping.get(&t){
                Some(index) => *index,
                None => panic!("The start-position has no corresponding vertex.")
            };

            
            let path = get_shortest_path(&edges, vertices.clone(),start_index, end_index); 
            
            //Collect Nodes on the way from start to target
            let disjunct_nodes = get_nodes_from_path(path, &edges);
          
            //Increment total visited-counter for each node
            increment_visited_score_for_nodes(disjunct_nodes, &mut node_scores);
          
        }
        
    }

    node_scores
}



fn calculate_dijkstra_betweeness_node_scores(
    skeleton_graph : &Skeletongraph,
    position_vertices_mapping : &HashMap<(i64, i64), usize>,
    edges : &Vec<Edge>, 
    vertices : &Vec<Vertex>) 
    -> 
    HashMap<usize, u32>
{
    let mut node_scores = HashMap::new();

    let all_leaf_vertex_ids = get_all_leaf_ids(&skeleton_graph, &position_vertices_mapping);
  
    for i in 0..all_leaf_vertex_ids.len(){       
        let paths = get_all_shortest_paths(&edges, vertices.clone(), all_leaf_vertex_ids[i], &all_leaf_vertex_ids); 
        
        for path in paths{
            //Collect Nodes on the way from start to target
            let disjunct_nodes = get_nodes_from_path(path, &edges);
            
            //Increment total visited-counter for each node
            increment_visited_score_for_nodes(disjunct_nodes, &mut node_scores);
        }
        
    }

    node_scores
}


fn calculate_treeshrink_betweeness_node_scores(
    skeleton_graph : &Skeletongraph,
    position_vertices_mapping : &HashMap<(i64, i64), usize>,
    edges : &Vec<Edge>, 
    vertices : &Vec<Vertex>) 
    -> 
    HashMap<usize, u32>
{

    let all_leaf_vertex_ids = get_all_leaf_ids(&skeleton_graph, &position_vertices_mapping);
  
    let node_scores =  get_treepath_shrink_scores(edges, vertices.clone(), all_leaf_vertex_ids);

    node_scores
}


//----------------LOW-Level-Methods (private)---------------------------------------------------

fn get_all_leaf_ids(skeleton_graph : &Skeletongraph, position_vertices_mapping : &HashMap<(i64,i64), usize>) -> Vec<usize>{
  let mut all_leaf_vertex_ids = Vec::new(); 
  
  for leaf in &skeleton_graph.leafs{
        let (x, y) = leaf.clone();
        let s = (x as i64, y as i64);
        
        let current_endvertex = match position_vertices_mapping.get(&s){
                Some(index) => *index,
                None => panic!("The start-position has no corresponding vertex.")
            };
        all_leaf_vertex_ids.push(current_endvertex);
    }

    all_leaf_vertex_ids
}

fn increment_visited_score_for_nodes(disjunct_nodes : HashSet<usize>, node_scores : &mut HashMap<usize, u32> ){
  for node in disjunct_nodes{
                let old = match node_scores.get(&node){
                    Some(x) => *x,
                    None => 0
                };
                node_scores.insert(node, old +1);
            }
}

fn get_nodes_from_path(path : Vec<usize> , edges : &Vec<Edge>) -> HashSet<usize>{
    let mut disjunct_nodes = HashSet::new();
    for pathpart_id in path{
            let pathpart = &edges[pathpart_id];
            disjunct_nodes.insert(pathpart.connects.0);
            disjunct_nodes.insert(pathpart.connects.1);
        }
    
    disjunct_nodes
}
