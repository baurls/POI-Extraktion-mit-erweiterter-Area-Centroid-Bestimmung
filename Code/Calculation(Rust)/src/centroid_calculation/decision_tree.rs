/**************************************************************************************
*  ++ LUKAS BAUR, Bachelorarbeit 2018/2019 ++
*
*  This module classifies the poylgon-structure and calls the according 
*  calculation-function.
*  This module is based on an the clean_polygon_family module. 
*                     ______________________       ______________________
*                    |                      |     |                      | 
*  polygon_family -> | clean_polygon_family | ->  |     decision_tree    | -> (x,y) 
*                    |______________________|     |______________________| 

* First execute the clean_polygon_family, afterwards apply this module 
*    to the results
*
***************************************************************************************/






//-------------------------------Dokumentation----------------------------------------------------------
/*
INPUT
    <polygon_family_collection>  = [<polygon_family>, <polygon_family>, ..., <polygon_family>]
WHERE
    <polygon_family> = [<outer_polygon>, <inner_polygon>, <inner_polygon>, ..., <inner_polygon>]
    <xxx_polygon>>   = [(x1, y1), (x2, y2), ..., (xn, yn), (x1, y1)]

OUTPUT
    <result> = ENUM_ERROR or (x,y)

*/


//-------------------------------Import----------------------------------------------------------------
use super::simple_algorithms::{get_geometrical_center,get_polygonfamily_area};
use super::point_of_inaccessibility::{get_point_of_inaccessibility, point_to_polygon_dist};
use crate::polygon_check::PolygonShapeState;
use crate::polygon_check::check_polygon;
use crate::polygon_check::intersection_check::IntersectionType;
use crate::centroid_calculation::CenterCalculationErrorType;
use crate::centroid_calculation::skeleton_centerpoint::centrality_scores_calculation::{get_most_central_points, CentralityVariant, SearchingMethod, Weights};
use crate::centroid_calculation::skeleton_centerpoint::triangulation::{Polygontriangulation, generate_polygon_triangulation};
use crate::centroid_calculation::skeleton_centerpoint::skeleton_graph_generation::{generate_polygon_skeleton_graph, SkeletonType, Skeletongraph};



//------------------------------Publics----------------------------------------------------------------

pub fn get_center_point(cleaned_polygon_family_collection : &Vec<Vec<Vec<(f64,f64)>>>) -> Result<(f64,f64), CenterCalculationErrorType>{
    if is_empty(cleaned_polygon_family_collection){
        return Err(CenterCalculationErrorType::EmptyStrucure);
    }

    if is_composite(cleaned_polygon_family_collection){
        let largest_family = get_largest_family(cleaned_polygon_family_collection);
        println!("Area composed, take largest area.");
        return get_center_of_single_family(&largest_family);
    }

    assert!(cleaned_polygon_family_collection.len() == 1);
    let familiy = &cleaned_polygon_family_collection[0];
    return get_center_of_single_family(familiy)

}
//------------------------------Top-Level-Private----------------------------------------------------------------
fn get_center_of_single_family(familiy : &Vec<Vec<(f64,f64)>>) -> Result<(f64,f64), CenterCalculationErrorType>{
    //it has only one outer and >= 0 inner.
    
    if has_holes(familiy){
        println!("Single area has holes. Use PoI-method");
        //it is a singe outer with multiple holes
        let precision = 0.0000001f64;
        let (x,y,_) = get_point_of_inaccessibility(familiy, precision);
        let point = (x,y);
        return Ok(point);
    }
    else{
        //it is a singe outer without holes
        assert!(familiy.len() == 1);
        let single_outer = &familiy[0]; 
        let geometric_center = get_geometrical_center(familiy);
        if is_convex(single_outer){
            println!("Single area has no holes and is convex. Use mass center");
            return Ok(geometric_center);
        }
        else{
            //get best approximation
            println!("Single area has no holes but is concave. Use best Candidate");
            let candidates = get_candidates(single_outer);
            return Ok(get_best(&candidates, geometric_center));
        }
    }
}

