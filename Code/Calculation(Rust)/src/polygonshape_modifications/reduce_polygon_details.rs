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
pub fn reduce_shape_details(polygon : &Vec<(f64,f64)>, variant: ReductionType) -> Vec<(f64,f64)>{
    match variant {
        ReductionType::Naive => reduce_shape_details_naively(polygon),
    }
}
pub fn reduce_family_shape_details(polygon_familiy : &Vec<Vec<(f64,f64)>>, variant: ReductionType) -> Vec<Vec<(f64,f64)>>{
    let mut new_polygon_familiy : Vec<Vec<(f64,f64)>>  = Vec::new();
    for polygon in polygon_familiy{
        let new_polygon = reduce_shape_details(polygon, ReductionType::Naive);
        new_polygon_familiy.push(new_polygon);
    }

    new_polygon_familiy
}

//---------------------------------top level private functions------------------------------------------

fn reduce_shape_details_naively(polygon : &Vec<(f64,f64)>) -> Vec<(f64,f64)>{
    if polygon.len() < 10{
        return polygon.clone();
    }
    let average = get_average_length(polygon);
    let lower_bound = average ;
    let mut new_polygon : Vec<(f64,f64)> = Vec::new();
    let mut skip_current = false;
    for i in 0..polygon.len()-1{
        if skip_current{
            skip_current = false;
            continue;
        } 
        let current = polygon[i];
        let next = polygon[i+1];
        
        if get_length(current, next) < lower_bound{
            skip_current = true;
        }
        new_polygon.push(current);
    }
    //if not cyclic anymore: add first to the last
    if new_polygon.last().unwrap() != new_polygon.first().unwrap(){
        new_polygon.push(new_polygon[0]); 
    }

    new_polygon
}

//---------------------------------private functions----------------------------------------------------
fn get_average_length(polygon : &Vec<(f64,f64)>) -> f64{
    assert!(polygon.len() > 3); //min. 3 different points

    let mut average_len = 0f64;
    for i in 0..polygon.len() -1{
        let current = polygon[i];
        let next = polygon[i+1];
        let len = get_length(current, next);
        average_len = average_len + len;
    }
    average_len = average_len / (polygon.len() -1) as f64;

    average_len
}
//---------------------------------private functions----------------------------------------------------
fn get_length(a: (f64,f64), b: (f64,f64)) -> f64{
   let dx = a.0 - b.0;
   let dy = a.1 - b.1;

   let len_2 = dx * dx + dy*dy;
   let len = len_2.sqrt();

   len
}


//---------------------------------tests------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_length() {
        let a = (4f64, 18f64);
        let b = (9f64, 24f64); 
        let solution = 7.810249676f64;
        let diff = (solution - get_length(a,b));

        assert!(diff.abs() < 0.000001);
    }

    #[test]
    fn test_get_average_length(){
        let test_vector = vec![(1f64, 1f64), (4f64, 2f64), (5f64, 7f64), (0f64, 10f64), (1f64, 1f64)];
        //println!("{}", get_average_length(&test_vector));
        let solution = 5.786908552f64;
        let diff = (solution - get_average_length(&test_vector));
        assert!(diff.abs() < 0.000001);
    }
}