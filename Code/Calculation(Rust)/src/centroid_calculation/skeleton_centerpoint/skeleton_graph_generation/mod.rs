/****************************************************************************
*  ++ LUKAS BAUR, Bachelorarbeit 2018/2019 ++
*
*  This module calculates skeleton graphs based on a given triangulation
*
*****************************************************************************/

extern crate poly2tri;

//-------------------------------Dokumentation----------------------------------------------------------
/*
INPUT
    <triangulation>  = (<triangles>, <outer_lines>)
WHERE
    <triangles>      = [<triangle>, <triangle>, <triangle>, <triangle>, .. , <triangle>]
    <triangle>       = [[x1, y1], [x2, y2], [x3, y3]]
    <outer_lines>    = [<outer_line>, <outer_line>, <outer_line>, ..., <outer_line>, <outer_line>] 
    <outer_line>     = (x1, y1, x2, y2)

OUTPUT
    <skeleton_graph> = [<linesegment>, <linesegment>, ..., <linesegment>]
WHERE
    <linesegment>    = (a_x, a_y, b_x, b_y)
    [x, y] = (f64, f64),
    [<*>, .. <*>] = Vec<*>
*/


//-------------------------------Import----------------------------------------------------------------
  //global
use std::fs::File;
use std::io::Write;   
use std::collections::{HashSet};
use std::f64::MAX;

  //local
use super::triangulation::Polygontriangulation;
use super::triangulation::is_a_diagonal;



//------------------------------Structs----------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub enum SkeletonType {
    AignerApproach,
    FmiCenterBased
}

#[derive(Debug)]
enum TriangleType {
    Ear,
    Link,
    Branch,
}


#[derive(Debug)]
pub struct Skeletongraph {
    pub graph_edges : Vec<(f64,f64,f64,f64)>,
    pub leafs : Vec<(f64,f64)>
}
impl Skeletongraph {
    pub fn new(skeleton_graph : Vec<(f64,f64,f64,f64)>, endpoints : Vec<(f64,f64)>) -> Skeletongraph{
        Skeletongraph{
            graph_edges : skeleton_graph,
            leafs : endpoints
        }
    }
}


//------------------------------public functions----------------------------------------------------------------
//--------------Main-----------------------------
pub fn generate_polygon_skeleton_graph(triangulation : &Polygontriangulation, skeleton_type : SkeletonType) -> Skeletongraph{
    let triangles = triangulation.get_triangles();
    let outer_lines = triangulation.get_outer_lines();

    let skel = match skeleton_type{
        SkeletonType::AignerApproach => get_aigner_skeleton(&triangles, & outer_lines),
        SkeletonType::FmiCenterBased => get_fmi_center_based_skeleton(&triangles, & outer_lines)
    };

    skel
}

pub fn write_skeleton_to_file(skeletongraph : &Skeletongraph,  file: &mut File){
    let skeleton_lines : &Vec<(f64,f64,f64,f64)> = &skeletongraph.graph_edges;
    for line in skeleton_lines{
        let (a,b,c,d) = line;
            let file_content = format!("{} {} {} {} ", a,b,c,d);
            file.write_all(file_content.as_bytes());
    }
    file.write_all("\n".as_bytes());
}