//------------------------------Privates----------------------------------------------------------------
fn get_candidates(single_polygon : &Vec<(f64,f64)>) -> Vec<(f64,f64)>{
    let mut return_list : Vec<(f64,f64)> = Vec::new();
    
    let mut polygonfamily : Vec<Vec<(f64,f64)>> = Vec::new();
    polygonfamily.push(single_polygon.clone());

    //Point of Inaccessibility
    let precision = 0.0000001f64;
    let (x,y,_) = get_point_of_inaccessibility(&polygonfamily, precision);
    let poi = (x,y);
    println!("Candidate 1 (PoI): {:?}", poi);
    return_list.push(poi);

    make_greater(&mut polygonfamily); 
    
    //Skeleton-Centrality
    let triangulation : Polygontriangulation = generate_polygon_triangulation(&polygonfamily);
    let skeletongraph : Skeletongraph = generate_polygon_skeleton_graph(&triangulation, SkeletonType::FmiCenterBased);
        //Betweenness-Centrality
    let betweenness_method = &CentralityVariant::Betweenness(SearchingMethod::Disjkstra1ToAll);
    let mut betweenness_points : Vec<(f64, f64)>  = get_most_central_points(&skeletongraph, betweenness_method);
    make_smaller(&mut betweenness_points);
    return_list.extend(&betweenness_points);
    println!("Candidate 2 (Between): {:?}", betweenness_points);

        //Closeness-Centrailty
    let closeness_method = &CentralityVariant::Closeness(SearchingMethod::Disjkstra1ToAll, Weights::CoverLength);
    let mut closeness_points : Vec<(f64, f64)>  = get_most_central_points(&skeletongraph, closeness_method);
    println!("Candidate 3 (Closeness): {:?}", closeness_points);
    make_smaller(&mut closeness_points);
    return_list.extend(&closeness_points);

    return_list
}

fn make_greater(polygonfamily : &mut Vec<Vec<(f64,f64)>>){
    for polygon in polygonfamily{
        for tuple in polygon{
            tuple.0 = tuple.0 * 10000000f64;       
            tuple.1 = tuple.1 * 10000000f64;        
        }
    }
}

fn make_smaller(polygon : &mut Vec<(f64,f64)>){
    for tuple in polygon{
            tuple.0 = tuple.0 * 0.0000001f64;       
            tuple.1 = tuple.1 * 0.0000001f64;        
        }
}

fn get_best(candidates : &Vec<(f64,f64)>, geometric_center : (f64,f64)) ->  (f64,f64){
    assert!(candidates.len() > 0);

    print!("candiaten: {:?}", candidates);

    let (x1,y1) = candidates.first().unwrap();
    let mut best = (*x1, *y1);
    let mut smallest_distance = get_squared_distance(best, geometric_center);
    
    for point in candidates{
        let current = point.clone();
        let distance = get_squared_distance(current.clone(), geometric_center);
    
        //update, if better
        if distance < smallest_distance{
            smallest_distance = distance;
            best = current;
        }
    }
    
    best
}

fn get_squared_distance(a : (f64,f64), b : (f64,f64)) -> f64{
    let x_diff = a.0 - b.0;
    let y_diff = a.1 - b.1;
    
    x_diff * x_diff + y_diff * y_diff
}

fn is_composite(polygon_family_collection : &Vec<Vec<Vec<(f64,f64)>>>) -> bool{
    if polygon_family_collection.len() > 1{
        return true;
    }
    false
}


fn is_empty(polygon_family_collection : &Vec<Vec<Vec<(f64,f64)>>>) -> bool{
    if polygon_family_collection.len() == 0{
        return true;
    }
    false
}


fn has_holes(polygon_family :  &Vec<Vec<(f64,f64)>>) -> bool{
    if polygon_family.len() > 1{
        return true;
    }
    false
}

//[tested]        
fn is_convex(polygon : &Vec<(f64,f64)>) -> bool{
    if polygon.len() < 4{
        return false; //two-points-polygon can't be convex. It has to be at least a triangle.
    }    
    let mut last_direction = 0f64;
    let mut is_first_round = true;
    for i in 0..polygon.len()-1{
        let current_point = polygon[i];
        let next_point = polygon[(i+1) % polygon.len()];


        let dx = next_point.0 - current_point.0;
        let dy = next_point.1 - current_point.1;


        let over_next_point = polygon[(i+2) % polygon.len()];
        let new_direction =  (over_next_point.0 * dy) - (over_next_point.1* dx) + (dx * current_point.1) - (dy * current_point.0);

        println!("{}", new_direction);
        if is_first_round {
            is_first_round = false;
            //set as start-value
            last_direction = new_direction;            
        }
        else {
            if (new_direction > 0f64 && last_direction < 0f64) || (new_direction < 0f64 && last_direction > 0f64 ){
                return false;
            }
        }
    }
    
    true
}

fn check_family(polygon_family : &Vec<Vec<(f64,f64)>>) ->  PolygonShapeState{
    if polygon_family.len() == 0{
        return PolygonShapeState::EmptyStrucure;
    }
    for polygon in polygon_family{
        let state = check_polygon(&polygon);
        match state{
            PolygonShapeState::Ok => continue,
            _ => return state,
        };
    }

    PolygonShapeState::Ok
}


