/****************************************************************************
*  ++ LUKAS BAUR, Bachelorarbeit 2018/2019 ++
*
*  This module serves a testing-suit for all modules created for this thesis.
*
*****************************************************************************/



//-------------------------------Import----------------------------------------------------------------
//from external
use std::fs::File;
use std::collections::HashMap;
use std::io::Read;
use std::io::Write;


//from lib
use crate::{generate_polygons, 
load_segmented_polygons_from_file,
PolygonParsingError,
PointInPolygonRelation,
load_from_fmi_lon_lat_format,
sort_polygons_topologically,
get_coordiante_representation_vector,
is_inside,
Node,
Segment, 
PoiArea, 
SegmentId,
NodeId,
load_from_polygonlist,
Skeletongraph,
generate_polygon_skeleton_graph,
get_most_central_points,
get_point_of_inaccessibility, 
Polygontriangulation,
generate_polygon_triangulation,
write_diagonals_to_file,
write_skeleton_to_file,
write_central_points_to_file,
CentralityVariant,
Weights,
SearchingMethod,
ReductionType,
reduce_family_shape_details,
write_to_fmi_lon_lat_format_file,
create_empty_file,
params_to_string,
create_file,
SkeletonType,
clean_and_get_center_point
};


//---------------------------------public API-function---------------------------------------------------




pub fn test_point_in_polygon(){
        let mut polygon:  Vec<(i32,i32)> = Vec::new();
        polygon.push((0,0));
        polygon.push((1,0));
        polygon.push((2,1));
        polygon.push((3,0));
        polygon.push((4,0));
        polygon.push((2,5));
        polygon.push((0,0));
        
        let point : (f32,f32) = (3 as f32,2 as f32); 

        println!("Punkt ist {}", match is_inside(&polygon, point){
            PointInPolygonRelation::isInside => "innerhalb",
            PointInPolygonRelation::isOutside => "außerhalb",
            PointInPolygonRelation::onBorder => "genau auf einer Kante",
        });
}


pub fn test_polylabel_calculation(){
    //set Filename
    let filename = String::from("resources/testpolygons/lon_lat_FMI_format/good_PoI_example3");
    let mut input_path  = filename.clone();
    input_path.push_str(".txt");
    let mut output_path  = filename.clone();
    output_path.push_str(".result");

    //Open File to read
    let mut file_ptr = create_file(&output_path);
    

    let loaded : Vec<Vec<Vec<(f64,f64)>>> = load_from_fmi_lon_lat_format(&input_path);
       
    for polygon_family in loaded{
        let point : (f64, f64, f64)= get_point_of_inaccessibility(&polygon_family, 0.1 as f64);
        let writestring = format!("{} {} {}\n", point.0, point.1, point.2);
        write_string_to_file(writestring, &mut file_ptr);
        println!("{:?}", point);
    }

}


pub fn test_polygon_sorting(){
    let inner_list = load_from_polygonlist(&"resources/testpolygons/inner_polygons001".to_string());
    let outer_list = load_from_polygonlist(&"resources/testpolygons/outer_polygons001".to_string());
    let result = sort_polygons_topologically(inner_list, outer_list);
    println!("{:?}", result)
} 


pub fn test_a_star(){
    println!("A* Test --------------------");
    let (skeleton_graph, endpoints) = load_lon_lat_skeleton(&"resources/testgraphs/test_skeleton1.txt".to_string());
    let skel : Skeletongraph = Skeletongraph::new(skeleton_graph , endpoints);
    
    println!("{:?}", skel);
    let central_points = get_most_central_points(&skel, &CentralityVariant::Betweenness(SearchingMethod::AStar1To1));
    println!("{:?}", central_points);
}


pub fn test_polygon_skeleton_centrality(){
    println!("Test Polygon Triangulation-----------------------");
    //let filenames  = ["triangulation_skeleton_graph", "bw", "brandenburg", "bremen", "niedersachsen", "nordrhein-westfahlen", "rheinlandpfalz", "saarland"];
    let filenames  = ["triangulation_skeleton_graph", "brandenburg", "bw", "bremen", "niedersachsen", "nordrhein-westfahlen", "rheinlandpfalz", "saarland"];
    //let filenames  = ["bw_shrinked1", "bw_shrinked2", "bw_shrinked3", "bw_shrinked4", "bw_shrinked7"];
    //let filenames  = ["brandenburg_shrinked1", "brandenburg_shrinked2", "brandenburg_shrinked3", "brandenburg_shrinked4", "brandenburg_shrinked7"];
    //let filenames  = ["rheinlandpfalz_shrinked1", "rheinlandpfalz_shrinked2", "rheinlandpfalz_shrinked3", "rheinlandpfalz_shrinked4", "rheinlandpfalz_shrinked7"];
    //let filenames = ["brandenburg"];

    let input_path  = "resources/testpolygons/lon_lat_FMI_format/"; 
    //let input_path  = "resources/testpolygons/lon_lat_FMI_format/shrinked/";
    let output_path = "output/point_centrality/";
    //let output_path = "output/point_centrality/shrinked/";
    
    test_polygon_skeleton_centrality_from_files(&filenames, input_path, output_path) 
}   


