/****************************************************************************
*  ++ LUKAS BAUR, Bachelorarbeit 2018/2019 ++
*
*  This module reduces the shape details of a polygon or poylgonfamily. It uses
*  morphological erosion 
*
*****************************************************************************/



//-------------------------------Import----------------------------------------------------------------

//from local

//from external

//-------------------------------Constants--------------------------------------------------------------


//------------------------------Structs & Co------------------------------------------------------------
#[derive(Debug)]
pub enum ReductionType {
    Naive,
}


//---------------------------------public API-function---------------------------------------------------

/*
INPUT:  <polygon> = [(x,y), (x,y), ..., (x,y)]
        x,y = f64

OUTPUT: <polygon> = [(x,y), (x,y), ..., (x,y)]
*/
pub fn reduce_shape_details(polygon : &Vec<(f64,f64)>) -> Vec<(f64,f64)>{
    
}
pub fn reduce_family_shape_details(polygon_familiy : &Vec<Vec<(f64,f64)>>) -> Vec<Vec<(f64,f64)>>{
    let mut new_polygon_familiy : Vec<Vec<(f64,f64)>>  = Vec::new();
    for polygon in polygon_familiy{
        let new_polygon = reduce_shape_details(polygon);
        new_polygon_familiy.push(new_polygon);
    }

    new_polygon_familiy
}

//---------------------------------private functions----------------------------------------------------
