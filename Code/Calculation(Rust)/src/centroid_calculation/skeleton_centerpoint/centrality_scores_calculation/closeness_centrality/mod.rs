//-------------------------------Import----------------------------------------------------------------

use super::graph::{Vertex, Edge};
use super::shortest_paths::dijkstra_all_paths::{get_all_shortest_paths};
use crate::centroid_calculation::skeleton_centerpoint::skeleton_graph_generation::Skeletongraph;
use super::{SearchingMethod, Weights};

use std::collections::HashMap;



//----------------Public API----------------------------------------------------

pub fn calculate_closeness_node_scores(
    skeleton_graph : &Skeletongraph,
    position_vertices_mapping : &HashMap<(i64, i64), usize>,
    edges : &Vec<Edge>, 
    vertices : &Vec<Vertex>,
    method : &SearchingMethod,
    weight: &Weights) 
    -> 
    HashMap<usize, u32>
{
    match method {
        SearchingMethod::Disjkstra1ToAll => calculate_dijkstra_closeness_node_scores(skeleton_graph, position_vertices_mapping, edges, vertices, weight),
        _ => panic!("Method not implemented yet:")
    }

    
}





//----------------HIGH-Level-Methods (private)---------------------------------------------------


//node_scores is map with 
//  usize        --->  f64 (roundet to u32)  by
//  vertex_id    |-->  inverse_standard_deviation
fn calculate_dijkstra_closeness_node_scores(
    skeleton_graph : &Skeletongraph,
    position_vertices_mapping : &HashMap<(i64, i64), usize>,
    edges : &Vec<Edge>, 
    vertices : &Vec<Vertex>,
    weight: &Weights) 
    -> 
    HashMap<usize, u32>
{

    //collect all leaf-vertices
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

    //calc scores
    let mut node_scores = HashMap::new();
    for i in 0..vertices.len(){

        //Ignore border points
        if vertex_is_earpoint(&all_leaf_vertex_ids, i){
            set_lowest_score(i, &mut node_scores);
            continue;
        }  

        //get all pahs from i to everywhere
        let paths = get_all_shortest_paths(&edges, vertices.clone(), vertices[i].id, &all_leaf_vertex_ids); 
        
        let mut all_lenghts : Vec<u64> = Vec::new();
        for path in paths{
            let current_length = get_path_length(path, edges);
            all_lenghts.push(current_length)            
        }
        if all_lenghts.len() < 2{
            //Ignore the case only 1 way to a vertex exist: Vertex is on Border and the skeleton-'tree' is a path (with no branches)
            set_lowest_score(i, &mut node_scores);
            continue;
        }

        let standard_deviation = get_standard_deviation(all_lenghts);


        if standard_deviation == 0f64{
            //Case: The current Vertex is exactly in the middle of all paths. -> it's optimal, set to max value.
            set_hightest_score(i, &mut node_scores);
            continue;
        }
        assert!(standard_deviation > 0f64);
        let pseudo_inverse_standard_deviation = std::u32::MAX - (standard_deviation as u32);
        //println!("inv. std dev: {}", pseudo_inverse_standard_deviation);
        node_scores.insert(i, pseudo_inverse_standard_deviation);
    }

    node_scores
}

//----------------LOW-Level-Methods (private)---------------------------------------------------

fn get_path_length(path : Vec<usize>, edges : &Vec<Edge>) -> u64{
    let mut total_path_length = 0u64;
    for edge_id in path{
        let edge = &edges[edge_id]; 
        total_path_length = total_path_length + edge.cost;
    }

    total_path_length
}

//---

fn get_standard_deviation(all_values: Vec<u64>) -> f64{
    assert!(all_values.len() > 1);

    let avg = get_mean_value(&all_values);
    let mut mean_square_deviation_sum = 0f64;
    for value in &all_values{
        let mean_deviaion = (*value as f64) - avg; 
        let mean_square_deviation = mean_deviaion  * mean_deviaion;
        mean_square_deviation_sum = mean_square_deviation_sum + mean_square_deviation;
    }
    let averaged_sum = mean_square_deviation_sum / ((all_values.len() -1) as f64); 
    let standard_deviation = averaged_sum.sqrt();

    standard_deviation
}

//---

fn get_mean_value(all_values: &Vec<u64>)-> f64{
    assert!(all_values.len() > 0);
    let mut sum  = 0u64;
    for value in all_values{
        sum = sum + value;
    }
    let f_sum = sum as f64;
    let mean = f_sum / (all_values.len() as f64);

    mean
}

//---

fn set_lowest_score(vertex: usize, node_scores : &mut HashMap<usize, u32>){
    set_score(vertex, node_scores, 0u32);
}

//---

fn set_hightest_score(vertex: usize, node_scores : &mut HashMap<usize, u32>){
    set_score(vertex, node_scores, std::u32::MAX);
}

//---

fn set_score(vertex: usize, node_scores : &mut HashMap<usize, u32>, score: u32){
    node_scores.insert(vertex, score);
}

//---

fn vertex_is_earpoint(all_leaf_vertex_ids : &Vec<usize>, testvertex : usize)-> bool{
    all_leaf_vertex_ids.contains(&testvertex)
}




//----------------Tests-----------------------------------------------------
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_mean_value() {
        let mut testvec : Vec<u64> = Vec::new();
        testvec.push(12);
        testvec.push(10);
        testvec.push(10);
        testvec.push(9);
        testvec.push(5);
        let mean = get_mean_value(&testvec);
        println!("mean:    {}", mean);
        println!("std_dev: {}", get_standard_deviation(testvec));        
   
    }
}