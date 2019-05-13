/****************************************************************************
*  ++ LUKAS BAUR, Bachelorarbeit 2018/2019 ++
*
*  This module cleans the input as a preprocessing of the polygon decision
*  module.
*                     ______________________       ______________________
*                    |                      |     |                      | 
*  polygon_family -> | clean_polygon_family | ->  |     decision_tree    | -> (x,y) 
*                    |______________________|     |______________________| 

*****************************************************************************/
mod tests;

//------------------------------Structs----------------------------------------------------------------
use crate::polygon_check::intersection_check::IntersectionResult;
use crate::polygon_check::intersection_check::IntersectionType;
use crate::polygon_check::PolygonShapeState;
use crate::polygon_check::check_polygon;

use std::collections::HashSet;

#[derive(Debug, PartialEq)]
enum PolygonCleaningResult {
    Original,
    Repaired(Vec<(f64,f64)>),
    Empty
}
#[derive(Debug, PartialEq)]
pub enum FamilyCleaningResult {
    Succeeded(Vec<Vec<(f64,f64)>>),
    Failed
}
//------------------------------Publics----------------------------------------------------------------
//[tested]
pub fn clean_and_repair_family(polygon_family : &Vec<Vec<(f64,f64)>>) -> FamilyCleaningResult{
    let mut is_outer_polygon = true;
    let mut cleaned_polygonfamily : Vec<Vec<(f64,f64)>> = Vec::new();
    for polygon in polygon_family{
        let cleaned_polygon = reapir_or_clean_polygon(&polygon);
             
        match cleaned_polygon{
            PolygonCleaningResult::Empty => {
                //Structure is damaged. Can't append to List
                //Exit if its the outer-polygon
            if is_outer_polygon == true{
                    return FamilyCleaningResult::Failed;   
                }
            },
            PolygonCleaningResult::Original => {
                //Polygon is already OK
                cleaned_polygonfamily.push(polygon.clone());
            },
            PolygonCleaningResult::Repaired(repaired_polygon) =>{
                //Set new Polygon to List
                cleaned_polygonfamily.push(repaired_polygon);
            }
        }
        is_outer_polygon = false;

  }

  FamilyCleaningResult::Succeeded(cleaned_polygonfamily)
}

//------------------------------Top-Level-Private----------------------------------------------------------------
//[tested]
fn reapir_or_clean_polygon(polygon: &Vec<(f64,f64)>) -> PolygonCleaningResult{
    let first_try_result = check_polygon(polygon);
    match first_try_result{
        PolygonShapeState::Ok => return PolygonCleaningResult::Original,
        PolygonShapeState::EmptyStrucure => return PolygonCleaningResult::Empty,
        PolygonShapeState::LessThan4Points => return PolygonCleaningResult::Empty,
        PolygonShapeState::OnlySinglePoint => return PolygonCleaningResult::Empty,
        PolygonShapeState::PointsAreNotUnique => return handlePointsNotUnique(polygon),
        PolygonShapeState::NotClosed => return handlePolygonNotClosed(polygon),
        PolygonShapeState::SelfIntersecting(intersectionType) => return handlePolygonIntersection(intersectionType, polygon),
    }
}
//---------------------------------private---------------------------------
fn handlePointsNotUnique(polygon: &Vec<(f64,f64)>) -> PolygonCleaningResult{
    //Here could be some more magic implemented. 
    //For example: A eight-loop could be changed into two polygons.
    //  ->  but actually -it's not as trivial as it sounds 
    //      like (think of holes inside the loop which has
    //      to be refereneced correctly) 
    PolygonCleaningResult::Empty
}


fn handlePolygonNotClosed(polygon: &Vec<(f64,f64)>) -> PolygonCleaningResult{
    //try to close the path naively
    assert!(polygon.len() > 0); //because of polygon check in module "polygon_check"
    let first_point = polygon[0];

    //new poolygon is closed old polygon.
    let mut polygon_candidate = polygon.clone();
    polygon_candidate.push(first_point);

    let second_try = check_polygon(&polygon_candidate);
    if second_try == PolygonShapeState::Ok{
        return PolygonCleaningResult::Repaired(polygon_candidate);
    }
    //not possible to fix 
    PolygonCleaningResult::Empty
}

fn handlePolygonIntersection(intersectionType : IntersectionType, polygon : &Vec<(f64,f64)>) -> PolygonCleaningResult{
    //here: also a lot of magic functions could be called-
    //-> not easy

    PolygonCleaningResult::Empty
}


