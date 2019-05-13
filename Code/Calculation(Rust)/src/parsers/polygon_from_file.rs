/*


++ LUKAS BAUR, Bachelorarbeit 2018/2019 ++

This module loads polygons from files.  

*/

use std::fs::File;
use std::io::Read;
use std::str::FromStr;

/*
INPUT:  File in form of <polygon_list> with 
        <polygon_list> =  [<polygon>, <polygon>, ..., <polygon>]
        <polygon> = [<point>, <point>, ..., <point>]
        <point>     = (x_i, y_i) 

OUTPUT: Returns the same structure with Vec-of-Vecs. Points are stored as (i32, i32).
*/
pub fn load_from_polygonlist(filepath: &String)-> Vec<Vec<(i32, i32)>>{
    let mut file = File::open(filepath).expect("Could not open input file! Exiting!");
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    contents = contents.trim().to_string();
 
    let mut polynon_collection : Vec<Vec<(i32, i32)>> = Vec::new();
    for polygon in contents.split("\n"){
        let mut polynon_vec : Vec<(i32, i32)> = Vec::new();
        
                for tuple_str in  polygon.trim().to_string().split(")"){
                let mut start_index :usize = 0;
                let start_str = tuple_str.find("(");
                match start_str {
                    Some(_) => start_index = start_str.unwrap()+1,
                    _  => continue,
                };  
                
                let mut point_vec: (i32, i32) = (0,0);
                let core_str = &tuple_str[start_index..].trim();
                let mut counter :usize= 0;
                for tuple_entry in core_str.split(","){
                    let plain = tuple_entry.trim();
                    let zahl : i32 = match plain.parse(){
                        Ok(zahl) => zahl,
                        _ => break
                    };
                    match counter {
                        0 => point_vec.0 = zahl,
                        1 => point_vec.1 = zahl,
                        _ => break
                    };
                    counter = counter +1;
                }
                if counter == 2{
                    polynon_vec.push(point_vec.clone());
                }
            }            
        if polynon_vec.len() > 0{
            polynon_collection.push(polynon_vec.clone());
        }
    }
    polynon_collection
}


#[derive(Debug)]
enum ReadMode {
    InterpretOuter,
    InterpretInners,
    InterpretInnerNumber,
}

/*
INPUT:  <polgonfamily>\n
        <polgonfamily>\n
        ...
        <polgonfamily>
        
        
        WHERE
        <polgonfamily> =    <outer_polygom_lonlat_list>\n
                            k\n
                            <inner_polygom_lonlat_list_1>\n
                            <inner_polygom_lonlat_istt_2>\n
                            ...
                            <inner_polygom_lonlat_list_k>\n
                            
        <outer_polygom_lonlat_list>     = <polygom_lonlat_list>
        <inner_polygom_lonlat_list_i>   = <polygom_lonlat_list>
        <polygom_lonlat_list> = <point_1> <point_2> ... <point_n> <point_1>
        <point_i> = lon lat
OUTPUT: [<polygonfamily>, <polygonfamily>, ..., <polygonfamily>]
        
        WHERE
        <polygonfamily> = [<outer_polygon>, <inner_polygon>, <inner_polygon>, ..., <inner_polygon>]
        <outer_polygon> = <polygon>       
        <inner_polygon> = <polygon> 
        <polygon> = [(x,y), (x,y), ..., (x,y)]
        x,y = T
*/
pub fn load_from_fmi_lon_lat_format<T>(filepath: &String) -> Vec<Vec<Vec<(T,T)>>> where T: FromStr + Copy
{
    let mut file = File::open(filepath).expect("Could not open input file! Exiting!");
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    contents = contents.trim().to_string();
    
    let mut read_mode : ReadMode = ReadMode::InterpretOuter;
    let mut polygonfamily_collection : Vec<Vec<Vec<(T,T)>>> = Vec::new();

    let mut inner_polygons : Vec<Vec<(T,T)>> = Vec::new();
    let mut polygon_family : Vec<Vec<(T,T)>> = Vec::new();
    

    let mut parse_amount :u32 = 0 as u32;
    let mut already_parsed :u32 = 0 as u32;

    for line in contents.split("\n"){
        match read_mode {
            ReadMode::InterpretOuter => {
                //interpret Outer-Polygon
                let outer_polygon : Vec<(T,T)> = polygon_from_lonmlat_string(line.to_string());
                polygon_family.push(outer_polygon);
                read_mode = ReadMode::InterpretInnerNumber;
            },
            ReadMode::InterpretInnerNumber => {
                //interpret Inner-Polygons    
                parse_amount = u32::from_str(line ).unwrap();
                if parse_amount > 0{
                    read_mode = ReadMode::InterpretInners;
                }
                else{
                    read_mode = ReadMode::InterpretOuter;
                    polygonfamily_collection.push(polygon_family.clone());
                    polygon_family.clear();
                }
            },
            ReadMode::InterpretInners =>{
                //interpret number of inners
                let current_inner_polygon : Vec<(T,T)> = polygon_from_lonmlat_string(line.to_string());
                inner_polygons.push(current_inner_polygon);
                already_parsed = already_parsed + 1;

                if already_parsed >= parse_amount{
                    already_parsed = 0;
                    for inner_polygon in inner_polygons.clone() {
                        polygon_family.push(inner_polygon);
                    }
                    inner_polygons.clear();
                    polygonfamily_collection.push(polygon_family.clone());
                    polygon_family.clear();
                    read_mode = ReadMode::InterpretOuter
                }
            }
        };
        
    }

    polygonfamily_collection
}

