/****************************************************************************
*  ++ LUKAS BAUR, Bachelorarbeit 2018/2019 ++
*
*  This module calls the local file "polygonizer" to obtain
*  a tree-like structure of polygons out of a segment-list 
*
*****************************************************************************/
extern crate num;

//-------------------------------Import----------------------------------------------------------------
use num::Num;

use std::process::{Command, Stdio};
use std::io::Write;
use std::fmt::{Display, Debug};
use std::str::FromStr;
use std::collections::HashMap;


mod tests;

//------------------------------Consts----------------------------------------------------------------
const PATH_TO_SCRIPT : &str =  "./src/polygonizer/polygonize";



//------------------------------Publics----------------------------------------------------------------
pub fn get_polygons_from_segments<T:Num + Display+FromStr+Debug+Clone+Copy>(linesegments : Vec<(T, T,  T, T)>) -> Vec<Vec<Vec<(i32, i32)>>>{

    let result_string  = call_polygonizer_script(linesegments);
    //println!("Result_string: \n{}", result_string);
    let mut result_vector : Vec<Vec<i32>> = convert_to_vector_from_string(result_string);
    //println!("Result_vector: \n{:?}", result_vector);

    let mut familiy_collection : Vec<Family> = Vec::new();
    familiy_collection = get_families_from_tree(result_vector);

    let mut familiy_collection_as_multi_vector : Vec<Vec<Vec<(i32, i32)>>> = Vec::new();
    familiy_collection_as_multi_vector = convert_to_multivector(familiy_collection);


    //println!("Fertiges Familien-Vec-Objekt: {:?}\n",familiy_collection_as_multi_vector );
    familiy_collection_as_multi_vector
}


pub fn call_polygonizer_script<T:Num + Display + FromStr+Debug+Clone+Copy>(linesegments : Vec<(T, T,  T, T)>) -> String{
    let passing_argument = convert_stringlist_from_segments(linesegments);

    
    //start poligonize script
    let mut child = Command::new(PATH_TO_SCRIPT)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Das Script kann nicht ausgeführt werden. Nicht vorhanden?");
    

    //pass point-polygon-list into process
    let stdin = child.stdin.as_mut().expect("failed to get stdin");
    stdin.write_all(passing_argument.as_bytes()).expect("failed to write to stdin");
    
    //read output
    let output = child
        .wait_with_output()
        .expect("failed to wait on child");


    (String::from_utf8_lossy(&output.stdout)).to_string()
}



//------------------------------Private----------------------------------------------------------------
fn convert_stringlist_from_segments<T:Num+ Display+FromStr+Debug+Clone+Copy>(linesegments : Vec<(T, T,  T, T)>) -> String{
    let mut plain_string = String::new();
    let mut first_run = true;
    for (a,b,r,s) in linesegments{
        if first_run == true{
            first_run = false;
            plain_string = format!("{} {} {} {}", a, b, r, s);
        }
        else {
            plain_string = format!("{}\n{} {} {} {}", plain_string, a, b, r, s);
        }
    }

    plain_string
}

fn convert_to_vector_from_string(input_string : String) -> Vec<Vec<i32>>{
    let lines_array = input_string.split("\n");
    let mut result_vector : Vec<Vec<i32>> = Vec::new();
    for line in lines_array{
        let trimed = line.trim();
        let entries = trimed.split(" ");
        let mut line_entry : Vec<i32>  = Vec::new();
        for entry in entries{
            match i32::from_str(entry){
                Ok(t) => line_entry.push(t),
                Err(_) => {}
            };
        }
        result_vector.push(line_entry);
    }

    result_vector
}


fn convert_to_multivector(families : Vec<Family>) -> Vec<Vec<Vec<(i32, i32)>>>{
    let mut result_vec : Vec<Vec<Vec<(i32, i32)>>> = Vec::new();
    for family in families{
        let mut familiy_vec : Vec<Vec<(i32, i32)>> = Vec::new();
        familiy_vec.push(get_as_pointlist(family.outer));
        for inner_kontur in family.inner{
            familiy_vec.push(get_as_pointlist(inner_kontur));
        }
        result_vec.push(familiy_vec);
    }

    
    result_vec
}

fn get_as_pointlist(single_points : Vec<i32>) -> Vec<(i32, i32)>{
    assert!(single_points.len() % 2 == 0); //Es gibt immer eine gerade Anzahl an Punkten
    
    let mut point_list : Vec<(i32, i32)> = Vec::new();
    for i in (0..single_points.len()).step_by(2){
        let first = single_points[i];
        let second = single_points[i+1];
        let point = (first, second);
        point_list.push(point);
    }

    if point_list.len() > 0{
        //erster und letzter Punkt des Polygons sind immer gleich
        assert_eq!(point_list.first().unwrap(),point_list.last().unwrap());
    }    

    point_list
}

type Pointlist<T>  = Vec<T>;
type FaceID = usize;

#[derive(Debug)]
enum Facetype{
    Inner(FaceID),
    Outer(FaceID),
    Infinite,
}

#[derive(Debug)]
enum Primitive<T:Num+ Display+FromStr+Debug+Clone+Copy>{
    Face(Facetype, Vec<Primitive<T>>), //Typ mit Border und ID, Kinderliste
    Linelist(Pointlist<T>)             //Punktliste
}

//Face
#[derive(Debug)]
struct Face {
    id: Face_id,
    kontur: Vec<i32>,   
    aufteilende_faces: Vec<Face_id>,
    loecher: Vec<Loch>
}


//Face-ID
#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Face_id(i32);