//---------------------------private functions -----------------------------------------------------
//%%%%%%%%% FmiCenterBased %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
fn get_fmi_center_based_skeleton(triangles : &Vec<[[f64; 2]; 3]>, outer_lines : &Vec<(f64, f64, f64, f64)>) -> Skeletongraph{
    let mut skeleton_graph : Vec<(f64,f64,f64,f64)> = Vec::new();
    let mut endpoints : Vec<(f64,f64)> = Vec::new();
    
    let mut closed_list : HashSet<usize> = HashSet::new();
    let mut current_trinangle_index = 0;
    for triangle in triangles{
        //mark triangle as already done
        closed_list.insert(current_trinangle_index);

        //add addidional part for ears
        let triangle_type : TriangleType = get_triangle_type(outer_lines ,*triangle);
        match triangle_type{
            TriangleType::Ear =>
            {
                let part = get_part_skeleton_of_ear_triangle(outer_lines, *triangle, &mut endpoints);
                skeleton_graph.push(part);
            }
            _ => {}
        }

        //add center-center lines.
        let adjacent_triangle_indices = get_adjacent_triangle_indices(current_trinangle_index, triangles);

        let (center_x,center_y) =  get_streched_triangle_centroid(*triangle);
        for adjacent_triangle_index in adjacent_triangle_indices{
            if closed_list.contains(&adjacent_triangle_index) {
                continue;
            }
            
            let other_triangle = triangles[adjacent_triangle_index];
            let (center_x1,center_y1) =  get_streched_triangle_centroid(other_triangle);
            let linesegment = (center_x, center_y, center_x1,center_y1);
            skeleton_graph.push(linesegment);
        
        }
        
        
        //prepare for next run
        current_trinangle_index = current_trinangle_index +1
    }
         
    
    let skel : Skeletongraph = Skeletongraph::new(skeleton_graph, endpoints);

    skel
}

 fn get_adjacent_triangle_indices(current_trinangle_index: usize, triangles : &Vec<[[f64; 2]; 3]>) -> Vec<usize>{
    let current_triangle = triangles[current_trinangle_index];
    let mut matches : Vec<usize> = Vec::new();
    for i in 0..triangles.len(){
        if i == current_trinangle_index{
            continue;
        }
        let other_trinangle = triangles[i];
        if triangles_are_adjacent(&current_triangle, &other_trinangle){
            matches.push(i);
        }
    }

    matches
}

fn triangles_are_adjacent(current_triangle : &[[f64; 2]; 3], other_trinangle: &[[f64; 2]; 3])-> bool{
    for i in 0..3{
        let edge = (current_triangle[i], current_triangle[(i+1)%3]);
        for j in 0..3{
            let other_edge = (other_trinangle[j], other_trinangle[(j+1)%3]);
            let inverse_other_edge = (other_trinangle[(j+1)%3], other_trinangle[j]);
            if edge == other_edge || edge == inverse_other_edge{
                return true;
            }
        }
    }
 
    false
}



/*          *
            . *
           .    *
          .       *
         .          *
        .             *
       .                *
      .        x          *
     .     o                *
    .  o                      *
   o............................*
*/
fn get_part_skeleton_of_ear_triangle(polygon_lines : &Vec<(f64,f64,f64,f64)> ,  ear_triangle : [[f64; 2]; 3], endpoints: &mut Vec<(f64,f64)>) -> (f64,f64,f64,f64) {
    //ear triangle = one side is not part of the outer polygon (= diagonal)
    let (a,b,c,d)  = get_ear_diagonal(polygon_lines, ear_triangle);

    //get endpoint
    let (opp_x, opp_y) = get_opposide_vertex_of_diagonal(ear_triangle, a,b,c,d);
    endpoints.push((opp_x, opp_y));
    
    //get center point of diagonal via interpolationof triangle
    let (center_x,center_y) =  get_streched_triangle_centroid(ear_triangle);

    (opp_x, opp_y,center_x,center_y)
}


//%%%%%%%%% AIGNER %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
fn get_aigner_skeleton(triangles : &Vec<[[f64; 2]; 3]>, outer_lines : &Vec<(f64, f64, f64, f64)>) -> Skeletongraph{
    let mut skeleton_graph : Vec<(f64,f64,f64,f64)> = Vec::new();
    let mut endpoints : Vec<(f64,f64)> = Vec::new();
    
    for i in 0.. triangles.len(){
        //get triangle
        let triangle = triangles[i];
                    
        let triangle_graph : Vec<(f64,f64,f64,f64)> = get_aigner_triangle_skeleton_graph(&outer_lines, triangle, &mut endpoints);
        for graph_part in triangle_graph{
            skeleton_graph.push(graph_part);
        }
    } 
    
    let skel : Skeletongraph = Skeletongraph::new(skeleton_graph, endpoints);

    skel
}


fn get_aigner_triangle_skeleton_graph(outer_lines : &Vec<(f64,f64,f64,f64)>, triangle : [[f64; 2]; 3], endpoints: &mut Vec<(f64,f64)>)-> Vec<(f64,f64,f64,f64)>{
    //get type
    let triangle_type : TriangleType = get_triangle_type(outer_lines ,triangle);

    //get suitable skeleton
    match triangle_type {
        TriangleType::Ear => get_skeleton_of_ear_triangle(outer_lines, triangle,  endpoints),
        TriangleType::Link => get_skeleton_of_link_triangle(outer_lines, triangle),
        TriangleType::Branch => get_skeleton_of_branch_triangle(triangle),
        
    }
}

