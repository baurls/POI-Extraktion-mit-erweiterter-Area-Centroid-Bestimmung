/****************************************************************************
*  ++ LUKAS BAUR, Bachelorarbeit 2018/2019 ++
*
*  This module checks the poylgon properties. 
*
*****************************************************************************/
pub mod intersection_check;
mod tests;

//------------------------------Structs----------------------------------------------------------------
#[derive(Debug, PartialEq)]
pub enum PolygonShapeState{
    Ok,                                 //all right
    EmptyStrucure,                      //no points
    OnlySinglePoint,                    //only [(x,y)]
    NotClosed,                          //first =/= last
    LessThan4Points,                    //a face is at leat a trianle (3 Points + startpoint)
    PointsAreNotUnique,                 //there's another loop: [a, b, c, d, b, e, f , a]
    SelfIntersecting(IntersectionType)  //there's a intersectin of at least two lines
}

//-------------------------------Import----------------------------------------------------------------
use intersection_check::IntersectionResult;
use intersection_check::IntersectionType;

use std::collections::HashSet;


//------------------------------Publics----------------------------------------------------------------
//[Tested]
pub fn check_polygon(polygon_ring : &Vec<(f64,f64)>) -> PolygonShapeState{
    if polygon_ring.len() == 0{
        return PolygonShapeState::EmptyStrucure;
    }
    if polygon_ring.len() == 1{
        return PolygonShapeState::OnlySinglePoint;
    }
    if polygon_is_closed(polygon_ring) == false{
        return PolygonShapeState::NotClosed;
    }
    if polygon_ring.len() < 4{
        return PolygonShapeState::LessThan4Points;
    }
    if all_points_are_unique(polygon_ring) == false{
        return PolygonShapeState::PointsAreNotUnique;
    }
    let check_intersecting = intersection_check::polygon_is_self_intersecting(polygon_ring);
    match check_intersecting{
        IntersectionResult::HasNoIntersection => {},
        IntersectionResult::HasIntersection(intersection_type) => return PolygonShapeState::SelfIntersecting(intersection_type)
    };

    PolygonShapeState::Ok
}

//------------------------------Top-Level-Private----------------------------------------------------------------

//       o - - - - - - - - - - - - o                                                                         
//       .                           .                                              
//       .                             .                                            
//       .                               .                                          
//       o p1                             o                                        
//                                       .                                         
//                      m               .                                        
//                      o - - -  - - - o                                        
//                                                                                

//[p1, p2, p3, ..., pn, pm] : p1 = pm 
//[Tested]
fn polygon_is_closed(polygon_ring : &Vec<(f64,f64)>)-> bool{
    assert!(polygon_ring.len() > 1);
    let first = polygon_ring[0];
    let last  = polygon_ring[polygon_ring.len()-1];
    if first == last{
        return true;
    }

    false
}



//find a loop:     c - - - d
//                 .    .                  
//                 .  .                     
//    a - - - - -  b                              
//      .          .                         
//        .        .                         
//         f - - - e                        

//[p1, p2, p3, ..., pn, p1]   1 <= i<= n: pi =/= pj
//[Tested]
fn all_points_are_unique(polygon_ring : &Vec<(f64,f64)>) -> bool{
    let mut unique_points : HashSet<String> = HashSet::new();
    for i in 0..polygon_ring.len()-1{
        let point = polygon_ring[i];
        let pointstring = format!("{:?}", point);
        match unique_points.insert(pointstring) {
            true  => {},
            false => return false
        };
    }

    true
}


