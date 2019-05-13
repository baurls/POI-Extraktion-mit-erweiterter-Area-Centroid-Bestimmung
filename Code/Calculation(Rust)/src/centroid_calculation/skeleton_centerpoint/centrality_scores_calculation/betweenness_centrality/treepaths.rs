//-------------------------------Import----------------------------------------------------------------
extern crate priority_queue;

use std::collections::{HashSet};
use std::collections::HashMap;
use priority_queue::PriorityQueue;

use crate::centroid_calculation::skeleton_centerpoint::centrality_scores_calculation::graph::{Vertex, Edge};


//------------------------------Publics ----------------------------------------------------------------

pub fn get_treepath_shrink_scores(edges : &Vec<Edge>, vertices : Vec<Vertex>, end_vertices_indices : Vec<usize>) -> HashMap<usize, u32>{
    let mut scores = initialize_vector(vertices.len(), 0);
    let mut representitive = initialize_vector(vertices.len(), 0);

    let count_end_vertices = end_vertices_indices.len();

    let mut closed_list : HashSet<usize> = HashSet::new();
    let mut prioqueue : PriorityQueue<usize, i32> = PriorityQueue::new();

    let mut current_smallest_priority = 0i32;

    assert!( scores.len() == vertices.len());

    
    //initialization
    for current_end_vertex_index in end_vertices_indices{
        closed_list.insert(current_end_vertex_index);   
        representitive[current_end_vertex_index] = 1;

        let current_end_vertex = &vertices[current_end_vertex_index];
        for adjacent_vertex_id in current_end_vertex.get_adjacient_vertices(edges){
            add_to_queue_or_increment_prio(&adjacent_vertex_id, &mut prioqueue);
        }
    }
    //main part
    while prioqueue.is_empty() == false {
        let (current_node_id, _) = prioqueue.pop().unwrap();
        let current_node = &vertices[current_node_id];
        let adjacent_ids = current_node.get_adjacient_vertices(&edges);

        //count vertex without scores
        let mut count_not_computed_vertices = 0;
        let mut already_calculated_ids : Vec<usize> = Vec::new();
        for adjacent_id in &adjacent_ids{
            if !closed_list.contains(&adjacent_id){
                count_not_computed_vertices = count_not_computed_vertices +1;
            }
            else {
                already_calculated_ids.push(*adjacent_id);
            }
        }
        if count_not_computed_vertices > 1{
            current_smallest_priority = current_smallest_priority -1;
            prioqueue.push(current_node_id, current_smallest_priority);
            continue; 
        }  
        //The current (open) vertex has 0 or 1 neighbour without a score yet.
        assert!(count_not_computed_vertices == 0 || count_not_computed_vertices == 1);
        
        //calculate score
        let (current_score, other_representives_sum)  =  calculate_score(&already_calculated_ids, &representitive, count_end_vertices);
        scores[current_node_id] = current_score;

        closed_list.insert(current_node_id);
        representitive[current_node_id] = other_representives_sum;

        //füge neu gefundene Knoten in Queue ein:
        for adjacent_vertex_id in &adjacent_ids{
            if closed_list.contains(&adjacent_vertex_id){
                continue;
            }
            add_to_queue_or_increment_prio(adjacent_vertex_id, &mut prioqueue);
        }
    }
    
    let mut resultlist : HashMap<usize, u32> = HashMap::new();
    //convert to result-format
    for i in 0..vertices.len(){
        resultlist.insert(i, scores[i] as u32);
    }    
    println!("Result: {:?}", resultlist);
    resultlist
}

fn add_to_queue_or_increment_prio(adjacent_vertex_id : &usize, prioqueue : &mut PriorityQueue<usize, i32> ){
    let old_entry = prioqueue.get(adjacent_vertex_id);
    //Add, if not in queue, or increment by one
    prioqueue.push(*adjacent_vertex_id, 
                match old_entry {
                    Some((_, priority)) => priority + 1,
                    None => 1,
                });
}

fn calculate_score(already_calculated_ids : &Vec<usize>, representitive : &Vec<usize>, total_vertices_amount : usize)-> (usize, usize){
    let mut total_representives_others = 0;
    let mut vertex_values : Vec<usize> = Vec::new();
    for already_calculated_id in already_calculated_ids{
        let vertex_value = representitive[*already_calculated_id];
        vertex_values.push(vertex_value);
        total_representives_others = total_representives_others + representitive[*already_calculated_id];
    }
    let self_representives = total_vertices_amount - total_representives_others;
    assert!(self_representives >= 0);
    vertex_values.push(self_representives);
    
    let score = get_pairwise_distinct_multiplication_sum(&vertex_values);
    
    (score, total_representives_others)
}

fn initialize_vector(size : usize, init_value : usize) -> Vec<usize>{
    let mut empty_vec : Vec<usize> = Vec::new(); 
    for i in 0..size{
        empty_vec.push(init_value);
    }

    empty_vec
}

fn get_pairwise_distinct_multiplication_sum(values: &Vec<usize>) -> usize{
    let mut multiplication_sum = 0;
    for i in 0..values.len() -1 {
        for j in i+1..values.len(){
            multiplication_sum = multiplication_sum + (values[i] * values[j]);
        }
    }

    multiplication_sum
}

#[test]
fn test_treepaths() {
    
}