//---



/*
         *
        .  *
       .      *
      .          o
     .      o       *
    .  o               *
   o......................*

*/
fn get_skeleton_of_ear_triangle(polygon_lines : &Vec<(f64,f64,f64,f64)> ,  ear_triangle : [[f64; 2]; 3], endpoints: &mut Vec<(f64,f64)>) -> Vec<(f64,f64,f64,f64)> {
    //ear triangle = one side is not part of the outer polygon (= diagonal)
    let (a,b,c,d)  = get_ear_diagonal(polygon_lines, ear_triangle);

    //get endpoint
    let (opp_x, opp_y) = get_opposide_vertex_of_diagonal(ear_triangle, a,b,c,d);
    endpoints.push((opp_x, opp_y));
    
    //get center point of diagonal via interpolation
    let (diag_x,diag_y) = interpolate_points(a,b,c,d);

    let mut skeleton : Vec<(f64,f64,f64,f64)> = Vec::new();
    skeleton.push((opp_x, opp_y,diag_x,diag_y));
    skeleton
}


/*
         b
        *  *
       *      *
      o o o o o o o
     *              *
    *                  *
   a.....................c

*/
fn get_skeleton_of_link_triangle(polygon_lines : &Vec<(f64,f64,f64,f64)> ,  link_triangle : [[f64; 2]; 3]) -> Vec<(f64,f64,f64,f64)> {
    //link triangle = two side are not part of the outer polygon (= diagonal)
    let mut skeleton : Vec<(f64,f64,f64,f64)> = Vec::new();
    let (ax, ay, bx, by, cx, cy) = get_link_diagonal_points(polygon_lines, link_triangle);
    let (ab_x, ab_y) = interpolate_points(ax, ay, bx, by);
    let (bc_x, bc_y) = interpolate_points(cx, cy, bx, by);
    let linesegment =  (ab_x, ab_y, bc_x, bc_y);
    skeleton.push(linesegment);
    skeleton
}


/*
         *
        *  *
       *      *
      o o o o o o o
     *       o      *
    *         o        *
   * * * * * * o * * * * *

*/
fn get_skeleton_of_branch_triangle(branch_triangle : [[f64; 2]; 3]) -> Vec<(f64,f64,f64,f64)> {
    //branch triangle = all side are not part of the outer polygon (= diagonal)
    let mut skeleton : Vec<(f64,f64,f64,f64)> = Vec::new();
    let triangle_centroid = get_triangle_centroid(branch_triangle);
    let (center_x, center_y) = triangle_centroid;
    for i in 0..3{
        let point_i = branch_triangle[i];
        let point_i1 = branch_triangle[(i+1)%3];
        let border_line = (point_i[0], point_i[1], point_i1[0],  point_i1[1]);
        let (ax, ay, bx, by) = border_line;
        let (ab_x, ac_y) = interpolate_points(ax, ay, bx, by);
        let linesegment = (ab_x, ac_y, center_x, center_y);
        skeleton.push(linesegment);        
    }
    
    skeleton
}

//---



//----------- private helper methods----------------------------------------------------------------- 
fn get_triangle_centroid(triangle : [[f64; 2]; 3]) -> (f64,f64){
    let mut x_sum = triangle[0][0];
    let mut y_sum = triangle[0][1];
    for i in 1..3{
        x_sum = x_sum + triangle[i][0];
        y_sum = y_sum + triangle[i][1];
    }
    x_sum = x_sum / (3 as f64);
    y_sum = y_sum / (3 as f64);

    (x_sum, y_sum)
}

//---
// a
// *     *
// *           *
// *                 *
// d                       *
// *                            c
// *                 *             
// *       *             
// b 
// shortest edge = d
fn get_streched_triangle_centroid(triangle : [[f64; 2]; 3]) -> (f64,f64){
    let mut shortest_edge = (0f64, 0f64, 0f64, 0f64);    
    let mut shortest_sq_len = MAX;

    for i in 0..3{
        let start_edge = (triangle[i][0] , triangle[i][1]);
        let end_edge   = (triangle[(i+1) % 3][0] , triangle[(i+1) % 3][1]);
        let edge = (start_edge.0, start_edge.1, end_edge.0, end_edge.1);
        let sq_len = get_squared_length(edge);
        if sq_len < shortest_sq_len{
            shortest_sq_len = sq_len;
            shortest_edge  = edge;
        }
    }
    
    //get opposide point of shortest edge
    let c = get_opposide_vertex_of_diagonal(triangle, shortest_edge.0, shortest_edge.1, shortest_edge.2, shortest_edge.3);
    //--

    //interpolate ab = d
    let d = interpolate_points(shortest_edge.0, shortest_edge.1, shortest_edge.2, shortest_edge.3);
    
    //interpolate dc
    let center_point = interpolate_points(d.0, d.1, c.0, c.1); 
    
    
    center_point
}