fn polygon_from_lonmlat_string<T>(raw_content:String) -> Vec<(T,T)> where T: FromStr + Copy{
    let mut count = 0;
    let mut is_end_of_point = false;
    let mut result_polygon : Vec<(T,T)> = Vec::new();
    let mut first_value : T = match T::from_str("0"){
        Ok(val) => val,
        Err(_) => panic!("Cannot parse form String")
    };
    let mut second_value : T = match T::from_str("0"){
        Ok(val) => val,
        Err(_) => panic!("Cannot parse form String")
    };

    for entry in raw_content.split(" "){
        if is_end_of_point{
            second_value = match T::from_str(entry){
                    Ok(val) => val,
                    Err(_) => panic!("Cannot parse form String")
                };
            result_polygon.push((first_value, second_value));
            is_end_of_point = false;
            count = count + 1;
        }
        else {
            first_value = match T::from_str(entry){
                    Ok(val) => val,
                    Err(_) => panic!("Cannot parse form String")
                };
            is_end_of_point = true;
            count = count + 1;
        }
    }
    if count %2 == 1{
        panic!("Cannot parse from String. There must be as many y-values as x-values.");
    }
    result_polygon
}

/*
INPUT:  File in form of <polygon_segments_collection> with 
        <polygon_segments_collection> =  [<polygon_segment_list>, <polygon_segment_list>, ..., <polygon_segment_list>]
        <polygon_segment_list> = [<segment>, <segment>, ..., <segment>]
        <segment> = [<point>, <point>, ..., <point>]
        <point>     = (x_i, y_i)]

OUTPUT: Returns the same structure with Vec-of-Vecs. Points are stored as (i32, i32).
*/
pub fn load_segmented_polygons_from_file(filepath: &String)->Vec<Vec<Vec<(i32, i32)>>>{
    let mut file = File::open(filepath).expect("Could not open input file! Exiting!");
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    contents = contents.trim().to_string();
 
    let mut polynon_collection : Vec<Vec<Vec<(i32, i32)>>> = Vec::new();
    for polygon in contents.split("\n"){
        let mut polynon_vec : Vec<Vec<(i32, i32)>> = Vec::new();
        for slice in polygon.split("]"){
            let mut slice_vec : Vec<(i32, i32) > = Vec::new();
            for tuple_str in  slice.trim().to_string().split(")"){
                let mut start_index :usize = 0;
                let start_str = tuple_str.find("(");
                match start_str {
                    Some(index) => start_index = start_str.unwrap()+1,
                    _  => continue,
                };  
                
                let mut point_vec: (i32, i32) = (0,0);
                let core_str = &tuple_str[start_index..].trim();
                let mut counter :usize= 0;
                for tuple_entry in core_str.split(","){
                    let plain = tuple_entry.trim();
                    let zahl : i32 = match plain.parse(){
                        Ok(zahl) => zahl,
                        _ => break
                    };
                    match counter {
                        0 => point_vec.0 = zahl,
                        1 => point_vec.1 = zahl,
                        _ => break
                    };
                    counter = counter +1;
                }
                if counter == 2{
                    slice_vec.push(point_vec.clone());
                }
            }
            if slice_vec.len() > 0{
                polynon_vec.push(slice_vec.clone());
            }
        }
        if polynon_vec.len() > 0{
            polynon_collection.push(polynon_vec.clone());
        }
    }
    polynon_collection
}