//[tested]  
// get the point from all candidates which has the largest (positive) distance to the border      
fn get_most_central_point(method_point_vector : Vec<(&str, (f64, f64))>, polygon_family : &Vec<Vec<(f64,f64)>>) -> Result<(f64,f64), CenterCalculationErrorType>{
    assert!(method_point_vector.len() > 0);

    let (first_method, (x1,y1)) = method_point_vector.first().unwrap();
    let mut best = (*x1, *y1);
    let mut largest_distance = point_to_polygon_dist(*x1, *y1, polygon_family);
    let mut best_method = first_method;
    for (method, (x,y)) in &method_point_vector{
        let distance = point_to_polygon_dist(*x, *y, polygon_family);
    
        //update, if better
        if distance > largest_distance{
            largest_distance = distance;
            best = (*x,*y);
            best_method = &method;
        }
    }
    if largest_distance < 0f64{
        return Err(CenterCalculationErrorType::PointOutside);
    }
    println!("Best method: {} (distance: {})", best_method, largest_distance);
    Ok(best)
}

//[tested]        
fn get_largest_family(cleaned_polygon_family_collection : &Vec<Vec<Vec<(f64,f64)>>>) -> Vec<Vec<(f64,f64)>>{
    assert!(cleaned_polygon_family_collection.len() > 1);
    let mut best_family_index = 0;
    let mut largest_area = 0f64;
    for i in 0..cleaned_polygon_family_collection.len(){
        let familiy = &cleaned_polygon_family_collection[i];
        let area = get_polygonfamily_area(familiy);
        if area > largest_area{
            largest_area = area;
            best_family_index = i;
        }
    }
    
    cleaned_polygon_family_collection[best_family_index].clone()
}


//------------------------------Test---------------------------------------------------------------------------------------------


#[test]
fn is_convex_test() {
    let testvec1 = vec![(0f64, 0f64), (5f64, 0f64),(5f64, 5f64), (3f64, 6f64), (0f64, 2f64), (0f64, 0f64)];
    assert!(is_convex(&testvec1));
    let testvec2 = vec![(0f64, 0f64), (5f64, 0f64),(5f64, 5f64), (3f64, 6f64), (0f64, 2f64), (0f64, 1f64) ,(0f64, 0f64)];
    assert!(is_convex(&testvec2));
    let testvec3 = vec![(0f64, 0f64), (5f64, 0f64),(5f64, 5f64), (3f64, 6f64), (3f64,1.5f64) ,(0f64, 2f64), (0f64, 1f64) ,(0f64, 0f64)];
    assert!(!is_convex(&testvec3));
    let testvec4 = vec![(0f64, 0f64), (4f64, 0f64),(5f64, 3f64), (5f64, 4f64), (7f64,4.5f64) ,(8f64, 10f64), (0f64, 10f64) ,(0f64, 0f64)];
    assert!(!is_convex(&testvec4));
    let testvec5 = vec![(0f64, 0f64), (4f64, 0f64), (4f64, 4f64), (4.001f64, 100f64), (0f64, 100f64), (0f64, 0f64)];
    assert!(!is_convex(&testvec5));
}


#[test]
fn get_most_central_point_test(){
    let outer = vec![(0f64, 0f64), (10f64, 0f64),(8f64, 10f64), (1f64, 8f64), (0f64, 0f64)];
    let inner1 = vec![(1f64, 1f64), (2f64, 1f64),(1.5f64, 3f64), (1f64, 1f64)];
    let inner2 = vec![(5f64, 6f64), (5f64, 7f64),(4f64, 7f64), (4f64, 6f64), (5f64, 6f64)];
    let familiy = vec![outer, inner1, inner2];
    
    let method_point_vector = vec![("Mittelpunkt", (4f64, 4f64)), ("Skel1", (4.5f64, 1.5f64)), ("Skel2", (5.5f64, 2f64)), ("Skel3", (6.5f64, 3f64)), ("Skel4", (7.5f64, 4f64)), ("Skel5", (7.5f64, 5.5f64))];


    let result = get_most_central_point(method_point_vector, &familiy);
    println!("{:?}", result);
}

#[test]
fn get_largest_family_test(){
    let polygon1 = vec![(0f64, 0f64),(10f64, 0f64),(10f64, 10f64),(0f64, 10f64),(0f64, 0f64)];
    let polygon2 = vec![(1f64, 1f64),(3f64, 1f64),(3f64, 3f64),(1f64, 3f64),(1f64, 1f64)];
    let polygon3 = vec![(5f64, 5f64),(6f64, 5f64),(6f64, 6f64),(5f64, 6f64),(5f64, 5f64)];
    
    let poly_fam1 = vec![polygon1.clone(), polygon2.clone()];
    let poly_fam2 = vec![polygon1.clone(), polygon3.clone()];
    let poly_fam3 = vec![polygon1.clone()];

    let fam_collection = vec![ poly_fam1.clone(), poly_fam2.clone(), poly_fam3.clone()];
    assert_eq!(poly_fam3.clone(), get_largest_family(&fam_collection)); 
}

#[test]
fn get_squared_distance_test(){
    assert_eq!(41f64, get_squared_distance((0f64, 0f64), (4f64, 5f64)));
    assert_eq!(41f64, get_squared_distance((1f64, 2f64), (5f64, 7f64)));
}