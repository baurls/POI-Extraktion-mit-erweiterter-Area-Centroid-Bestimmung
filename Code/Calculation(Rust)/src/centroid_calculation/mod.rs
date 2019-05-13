/**************************************************************************************
*  ++ LUKAS BAUR, Bachelorarbeit 2018/2019 ++
*
*  This module function as a high-level API for the center-point calculation.
*  It works in the following way: 
*       1) Cleans and repairs the input
*       2) Feeds the result into the decision-makeing-module
*       3) The module then calles the corresponding calculation-unit. 
*    
*
*    ____this module___________________________________________________________________________
*   |                    ______________________       ______________________                  |
*   |                    |                      |     |                      |                | 
*   |  polygon_family -> | clean_polygon_family | ->  |     decision_tree    | -> (x,y)       |
*   |                    |______________________|     |______________________|                |
*   |                                                     |             ^                     |
*   |                                                     |             |                     |
*   |                                                  ___v_____________|____                 | 
*   |                                                |                      |                 |
*   |                                                | corresponding module |                 |
*   |                                                |______________________|                 |
*   |_________________________________________________________________________________________|
*                                 
*
*
******************************************************************************************/


//-------------------------------Import----------------------------------------------------------------
pub(crate) mod point_of_inaccessibility;
pub(crate) mod simple_algorithms;
pub(crate) mod skeleton_centerpoint;
pub(crate) mod decision_tree;

use crate::clean_and_repair_family::FamilyCleaningResult;
use crate::clean_and_repair_family::clean_and_repair_family;

//------------------------------Structs------------------------------------ ----------------------------
#[derive(Debug, PartialEq)]
pub enum CenterCalculationErrorType{
    EmptyStrucure,
    PointOutside,
}


pub fn clean_and_get_center_point(polygon_family_collection : &Vec<Vec<Vec<(f64, f64)>>>) -> Result<(f64, f64), CenterCalculationErrorType>{
    let mut cleaned_family_collection : Vec<Vec<Vec<(f64, f64)>>> = Vec::new();
    //clean input
    for polygon_family in polygon_family_collection{
        let cleaning_result = clean_and_repair_family(polygon_family);
       
        match cleaning_result {
            FamilyCleaningResult::Succeeded(cleaned_familiy) => cleaned_family_collection.push(cleaned_familiy),
            FamilyCleaningResult::Failed => {},
        }
    }
    
    if cleaned_family_collection.len() ==  0 {
        return Err(CenterCalculationErrorType::EmptyStrucure);
    }


    //get center point for cleaned input
    let center_calculation_result = decision_tree::get_center_point(&cleaned_family_collection);
    match center_calculation_result {
        Ok(point) => return Ok(point),
        Err(error_type) => Err(error_type),
    }
}