//---

//
// (Ax,Ay)-------------(Bx, By)
//
fn get_squared_length((ax, ay, bx, by) : (f64, f64, f64, f64)) -> f64{
    let xdiff = ax - bx;
    let y_diff = ay - by;
    let sq_sum = (xdiff * xdiff) + (y_diff*y_diff);

    sq_sum 
}

fn get_opposide_vertex_of_diagonal(triangle : [[f64; 2]; 3], x1: f64,y1: f64,x2: f64,y2: f64) -> (f64,f64){
    for i in 0..3{
        let x = triangle[i][0];
        let y = triangle[i][1];
        if is_part_of_edge(x,y,x1,y1,x2,y2){
            continue;
        }
        return (x,y)
    }
    panic!("Opposide  of Diagonal ({},{})({},{}) of triangle {:?} not found.", x1, y1, x2, y2, triangle);
}

//---


fn get_link_diagonal_points(polygon_lines :  &Vec<(f64,f64,f64,f64)> , link_triangle : [[f64; 2]; 3]) -> (f64,f64,f64,f64,f64,f64){
    //find side without diagonal
    
    for i in 0..3{
        let point_i = link_triangle[i];
        let point_i1 = link_triangle[(i+1)%3];
        let testline = (point_i[0], point_i[1], point_i1[0],  point_i1[1]);
        if !is_a_diagonal(testline, polygon_lines){
            let (ax, ay, cx, cy) = testline;
            let opposide_vertex = link_triangle[(i+2)%3];
            let (bx, by) = (opposide_vertex[0], opposide_vertex[1]);
            return (ax, ay, bx, by ,cx,cy);
        }
    }
    panic!("No border-edge found.")
}

//---


fn is_part_of_edge(x: f64,y: f64,x1: f64,y1: f64,x2: f64,y2: f64)->bool{
    if x == x1 && y == y1{
        return true;
    }
    if x == x2 && y == y2{
        return true;
    }
    false
}

//---


fn interpolate_points(x1: f64, y1: f64, x2: f64, y2 :f64) -> (f64, f64){
    let x = (x1 + x2 )/ (2 as f64);
    let y = (y1 + y2 )/ (2 as f64);
    (x,y)
}

//---


fn get_triangle_type(polygon_lines : &Vec<(f64,f64,f64,f64)> ,  triangle : [[f64; 2]; 3] ) -> TriangleType{
    let mut diagonal_counter = 0;
    for i in 0..3{
        let point_i = triangle[i];
        let point_i1 = triangle[(i+1)%3];
        let testline = (point_i[0], point_i[1], point_i1[0],  point_i1[1]);
        if is_a_diagonal(testline, polygon_lines){
            diagonal_counter = diagonal_counter + 1;
        }
    }
    
    
    match diagonal_counter {
        1 => TriangleType::Ear,
        2 => TriangleType::Link,
        3 => TriangleType::Branch,
        _ => panic!("All three sides are part of the outer-border, so the polygon has to be cyclic")
    }
    
}

//---


fn get_ear_diagonal(polygon_lines : &Vec<(f64,f64,f64,f64)> , ear_triangle : [[f64; 2]; 3] ) -> (f64,f64,f64,f64){
    for i in 0..3{
        let point_i = ear_triangle[i];
        let point_1 = ear_triangle[(i+1) % 3];
        let testline = (point_i[0], point_i[1], point_1[0],  point_1[1]);
        if is_a_diagonal(testline, polygon_lines){
            return testline
        }
    } 
    panic!("No diagonal in ear-triangle found.");
}

//---
#[cfg(test)]
    use super::*;

#[test]
fn get_squared_length_test() {
    assert_eq!(get_squared_length((0f64,0f64,5f64,6f64)), 25f64+36f64);
}