fn test_polygon_skeleton_centrality_from_files(filenames: &[&str], input_path: &str, output_path : &str){
    for filename in filenames{
        println!("{}", filename);
        //Set input-filepath
        let mut input_file_path = params_to_string(input_path, filename, ".txt");

        //Set output filepaths
        let mut diagonal_output_file = create_empty_file(output_path, filename, ".diagonals");    
        let mut skeleton_output_file = create_empty_file(output_path, filename, ".skeleton");
        
        //load Polygon_collection    
        let input : Vec<Vec<Vec<(f64,f64)>>>  = load_from_fmi_lon_lat_format(&input_file_path);
            

        //************************* Main Calculation ***************************** 

        //triangulate
        let mut triangulations : Vec<Polygontriangulation> = Vec::new();
        for polygon_family in input{
            //calculate points
            let triangulation : Polygontriangulation = generate_polygon_triangulation(&polygon_family);
            write_diagonals_to_file(&triangulation, &mut diagonal_output_file);
            triangulations.push(triangulation);
        }


        //calc skeleton-graph
        let mut skeletongraphs : Vec<Skeletongraph> = Vec::new(); 
        for triangulation in triangulations{
            let skeletongraph : Skeletongraph = generate_polygon_skeleton_graph(&triangulation, SkeletonType::FmiCenterBased);
            write_skeleton_to_file(&skeletongraph, &mut skeleton_output_file);
            skeletongraphs.push(skeletongraph);
        } 
        
        //let method = &CentralityVariant::Closeness(SearchingMethod::Disjkstra1ToAll, Weights::CoverLength);
        let method = &CentralityVariant::Betweenness(SearchingMethod::Disjkstra1ToAll);
        //let method = &CentralityVariant::Betweenness(SearchingMethod::TreepathsShrink);
        
        let mut right_ending = match method {
                CentralityVariant::Betweenness(_) => ".skeleton_central_points_betweenness",
                CentralityVariant::Closeness(_,_)=>  ".skeleton_central_points_closeness",
            };
        let mut point_output_file  = create_empty_file(output_path, filename, right_ending);

        //calc most central point(s)
        for skeletongraph in skeletongraphs{
            let most_central_points : Vec<(f64, f64)>  = get_most_central_points(&skeletongraph, method);
            
            write_central_points_to_file(&most_central_points, &mut point_output_file);
            println!("{:?}", most_central_points);
        }

    }
}


pub fn test_load_polygon_from_file(){
    let loaded = load_from_polygonlist(&"resources/testpolygons/inner_polygons001".to_string());
    println!("{:?}", loaded);
}


pub fn test_geojson_geration(){

    println!("Anfang{}", "");

        let  (area, segment_mapping, node_mapping) = load_test_area();
         
        let outer_list = get_coordiante_representation_vector(&area.outer, &segment_mapping, &node_mapping);
        let inner_list = get_coordiante_representation_vector(&area.inner, &segment_mapping, &node_mapping);
        //println!("Innerlist: \n{:?}", inner_list);
        println!("Outerlist: \n{:?}", outer_list);

        //let inner_list_result = generate_polygons(&inner_list);
        let outer_list_result = generate_polygons(&outer_list);

        let outer_polygon_list : Vec<Vec<(i32,i32)>> = match outer_list_result {
            Ok(obj) => obj,
            Err(error) => match error{
                PolygonParsingError::DeadEnd(code) => panic!("Polygon construction failed: ".to_string() + &code)
            } 
        };
        
        println!("OuterList: \n{:?}", outer_polygon_list);
        //println!("Innerlist: \n{:?}", inner_polygon_list);
        

}


