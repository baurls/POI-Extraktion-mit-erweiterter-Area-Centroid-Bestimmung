/*
    ++ LUKAS BAUR, Bachelorarbeit 2018/2019 ++

    This modul calculates polygon-courses. A detailed description below.


*/


//--------------------------------------Data-Types and Enums------------------------------------------------
pub enum PolygonParsingError{
    DeadEnd(String)

}
enum InnerCicleType{
    NoCycle,
    Cicle(usize)
}




//------------------------------------Öffentliche Funktionen--------------------------------------------------------


 
/*
-- INPUT ----
    Polygongegmentlist <polygon_segment_list> as &Vec<Vec<(i32,i32)>> with
        <polygon_segment_list> = [<segment>, <segment>, ..., <segment>]
        <segment> = [<point>, <point>, ..., <point>]
        <point>     = (x_i, y_i)


-- OUTPUT ----
    Either a Result<Vec<Vec<(i32,i32)>> in form of <polygon_list>  or an PolygonParsingError. Therfore:
        <polygon_list> =  [<polygon>, <polygon>, ..., <polygon>]
        <polygon> = [<point>, <point>, ..., <point>]
        <point>     = (x_i, y_i)
*/
pub fn generate_polygons(paths: &Vec<Vec<(i32,i32)>>) 
    -> Result<Vec<Vec<(i32,i32)>>, PolygonParsingError>{

    //initialize    
    let mut result_polygons  : Vec<Vec<(i32,i32)>> = Vec::new();
    let mut unassigned_paths : Vec<Vec<(i32,i32)>> = Vec::new();

    //early elimination
    for path in paths{
        if is_single_circle(&path){
            result_polygons.push(path.clone());
        }
        else {
            unassigned_paths.push(path.clone());
        }   
    }
    let mut current_walking_stack : Vec<Vec<(i32,i32)>> = Vec::new();

    while true {
        //Preloading of Stack
        //  In Case of
        //      1) initial run or
        //      2) after finding a hole-cycle polygon 
        if current_walking_stack.len() == 0{
            if unassigned_paths.len() == 0{
                break;
            }
            if unassigned_paths.len() == 1{
                return Err(PolygonParsingError::DeadEnd("One non-cyclic element as input left".to_string()));
            }

            assert!(unassigned_paths.len() > 1);

            //add element from unassigned list
            current_walking_stack.push(unassigned_paths.pop().unwrap());
            assert!(unassigned_paths.len() > 0);
        }

        //find end of current Stack
        let current_node : &(i32,i32) = get_ending(&current_walking_stack);


        //search for candidate
        let candidate_index =  match get_candidate_index(current_node, &mut unassigned_paths) {
            Ok(obj) => obj,
            Err(code) => return Err(code)
        };


        //add found candiadte to stack
        let  candidate = unassigned_paths.get(candidate_index).unwrap().clone();
        unassigned_paths.remove(candidate_index);
        stick_accordingly(candidate, &mut current_walking_stack);
        
        println!("{:?}", current_walking_stack);

        //test for hole cycle
        if is_cyclic(&current_walking_stack){
            result_polygons.push(stitch_together(&current_walking_stack, 0));
            current_walking_stack = Vec::new();
            continue;
        }


        //test for sub-cycle
        let cycle_check = check_inner_cycle(&current_walking_stack);
        match cycle_check {
            InnerCicleType::NoCycle => {
                continue;
            },
            InnerCicleType::Cicle(start_index) => {
                result_polygons.push(stitch_together(&current_walking_stack, start_index));
                current_walking_stack = current_walking_stack[0..start_index].to_vec();
                continue;
                }
        }

    }
    Ok(result_polygons)
}





//--------------------------------------Hilfsfunktionen-----------------------------------------------------



//for Stack a->b->c->d->e->b 
//Index     0  1  2  3  4  5
//Returns 1
fn check_inner_cycle(current_walking_stack : &Vec<Vec<(i32,i32)>>) -> InnerCicleType{
    let last = get_ending(current_walking_stack).clone();
    let mut index = 0;
    for stack_entry in current_walking_stack{
        if stack_entry.first().unwrap().clone() == last{
            return InnerCicleType::Cicle(index);
        }
        index = index +1;
    }
    InnerCicleType::NoCycle
}



//for startindex=0
//Input: [1, 2, ... , 7, 8] [8, 9,..]...[ ..., 44, 45][45, ...]
//Output [1, 2, ... , 7, 8, 9,..., .., 44, 45, ...]

