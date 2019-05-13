//-------------------------------Import----------------------------------------------------------------
extern crate priority_queue;

use std::collections::{HashSet};
use priority_queue::PriorityQueue;
use crate::centroid_calculation::skeleton_centerpoint::centrality_scores_calculation::graph::{Vertex, Edge};



//------------------------------Publics ----------------------------------------------------------------

pub fn get_all_shortest_paths(edges : &Vec<Edge>, input_vertices : Vec<Vertex>, s_vertex_id: usize, all_end_vertex_indices : &Vec<usize>) -> Vec<Vec<usize>>{
   let mut paths : Vec<Vec<usize>> = Vec::new();
   
   let mut vertices = input_vertices;

   //start Dijkstra Algorithm
    let mut open_vertices = PriorityQueue::new();
    let mut closed_vertices : HashSet<usize> = HashSet::new();

    open_vertices.push(s_vertex_id, 0 as i64);


    while open_vertices.is_empty() == false{
        let (current_node_id, _) = match open_vertices.pop(){
            Some(val) => val,
            None => panic!("The queue wasn't empty, however the pop failed,")
        };
        
        closed_vertices.insert(current_node_id);
        
        expand_node(current_node_id, &mut vertices, &edges, &closed_vertices, &mut open_vertices) ;
    }


    //end Dijkstra Algorithm
    for i in 0..all_end_vertex_indices.len(){
        let end_vertex_id = all_end_vertex_indices[i];
        if end_vertex_id == s_vertex_id{
            continue;
        }
        let path : Vec<usize> = walk_path_back(&vertices, edges, end_vertex_id, s_vertex_id);
        paths.push(path);
    }

    paths
}

//find path back from t -> .. -> s
fn walk_path_back(vertices : &Vec<Vertex>, edges :&Vec<Edge>, t_vertex_id : usize, s_vertex_id : usize) -> Vec<usize>{
    let mut edgepath : Vec<usize> = Vec::new();
    let mut current_vertex_id = t_vertex_id;
    while current_vertex_id !=  s_vertex_id{
        //get current Vertext-struct
        let current_vertex = &vertices[current_vertex_id];
        
        //load successor Vertex and Edge and push on stack.
        let successor_edge_id = current_vertex.pfad_vorgaenger_edge_id;
        let current_edge = &edges[successor_edge_id];
        edgepath.push(current_edge.id);
        
        //set start for nex run
        current_vertex_id = current_edge.get_opposide_vertex(current_vertex_id)
    }
    edgepath
}

fn expand_node(current_node_id : usize, vertices :  &mut Vec<Vertex>, edges : &Vec<Edge>, closed_vertices : &HashSet<usize>, open_vertices : &mut PriorityQueue<usize, i64>){ 
    let node_struct = vertices[current_node_id].clone();
    for successor_edge_id in &node_struct.ausgehende_kanten_ids{
        let sucessor_edge = &edges[*successor_edge_id];
        let successor_vertex_id : &usize = &sucessor_edge.get_opposide_vertex(current_node_id);
        let successor_vertex : &mut Vertex = &mut vertices[*successor_vertex_id];
        
        //skip vertex if already calculated  
        if closed_vertices.contains(successor_vertex_id){
            continue;
        }

        let new_cost_to_current_vertex = node_struct.cost_from_s + sucessor_edge.cost;

        let sucessor_is_in_open_list = match open_vertices.get(successor_vertex_id){
            Some((_, _)) => true,
            None => false
        };
        let olt_cost_to_current_vertex = successor_vertex.cost_from_s;
        if sucessor_is_in_open_list && new_cost_to_current_vertex >= olt_cost_to_current_vertex{
            continue;
        } 
        successor_vertex.pfad_vorgaenger_edge_id = *successor_edge_id;
        successor_vertex.cost_from_s = new_cost_to_current_vertex;

        let queue_priority : i64 = (-1 as i64) *  (new_cost_to_current_vertex as i64);
        
        if sucessor_is_in_open_list{
            open_vertices.change_priority(successor_vertex_id, queue_priority);
        }else {
            open_vertices.push(*successor_vertex_id, queue_priority);
        }

    }
}