#[test]
fn  test_treepath_shrink() {
    let (skeleton_vec, end_points_vec) = load_lon_lat_skeleton(&"resources/testgraphs/test_skeleton2.txt".to_string());
    let skel = Skeletongraph::new(skeleton_vec, end_points_vec); 
    let most_central_points : Vec<(f64, f64)>  = get_most_central_points(&skel, &CentralityVariant::Betweenness(SearchingMethod::TreepathsShrink));
}


#[test]
fn  test_naive_polygon_shirnk() {
    let iterations = 6;
    let mapname = "rheinlandpfalz"; 

    let path = format!("resources/testpolygons/lon_lat_FMI_format/{}.txt", mapname);
    let polygon_familiy_collection : Vec<Vec<Vec<(f64,f64)>>> = load_from_fmi_lon_lat_format(&path);
    

    let mut new_polygon_familiy_collection : Vec<Vec<Vec<(f64,f64)>>> = Vec::new();
    for polygon_familiy in polygon_familiy_collection{
        println!("{}", polygon_familiy[0].len());
        let mut temp_polygon_familiy = reduce_family_shape_details(&polygon_familiy, ReductionType::Naive);
            println!("{}", temp_polygon_familiy[0].len());
        for i in 0..iterations-1{
            temp_polygon_familiy = reduce_family_shape_details(&temp_polygon_familiy, ReductionType::Naive);
            println!("{}", temp_polygon_familiy[0].len());
        }
        new_polygon_familiy_collection.push(temp_polygon_familiy);
        println!("----------------");
        if polygon_familiy.len() < 2{
            continue;
        }
    }

    let output_path = "output/shrink_polygon_test/";
    let filename = &format!("{}_shrinked{}", mapname, iterations);
    let ending = ".txt";
    write_to_fmi_lon_lat_format_file(output_path, filename, ending, new_polygon_familiy_collection);

    
}

#[test]
fn  test_polygons_to_file() {
    let polygon_familiy_collection : Vec<Vec<Vec<(f64,f64)>>> = load_from_fmi_lon_lat_format(&"resources/testpolygons/lon_lat_FMI_format/bw.txt".to_string());
    let output_path = "output/load_polygon_test/";
    let filename = "bw";
    let ending = ".exported";
   
    write_to_fmi_lon_lat_format_file(output_path, filename, ending, polygon_familiy_collection);
    
}


//1. load Input
//2. Use Decisiontree to find center point 
pub fn test_clean_and_get_center_point(){
    let filename = "bremen";
    let input_path  = "resources/testpolygons/lon_lat_FMI_format/";
    let output_path = "output/point_centrality/";

    println!("{}", filename);
    //Set input-filepath
    let mut input_file_path = params_to_string(input_path, filename, ".txt");

    //load Polygon_collection    
    let polygon_family_collection : Vec<Vec<Vec<(f64,f64)>>>  = load_from_fmi_lon_lat_format(&input_file_path);


    let result = clean_and_get_center_point(&polygon_family_collection); 

    match result{
        Ok(point) => println!("OK: {:?}", point),
        Err(error) => println!("ERROR: {:?}", error),
   }
    
}











//---------------------------------private functions----------------------------------------------------






fn get_coordiante_representation(
    segment_collection: &Vec<SegmentId>,
    segment_mapping: &HashMap<SegmentId, Segment>,
    node_mapping: &HashMap<NodeId, Node>,
) -> String {
    let mut result = "".to_string();
    let mut is_first_outer_round = true;

    for segment_id in segment_collection {
        if is_first_outer_round{
            is_first_outer_round = false;
        }
        else {
            result.push_str( ", ");
        }
        
        
        result.push_str("[");
        let mut is_first_round = true;
        
        let segment = segment_mapping.get(&segment_id).unwrap();
        for node_id in &segment.nodes {
            let node = node_mapping.get(&node_id);
            let point_coordinate = node.unwrap().get_coordinates();

            if is_first_round{
                is_first_round = false;
            } else {
                result.push_str(", ");
            }
            result.push_str(&point_coordinate);
        }

    result.push_str("] ");
    }
    result
}