//for startindex=2
//Input: [1,2,3][3,4,5][5,6,7,8][8,9,10][10,11,5]
//Index     0      1       2        3
//Output [5,6,7,8,9,10,11,5]
fn stitch_together(segment_list : &Vec<Vec<(i32,i32)>>, startindex :usize) -> Vec<(i32,i32)>{
    let mut ist_first_run = true;
    let mut stitched_segments : Vec<(i32,i32)> = Vec::new();
    let mut last_end : (i32,i32) = (0,0);
    let mut current_index = 0;

    for segment in segment_list {

        if current_index < startindex{
            current_index = current_index +1;
            continue;
        }
        if ist_first_run{
            stitched_segments = segment.clone();
            ist_first_run = false;
        }
        else {
            assert_eq!(last_end, segment.first().unwrap().clone());
            stitched_segments.extend_from_slice(&segment[1..segment.len()]);
        }
        last_end = stitched_segments.last().unwrap().clone();   
        current_index = current_index +1;
    }

    stitched_segments
}



//current stack:   [..] .. [.. , e]
//Input: [e, a, b, c, d, ...]   Output:  [..] .. [.. , e][e, a, b, c, d, ..]
//Input: [..., w, x, y, z, e]   Output:  [..] .. [.. , e][e, z, y, x, w, ..]
fn stick_accordingly(mut new_segment :  Vec<(i32,i32)>, current_path: &mut Vec<Vec<(i32,i32)>>){
    let (_, end) =  get_single_path_endpoints(&new_segment);
    if end == get_ending(current_path){
        new_segment.reverse();
    }

    assert_eq!(new_segment.first().unwrap(), get_ending(&current_path));
    current_path.push(new_segment);

}



//Return index inside unassigned segments, which segment fits with the current endpoint.
fn get_candidate_index(end_node : & (i32,i32), path_list : &Vec<Vec<(i32,i32)>>) ->  Result<usize,PolygonParsingError>{
   let mut index = 0;
   for current_path in path_list{
        if segment_fits(end_node, current_path){
            return Ok(index);
        }
        index = index +1;
    }
    let error_description = "Cannot extract Polygon: At least one path segment is a dead end".to_string();
    Err(PolygonParsingError::DeadEnd(error_description))
}



// true iff 
//Stack {a -> b -> c} and Element: {c -> ..} or {.. -> c}
fn segment_fits(current_stack_end : &(i32,i32),  possible_path_segment :  &Vec<(i32,i32)>) -> bool{
    let (start, end) = get_single_path_endpoints(possible_path_segment);
    if start == current_stack_end || end == current_stack_end{
        return true
    }
    false
}



// { [(a,b), ... , (x,y)] [(x,y), ... , (p,q)]   ...    [(r,s), ... , (x,y)] }
// 
//Returns (a,b)  
fn get_beginning(paths: &Vec<Vec<(i32,i32)>>) -> &(i32,i32){
    paths.first().expect("Path ist empty").first().expect("First Pathsegment ist empty")
}



// { [(a,b), ... , (x,y)] [(x,y), ... , (p,q)]   ...    [(r,s), ... , (x,y)] }
// 
//Returns (x,y)  
fn get_ending(paths: &Vec<Vec<(i32,i32)>>) -> &(i32,i32){
    paths.last().expect("Path ist empty").last().expect("Last Pathsegment ist empty")
}



//[(a,b), ... , (x,y)] Returns: (a,b) and (x,y)  
fn get_single_path_endpoints(path : &Vec<(i32,i32)>) -> (&(i32,i32),&(i32,i32)) {
    let first = path.first().unwrap();
    let last =  path.last().unwrap();
    (first, last)
}



//[(a,b), ... , (x,y)] True iff (a,b) == (x,y)  
fn is_single_circle(path : &Vec<(i32,i32)>)-> bool{
    let (first, last) = get_single_path_endpoints(path);
    if first == last{
        return true;
    }
    false
}



// { [(a,b), ... , (x,y)] [(x,y), ... , (p,q)]   ...    [(r,s), ... , (x,y)] }
// 
//True iff (a,b) == (x,y)  
fn is_cyclic(paths: &Vec<Vec<(i32,i32)>>)->bool{
    let first = paths.first().unwrap().first().unwrap();
    let last = paths.last().unwrap().last().unwrap();
    if last == first{
        return true
    }
    return false
}


    