//Loch
#[derive(Debug)]
struct Loch {
    kontur: Vec<i32>,
    aufteilende_faces: Vec<Face_id>
}



//Familiy
#[derive(Debug)]
struct Family {
    outer: Vec<i32>,
    inner: Vec<Vec<i32>>
}

fn new_family(outer_kontur : Vec<i32>) -> Family{
    let mut inner_konur_list : Vec<Vec<i32>> = Vec::new();
    Family {
        outer: outer_kontur,
        inner: inner_konur_list
    }
} 


fn new_face(f_id : Face_id, kon : Vec<i32>) -> Face{
    let empty_face_array : Vec<Face_id> = Vec::new();
    let empty_loch_array : Vec<Loch> = Vec::new();

    Face {
        id: f_id,
        kontur: kon,   
        aufteilende_faces:empty_face_array,
        loecher: empty_loch_array
    }
}

fn get_without_first_entry( line : &Vec<i32>) -> Vec<i32>{
    let mut result : Vec<i32> = Vec::new();
    for i in 1..line.len(){
        result.push(line[i]);
    }

    result
}

fn get_holes(count_holes: usize, result_vector : &Vec<Vec<i32>>, inclusive_startindex : usize) -> Vec<Loch> {
    let mut loecher : Vec<Loch> = Vec::new();
    for hole_counter in 0..count_holes{
        let current_index = inclusive_startindex + 2 * hole_counter; 
        let current_loch : Loch = get_loch(result_vector, current_index);
        loecher.push(current_loch);
    }
    loecher
}

fn get_loch(result_vector : &Vec<Vec<i32>>, inclusive_startindex : usize) -> Loch{
    let loch_kontur =  result_vector[inclusive_startindex].clone();
    let bestehend_aus = get_as_face_id_vector(&result_vector[inclusive_startindex+1]);

    Loch {
        kontur: loch_kontur,
        aufteilende_faces: bestehend_aus
    }
}

fn get_as_face_id_vector(number_array : &Vec<i32>) ->  Vec<Face_id>{
    let mut face_ids : Vec<Face_id> = Vec::new();

    for f_id in number_array{
            face_ids.push(Face_id(*f_id));
    } 

    face_ids
}


fn get_families_from_tree(result_vector : Vec<Vec<i32>>) -> Vec<Family> {
    //todo: handle empty vector
    //todo: handle empty vector at first position
    assert!(result_vector.len() > 0);
    assert!(result_vector[0].len() == 1);
    
    let loecher_in_inf_face =  get_as_usize ( (result_vector[0])[0]);
    
    let mut top_level_face_holes : Vec<Loch> = Vec::new();
    let mut faces = HashMap::new();

    let mut current_top_level_face_no = 0i32;
    for i in 0..loecher_in_inf_face{
        //calculate positions
        current_top_level_face_no = current_top_level_face_no -1;
        let inf_face_loch_nr = i +1 ;
        let polygonzug_index = 2*inf_face_loch_nr -1;
        let faces_index = 2*inf_face_loch_nr;
             
        //Set holes in inf-face 
        let inf_loch_outer = get_loch(&result_vector, polygonzug_index);
              
    
        top_level_face_holes.push(inf_loch_outer);
    }

    
    let mut i = (loecher_in_inf_face * 2)+1;
    while i + 1  < result_vector.len(){
        //get values
        let current_face_id = Face_id((result_vector[i])[0]);
        let count_holes = get_as_usize( (result_vector[i+1])[0] );
        let current_kontur = get_without_first_entry(&result_vector[i]);
               
        //create Face-Entry
        let mut face = new_face(current_face_id, current_kontur);
        face.loecher = get_holes(count_holes, &result_vector, i+2); 
        faces.insert(current_face_id, face);
        i = (i+2) + 2*count_holes;
    }



    //get families:
    let mut families : Vec<Family> = Vec::new();
    for top_outer_loch in top_level_face_holes{
        handle_outer_loch(&top_outer_loch ,&mut families, &faces);
    }

    families
}

fn handle_outer_loch(top_outer_loch : &Loch, families : &mut Vec<Family>, face_map : &HashMap<Face_id, Face>){
    //get outer-part:
    let mut family = new_family(top_outer_loch.kontur.clone());

    //get inner parts:
    for aufteilendes_face_id in &top_outer_loch.aufteilende_faces{
        let aufteilende_face = face_map.get(aufteilendes_face_id).unwrap();
        for inner_loch in &aufteilende_face.loecher{
            //füge Inner-Kontur zur familie hinzu.
            family.inner.push(inner_loch.kontur.clone());
        
            //Suche weitere Outer-Polygone rekursiv
            for inner_aufteilendes_face_id in &inner_loch.aufteilende_faces{
                let inner_aufteilende_face = face_map.get(inner_aufteilendes_face_id).unwrap();
                for outer_loch in &inner_aufteilende_face.loecher{
                    handle_outer_loch(outer_loch,families, face_map);
                }
            }
            
        }
    }
    families.push(family);
}


fn get_as_usize<T:Num+ Display+FromStr+Debug+Clone+Copy>(t:T) -> usize{
    let string_value = t.to_string();
    let usize_value = string_value.parse::<usize>().unwrap();

    usize_value
}

fn get_as_usize_vec<T:Num+ Display+FromStr+Debug+Clone+Copy>(t_vec :Vec<T>) -> Vec<usize>{
    let mut usize_vec = Vec::new();
    for t in t_vec{
        usize_vec.push(get_as_usize(t));
    }

    usize_vec
}