fn load_lon_lat_skeleton(path: &String) -> ( Vec<(f64,f64,f64,f64)>,  Vec<(f64,f64)>){
    let mut skeleton_vec : Vec<(f64,f64,f64,f64)> = Vec::new();
    let mut end_points_vec : Vec<(f64,f64)> = Vec::new();
    
    let mut file = File::open(path).expect("Could not open input file! Exiting!");
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    contents = contents.trim().to_string();
    
    let mut read_end_edges = true;

    for line in contents.split("\n"){
        if line.len() == 0{
            read_end_edges = false;
            continue;
        }

        let mut point_temp : Vec<f64> = Vec::new();
        for point in line.split(" "){
            point_temp.push(point.parse().unwrap());

            if read_end_edges {
                if point_temp.len() == 2{
                    let entry = tuple_from_vec(&point_temp);
                    end_points_vec.push(entry);
                    point_temp.clear();
                }
            }
            else{
                if point_temp.len() == 4{
                    let entry = quadruple_from_vec(&point_temp);
                    skeleton_vec.push(entry);
                    point_temp.clear();
                }
            }


        }
    }

    (skeleton_vec, end_points_vec)
}


fn tuple_from_vec(vec : &Vec<f64>) -> (f64,f64){
    assert!(vec.len() == 2);
    let p0 = *vec.get(0).unwrap();
    let p1 = *vec.get(1).unwrap();

    (p0,p1)
}


fn quadruple_from_vec(vec : &Vec<f64>) -> (f64,f64,f64,f64){
    assert!(vec.len() == 4);
    let p0 = *vec.get(0).unwrap();
    let p1 = *vec.get(1).unwrap();
    let p2 = *vec.get(2).unwrap();
    let p3 = *vec.get(3).unwrap();

    (p0,p1,p2,p3)
}


fn write_string_to_file(file_content : String, file : &mut File ){
    file.write_all(file_content.as_bytes());
}

fn test_polygon_wiederzusammensetzen(){
//Test for Polygons
    let polygon_segments_list = load_segmented_polygons_from_file(&"resources/testpolygons/test001".to_string());
    println!("-------------- Ende --------------");  
    for polgon_segemnts in polygon_segments_list{
        match generate_polygons(&polgon_segemnts){
            Ok(poly) => println!("Erfolgreiche Umwandlung:\n{:?}", poly),
            Err(error_code) => 
                    match error_code{
                        PolygonParsingError::DeadEnd(code) => println!("Dead End found: {}", code),
                        _ => panic!("Panic!!!!!!!!")
                    } 
        }
    }
    
}


fn load_test_area() -> (PoiArea, HashMap<SegmentId, Segment>, HashMap<NodeId, Node>){

        let test_outers : Vec<SegmentId> = vec![ 
                osmpbfreader::objects::WayId(3 as i64),
                osmpbfreader::objects::WayId(5 as i64),  
                osmpbfreader::objects::WayId(6 as i64),  
                osmpbfreader::objects::WayId(4 as i64),  
                osmpbfreader::objects::WayId(8 as i64)];
        let test_inners : Vec<SegmentId>= vec![
                osmpbfreader::objects::WayId(1 as i64),
                osmpbfreader::objects::WayId(2 as i64),  
                osmpbfreader::objects::WayId(7 as i64),
                ];
   
        
        let p1 = Node {
            osmid:  osmpbfreader::objects::NodeId(1 as i64),
            decimicro_lon: 35 as i32,
            decimicro_lat: 40 as i32,
        };
        let p2 = Node {
            osmid:  osmpbfreader::objects::NodeId(2 as i64),
            decimicro_lon: 40 as i32,
            decimicro_lat: 50 as i32,
        };
        let p3 = Node {
            osmid:  osmpbfreader::objects::NodeId(3 as i64),
            decimicro_lon: 35 as i32,
            decimicro_lat: 40 as i32,
        };
        let p4 = Node {
            osmid:  osmpbfreader::objects::NodeId(4 as i64),
            decimicro_lon: 30 as i32,
            decimicro_lat: 55 as i32,
        };
        let p5 = Node {
            osmid:  osmpbfreader::objects::NodeId(5 as i64),
            decimicro_lon: 75 as i32,
            decimicro_lat: 50 as i32,
        };
        let p6 = Node {
            osmid:  osmpbfreader::objects::NodeId(6 as i64),
            decimicro_lon:60 as i32,
            decimicro_lat: 60 as i32,
        };
        let p7 = Node {
            osmid:  osmpbfreader::objects::NodeId(7 as i64),
            decimicro_lon: 55 as i32,
            decimicro_lat: 40 as i32,
        };
        let p8 = Node {
            osmid:  osmpbfreader::objects::NodeId(8 as i64),
            decimicro_lon: 65 as i32,
            decimicro_lat: 40 as i32,
        };
        let p9 = Node {
            osmid:  osmpbfreader::objects::NodeId(9 as i64),
            decimicro_lon: 30 as i32,
            decimicro_lat: 20 as i32,
        };
        let p10 = Node {
            osmid:  osmpbfreader::objects::NodeId(10 as i64),
            decimicro_lon: 15 as i32,
            decimicro_lat: 35 as i32,
        };
        let p11 = Node {
            osmid:  osmpbfreader::objects::NodeId(11 as i64),
            decimicro_lon: 20 as i32,
            decimicro_lat: 65 as i32,
        };
        let p12 = Node {
            osmid:  osmpbfreader::objects::NodeId(12 as i64),
            decimicro_lon: 30 as i32,
            decimicro_lat: 70 as i32,
        };
        let p13 = Node {
            osmid:  osmpbfreader::objects::NodeId(13 as i64),
            decimicro_lon: 30 as i32,
            decimicro_lat: 70 as i32,
        };
        let p14 = Node {
            osmid:  osmpbfreader::objects::NodeId(14 as i64),
            decimicro_lon: 45 as i32,
            decimicro_lat: 65 as i32,
        };
        let p15 = Node {
            osmid:  osmpbfreader::objects::NodeId(15 as i64),
            decimicro_lon: 60 as i32,
            decimicro_lat: 50 as i32,
        };
        let p16 = Node {
            osmid:  osmpbfreader::objects::NodeId(16 as i64),
            decimicro_lon: 55 as i32,
            decimicro_lat: 20 as i32,
        };
        let p17 = Node {
            osmid:  osmpbfreader::objects::NodeId(17 as i64),
            decimicro_lon: 30 as i32,
            decimicro_lat: 20 as i32,
        };
        let p18 = Node {
            osmid:  osmpbfreader::objects::NodeId(18 as i64),
            decimicro_lon: 45 as i32,
            decimicro_lat: 65 as i32,
        };
        let p19 = Node {
            osmid:  osmpbfreader::objects::NodeId(19 as i64),
            decimicro_lon: 60  as i32,
            decimicro_lat: 75 as i32,
        };
        let p20 = Node {
            osmid:  osmpbfreader::objects::NodeId(20 as i64),
            decimicro_lon: 95 as i32,
            decimicro_lat: 70 as i32,
        };
        let p21 = Node {
            osmid:  osmpbfreader::objects::NodeId(21 as i64),
            decimicro_lon: 90 as i32,
            decimicro_lat: 45 as i32,
        };
        let p22 = Node {
            osmid:  osmpbfreader::objects::NodeId(22 as i64),
            decimicro_lon: 30 as i32,
            decimicro_lat: 20 as i32,
        };
        let p23 = Node {
            osmid:  osmpbfreader::objects::NodeId(23 as i64),
            decimicro_lon: 85 as i32,
            decimicro_lat: 15 as i32,
        };
        let p24 = Node {
            osmid:  osmpbfreader::objects::NodeId(24 as i64),
            decimicro_lon: 70 as i32,
            decimicro_lat: 25 as i32,
        };
        let p25 = Node {
            osmid:  osmpbfreader::objects::NodeId(25 as i64),
            decimicro_lon: 90 as i32,
            decimicro_lat: 45 as i32,
        };

        let p26 = Node {
            osmid:  osmpbfreader::objects::NodeId(26 as i64),
            decimicro_lon: 30 as i32,
            decimicro_lat: 20 as i32,
        };
        let p27 = Node {
            osmid:  osmpbfreader::objects::NodeId(27 as i64),
            decimicro_lon: 55 as i32,
            decimicro_lat: 20 as i32,
        };
        let p28 = Node {
            osmid:  osmpbfreader::objects::NodeId(28 as i64),
            decimicro_lon: 60 as i32,
            decimicro_lat: 50 as i32,
        };
        let p29 = Node {
            osmid:  osmpbfreader::objects::NodeId(29 as i64),
            decimicro_lon: 45 as i32,
            decimicro_lat: 65 as i32,
        };
        let p30 = Node {
            osmid:  osmpbfreader::objects::NodeId(30 as i64),
            decimicro_lon: 30 as i32,
            decimicro_lat: 55 as i32,
        };
        let p31 = Node {
            osmid:  osmpbfreader::objects::NodeId(31 as i64),
            decimicro_lon: 30 as i32,
            decimicro_lat: 40 as i32,
        };

        let mut node_mapping: HashMap<NodeId, Node> = HashMap::new();
        node_mapping.insert(p1.osmid, point_clone(&p1));
        node_mapping.insert(p2.osmid, point_clone(&p2));
        node_mapping.insert(p3.osmid, point_clone(&p3));
        node_mapping.insert(p4.osmid, point_clone(&p4));
        node_mapping.insert(p5.osmid, point_clone(&p5));
        node_mapping.insert(p6.osmid, point_clone(&p6));
        node_mapping.insert(p7.osmid, point_clone(&p7));
        node_mapping.insert(p8.osmid, point_clone(&p8));
        node_mapping.insert(p9.osmid, point_clone(&p9));
        node_mapping.insert(p10.osmid, point_clone(&p10));
        node_mapping.insert(p11.osmid, point_clone(&p11));
        node_mapping.insert(p12.osmid, point_clone(&p12));
        node_mapping.insert(p13.osmid, point_clone(&p13));
        node_mapping.insert(p14.osmid, point_clone(&p14));
        node_mapping.insert(p15.osmid, point_clone(&p15));
        node_mapping.insert(p16.osmid, point_clone(&p16));
        node_mapping.insert(p17.osmid, point_clone(&p17));
        node_mapping.insert(p18.osmid, point_clone(&p18));
        node_mapping.insert(p19.osmid, point_clone(&p19));
        node_mapping.insert(p20.osmid, point_clone(&p20));
        node_mapping.insert(p21.osmid, point_clone(&p21));
        node_mapping.insert(p22.osmid, point_clone(&p22));
        node_mapping.insert(p23.osmid, point_clone(&p23));
        node_mapping.insert(p24.osmid, point_clone(&p24));
        node_mapping.insert(p25.osmid, point_clone(&p25));
        node_mapping.insert(p26.osmid, point_clone(&p26));
        node_mapping.insert(p27.osmid, point_clone(&p27));
        node_mapping.insert(p28.osmid, point_clone(&p28));
        node_mapping.insert(p29.osmid, point_clone(&p29));
        node_mapping.insert(p30.osmid, point_clone(&p30));
        node_mapping.insert(p31.osmid, point_clone(&p31));


        let s1 = Segment{
            osmid: osmpbfreader::WayId(1 as i64),  
            nodes: vec![p1.osmid, p2.osmid, p30.osmid ]
        };
        let s2 = Segment{
            osmid: osmpbfreader::WayId(2 as i64),  
            nodes: vec![p3.osmid, p31.osmid, p4.osmid ]
        };
        let s3 = Segment{
            osmid: osmpbfreader::WayId(3 as i64),  
            nodes: vec![p9.osmid, p10.osmid,p11.osmid, p12.osmid]
        };
        let s4 = Segment{
            osmid: osmpbfreader::WayId(4 as i64),  
            nodes: vec![p13.osmid, p14.osmid, p15.osmid, p16.osmid, p17.osmid ]
        };
        let s5 = Segment{
            osmid: osmpbfreader::WayId(5 as i64),  
            nodes: vec![p18.osmid, p19.osmid, p20.osmid, p21.osmid]
        };
        let s6 = Segment{
            osmid: osmpbfreader::WayId(6 as i64),  
            nodes: vec![p22.osmid, p23.osmid, p24.osmid,p25.osmid]
        };
        let s7 = Segment{
            osmid: osmpbfreader::WayId(7 as i64),  
            nodes: vec![p5.osmid, p6.osmid, p7.osmid, p8.osmid, p5.osmid]
        };
        let s8 = Segment{
            osmid: osmpbfreader::WayId(8 as i64),  
            nodes: vec![p26.osmid, p27.osmid, p28.osmid, p29.osmid]
        };

        let mut segment_mapping: HashMap<SegmentId, Segment> = HashMap::new();
        segment_mapping.insert(s1.osmid,s1);
        segment_mapping.insert(s2.osmid,s2);
        segment_mapping.insert(s3.osmid,s3);
        segment_mapping.insert(s4.osmid,s4);
        segment_mapping.insert(s5.osmid,s5);
        segment_mapping.insert(s6.osmid,s6);
        segment_mapping.insert(s7.osmid,s7);
        segment_mapping.insert(s8.osmid,s8);
        
        (PoiArea{
            inner: test_inners,
            outer: test_outers,
            osmid: osmpbfreader::RelationId(9292929 as i64),
            name: "".to_string(),
            tags: osmpbfreader::Tags::new()
        }
        , segment_mapping,
        node_mapping)

}


fn point_clone(current : &Node) -> Node{
    Node {
        osmid:  current.osmid,
        decimicro_lat: current.decimicro_lat,
        decimicro_lon: current.decimicro_lon,
    }
}
