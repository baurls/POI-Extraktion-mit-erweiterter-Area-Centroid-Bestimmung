use std::collections::HashMap;
use std::fs::File;
use std::sync::mpsc::{ Sender};
use std::time::{Instant, Duration};

use osmpbfreader::{Relation, Tags};


mod centroid_calculation;
pub use centroid_calculation::skeleton_centerpoint::skeleton_graph_generation::{Skeletongraph, SkeletonType, write_skeleton_to_file,generate_polygon_skeleton_graph};
pub use centroid_calculation::skeleton_centerpoint::triangulation::{Polygontriangulation, generate_polygon_triangulation, write_diagonals_to_file};
pub use centroid_calculation::skeleton_centerpoint::centrality_scores_calculation::{get_most_central_points, write_central_points_to_file, CentralityVariant, SearchingMethod, Weights};
pub use centroid_calculation::point_of_inaccessibility::get_point_of_inaccessibility;
pub use centroid_calculation::simple_algorithms::{get_centroid, get_geometrical_center};
pub use centroid_calculation::clean_and_get_center_point;

//.............................


mod polygonshape_modifications;
pub use polygonshape_modifications::reduce_polygon_details::ReductionType;
pub use polygonshape_modifications::reduce_polygon_details::{reduce_shape_details, reduce_family_shape_details};
//.............................


mod time_measurement;
pub use time_measurement::centroid_calculation_measurement::measure_algorithm_times;
//.............................


mod topological_polygon_sorting;
pub use topological_polygon_sorting::sort_polygons_topologically;
//.............................


mod polygon_check;
//............................


mod polygonizer;
pub use polygonizer::get_polygons_from_segments;
//............................


mod clean_and_repair_family;
//............................


mod test_library;
pub use test_library::{test_a_star,  test_geojson_geration, test_point_in_polygon, test_polygon_skeleton_centrality, test_polygon_sorting, test_polylabel_calculation, test_clean_and_get_center_point};
//.............................


mod parsers;
pub use parsers::polygon_from_file::{load_segmented_polygons_from_file, load_from_polygonlist, load_from_fmi_lon_lat_format};
pub use parsers::polygon_to_file::{write_to_fmi_lon_lat_format_file};
pub use parsers::to_file::{params_to_string, create_empty_file, create_file};
pub use parsers::Tag;
pub use parsers::polygonparser::{generate_polygons, PolygonParsingError};
pub use parsers::{AdminLevel, Latitude, Longitude};
pub use parsers::{AreaId, NodeId, SegmentId};
pub use parsers::{Node, Segment};
//.............................


mod point_in_polyon_test;
pub use point_in_polyon_test::{PointInPolygonRelation, is_inside};



//------------------------------------------- AdminArea Komponenten ----------------------------------
//Admin Area Struce
pub struct AdminArea {
    pub osmid: AreaId,
    pub level: AdminLevel,
    pub name: String,

    pub inner: Vec<SegmentId>,
    pub outer: Vec<SegmentId>
}

//Admin-Area-Factory Struct
struct AdminAreaFactory {
    areas: Vec<AdminArea>,
    segments: HashMap<SegmentId, Segment>,
    nodes: HashMap<NodeId, Node>,
}

//Konstruktor
impl AdminAreaFactory {
    fn new() -> Self {
        AdminAreaFactory {
            areas: Vec::new(),
            segments: HashMap::new(),
            nodes: HashMap::new(),
        }
    }
}

//Interface for AreaFactory
impl parsers::AreaFactory for AdminAreaFactory {
    type Area = AdminArea;

    fn is_valid(&self, tags: &Tags) -> bool {
        return tags.contains("boundary", "administrative")
            && tags.get("name").is_some()
            && tags.get("admin_level").is_some();
    }

    fn to_area(
        &self,
        rel: &Relation,
        inner_id_sender: &Sender<SegmentId>,
        outer_id_sender: &Sender<SegmentId>,
    ) -> Option<AdminArea> {
        assert!(self.is_valid(&rel.tags));

        let osmid = rel.id;
        // TODO: Improve error handling
        let level = match rel.tags.get("admin_level") {
            Some(val) => match val.parse::<u8>() {
                Ok(lvl) => lvl,
                Err(err) => {
                    eprintln!(
                        "Could not parse admin level value of area {}:\n{}",
                        osmid.0, err
                    );
                    return None;
                }
            },
            None => {
                eprintln!("Could not find admin_level tag for area {}", osmid.0);
                return None;
            }
        };
        let name = match rel.tags.get("name") {
            Some(name) => name,
            None => {
                eprintln!("Could not get name of admin area {}", osmid.0);
                return None;
            }
        };

        let mut inner = Vec::new();
        let mut outer = Vec::new();

        for r in &rel.refs {
            match r.role.as_str() {
                "inner" => match r.member {
                    osmpbfreader::OsmId::Way(oid) => {
                        inner_id_sender.send(oid).unwrap();
                        inner.push(r.member);
                    }
                    _ => {
                        eprintln!("Inner relation id is not a WayId in area {}", osmid.0);
                    }
                },
                "outer" => match r.member {
                    osmpbfreader::OsmId::Way(oid) => {
                        outer_id_sender.send(oid).unwrap();
                        outer.push(r.member);
                    }
                    _ => eprintln!("Inner relation id is not a WayId in area {}", osmid.0),
                },
                _ => ()//eprintln!("Ignoring relation role {}", r.role),
            }
        }

        Some(AdminArea {
            osmid: osmid,
            level: level,
            name: name.clone(),

            inner: Vec::new(),
            outer: Vec::new(),
        })
    }

    fn set_segments(&mut self, segments: Vec<Segment>, nodes: Vec<Node>) {
        // TODO: remove clone
        self.segments
            .extend(segments.into_iter().map(|seg| (seg.osmid, seg)));
        self.nodes
            .extend(nodes.into_iter().map(|node| (node.osmid, node)));
    }
}

//------------------------------------------- PoiArea Komponenten ----------------------------------

//Poi Area Struce
struct PoiArea {
    pub osmid: AreaId,
    pub tags: Tags,
    pub name: String,

    pub inner: Vec<SegmentId>,
    pub outer: Vec<SegmentId>,
}

/*
Return-format:
    "osmid":    "1190091"
    "name":     "Isle of Man Government"
    "tags":     {"building"="yes", ... ,"name"="Isle of Man Government"}
    "inner":    {79141577, ... , 17272730}
    "outer":    {59384797, ... , 43567809}
        }
*/
impl std::string::ToString for PoiArea {
    fn to_string(&self) -> String {
        let name = &self.name;
        let osmid = &self.osmid.0;
        let tags = tags_to_string(&self.tags);
        let inner = segment_vec_to_string(&self.inner);
        let outer = segment_vec_to_string(&self.outer);

        let result = format!(
            "{{
            \"osmid\":\"{}\"
            \"name\":\"{}\"
            \"tags\":{}
            \"inner\":{}
            \"outer\":{}
        }}",
            osmid, name, tags, inner, outer
        );

        result.to_string()
    }
}

/*

{inner
      "type": "Feature",
      "geometry": {
        "type": "multipolygon",
        "coordinates": 
        [
            [30, 10], [10, 30], [30, 10], 
            [[20, 30], [35, 35], [30, 20], [20, 30]]
        ]
      },
      "properties": {
        "name": "toller POI",
        "osmid": "23456789",
        "tags": { 
           "building": "yes",
           "landuse": "garden"
        }
      }
}

*/

pub fn get_coordiante_representation_vector(segments : &Vec<SegmentId>, segment_mapping: &HashMap<SegmentId, Segment>, node_mapping: &HashMap<NodeId, Node>) -> Vec<Vec<(i32,i32)>>{
    let mut coordinate_list: Vec<Vec<(i32, i32)>> = Vec::new();   
        println!("segemnts: {:?}", segments);
        for segment_id in segments{
            let mut segment_coordinates : Vec<(i32,i32)>  = Vec::new(); 
            let segment = segment_mapping.get(&segment_id).expect("Segment-ID not found in mapping");
            for node_id in &segment.nodes{
                let x = node_mapping.get(&node_id).expect("Node-ID in Mapping noch found").decimicro_lon;
                let y = node_mapping.get(&node_id).expect("Node-ID in Mapping noch found").decimicro_lat;
                                
                segment_coordinates.push( (x,y) );
            }
            coordinate_list.push(segment_coordinates);
        }
    coordinate_list
}


// Converts 
//      [
//          [ (a,b), (c,d), (e,f) ],
//          [ (e,f), (g,h) ], 
//          [ (i,j), (k,l), (m,n), (a,b) ] 
//      ]
// to
//      [
//          [ (a,b,c,d), (c,d,e,f) ],
//          [ (e,f,g,h) ], 
//          [ (i,j,k,l), (k,l,m,n), (m,n,a,b) ] 
//      ]  
fn get_line_representation_vector(point_vector_list : &Vec<Vec<(i32,i32)>>) -> Vec<Vec<(i32,i32,i32,i32)>>{
    let mut segment_list: Vec<Vec<(i32,i32,i32,i32)>> = Vec::new();  

        for segment in point_vector_list{
            let mut line_list : Vec<(i32,i32,i32,i32)>  = Vec::new(); 
            for i in 1..segment.len(){
                //i   = aktueller  Punkt
                //i-1 = vorheriger Punkt
                let current = segment.get(i).unwrap();
                let before  = segment.get(i-1).unwrap();
                let line = (before.0, before.1, current.0, current.1);
                line_list.push(line);
            }
            segment_list.push(line_list);
        }
    segment_list
}


// Converts 
//      [
//          [ (a,b,c,d), (c,d,e,f) ],
//          [ (e,f,g,h) ], 
//          [ (i,j,k,l), (k,l,m,n), (m,n,a,b) ] 
//      ]  
// to
//      [
//          (a,b,c,d), (c,d,e,f), (e,f,g,h), (i,j,k,l), (k,l,m,n), (m,n,a,b)
//      ]  
fn group_lines_together(line_list_collection : &Vec<Vec<(i32,i32,i32,i32)>>) -> Vec<(i32,i32,i32,i32)>{
    let mut all_lines_ist: Vec<(i32,i32,i32,i32)> = Vec::new();  
    for line_list in line_list_collection{
            all_lines_ist.extend(line_list);
    }
    all_lines_ist
}

fn join_lines(inner: &Vec<(i32,i32,i32,i32)>, outer: &Vec<(i32,i32,i32,i32)>) ->  Vec<(i32,i32,i32,i32)>{
    let mut all_lines : Vec<(i32,i32,i32,i32)> = inner.clone();
    all_lines.extend(outer);

    all_lines
}



impl PoiArea{
    fn generate_geojson(&self, segment_mapping: &HashMap<SegmentId, Segment>,
    node_mapping: &HashMap<NodeId, Node>) -> String{


        //________________________________________________________________________
        //  STEP 1: Load the Inner- and Outer-Points
        
        let inner_point_list = get_coordiante_representation_vector(&self.inner, &segment_mapping, node_mapping);   //Shape: Vec<Vec<(i32,i32)>>
        //println!("Inner-List (als Punkte)\n{:?}\n--------------", inner_point_list);

        let outer_point_list = get_coordiante_representation_vector(&self.outer, &segment_mapping, node_mapping);   //Shape: Vec<Vec<(i32,i32)>>
        //println!("Outer-List (als Punkte)\n{:?}\n--------------", outer_point_list);


        
        //________________________________________________________________________        
        //  STEP 2: Convert to Line-Segemnts

        let inner_line_list = get_line_representation_vector(&inner_point_list);     //Shape: Vec<Vec<(i32,i32,i32,i32)>>
        //println!("Inner-List (als Liniensegmente)\n{:?}\n--------------", inner_line_list);
        let inner_lines_set = group_lines_together(&inner_line_list);                //Shape: <Vec<(i32,i32,i32,i32)>
        //println!("Inner-List (als Liniensegmentmenge)\n{:?}\n--------------", inner_lines_set);

        let outer_line_list = get_line_representation_vector(&outer_point_list);     //Shape: Vec<Vec<(i32,i32,i32,i32)>>
        //println!("Outer-List (als Liniensegmente)\n{:?}\n--------------", outer_line_list);
        let outer_lines_set = group_lines_together(&outer_line_list);                //Shape: <Vec<(i32,i32,i32,i32)>
        //println!("Outer-List (als Liniensegmentmenge)\n{:?}\n--------------", outer_lines_set);
        let mixed_lines = join_lines(&outer_lines_set, &inner_lines_set);    


        //________________________________________________________________________
        //  STEP 3: Convert to Inner- and Outer-Polygons
        let polygon_family_collection = get_polygons_from_segments(mixed_lines);
        println!("Poygon-Family: {:?}\n\n", polygon_family_collection);

        let floating_polygon_family_collection = convert_to_floating_structure(&polygon_family_collection);
        println!("Poygon-Family(als float): {:?}\n\n", floating_polygon_family_collection);

        let center_point = clean_and_get_center_point(&floating_polygon_family_collection); 
        println!("{:?}", center_point) ;

        let geojeson_multi_polygon_string = get_geo_json_multi_polygon_string(&polygon_family_collection);

        let tag_list = tags_to_string(&self.tags);
         
        let outer_list = "".to_string();
        let inner_list = "".to_string();
        
        let geojosn = format!(
        "{{
            \"type\": \"Feature\",
            \"geometry\": {},
            \"properties\": {{
                \"name\": \"{}\",
                \"osmid\": \"{}\",
                \"tags\": {{ 
                {}
                }}
            }}
        }}",
        geojeson_multi_polygon_string,
        self.name,
        self.osmid.0,
        tag_list    
        );
    geojosn
    }
}

fn convert_to_floating_structure(input_collection : &Vec<Vec<Vec<(i32, i32)>>>) -> Vec<Vec<Vec<(f64, f64)>>>{
    let mut output_family_collection : Vec<Vec<Vec<(f64, f64)>>> = Vec::new();
    for familiy in input_collection{
        let mut output_family : Vec<Vec<(f64, f64)>> = Vec::new();
        for polygon in familiy{
            let mut output_polygon : Vec<(f64, f64)> = Vec::new();
            for point in polygon{
                let output_point : (f64, f64) = convert_to_float_tuple(*point);
                output_polygon.push(output_point);
            }
            output_family.push(output_polygon);
        }
        output_family_collection.push(output_family);
    }

    output_family_collection
}

fn convert_to_float_tuple(integer_tuple : (i32, i32)) -> (f64, f64){
    let x_string = convert_to_degree_string(integer_tuple.0);
    let y_string = convert_to_degree_string(integer_tuple.1);

    let x = x_string.parse::<f64>().unwrap();
    let y = y_string.parse::<f64>().unwrap(); 


    let float_tuple = (x,y);
    float_tuple
}

fn get_geo_json_multi_polygon_string(polygon_family_collection :&Vec<Vec<Vec<(i32, i32)>>>) -> String{
    let mut geo_json_family_collection = "".to_string();
    for polygon_family in polygon_family_collection{
        let geo_json_family = get_geo_json_family(&polygon_family);  
        append_or_create(&mut geo_json_family_collection, geo_json_family);
    }
    
    let geojosn = format!(
        "{{
            \"type\": \"MultiPolygon\",
            \"coordinates\": 
            [
               {}
            ]
        }}",
        geo_json_family_collection   
        );
    geojosn
}

fn get_geo_json_family(polygon_family :&Vec<Vec<(i32, i32)>>) -> String{
    let mut geo_json_family = "[".to_string();
    let mut is_inner = false;
    for polygon in polygon_family{
        let mut geo_json_polygon = "".to_string();
        if is_inner{    //inner-Polygons has to be clockwise
            geo_json_polygon = get_geo_json_polygon(&reverse_order(polygon));   
        }else{
            geo_json_polygon= get_geo_json_polygon(&polygon);  
        }
        append_or_create(&mut geo_json_family, geo_json_polygon);
        is_inner = true;
    }
    geo_json_family.push_str("]");
    geo_json_family
}


fn get_geo_json_polygon(polygon :&Vec<(i32, i32)>) -> String{
    let mut geo_json_polygon = "[".to_string();
    for tuple in polygon{
        let geo_json_tuple = get_geo_json_tuple(&tuple);  
        append_or_create(&mut geo_json_polygon, geo_json_tuple);
    }
    geo_json_polygon.push_str("]");
    geo_json_polygon
}


fn get_geo_json_tuple(tuple :&(i32, i32)) -> String{
    let x = convert_to_degree_string(tuple.0);
    let y = convert_to_degree_string(tuple.1);
    format!("[{}, {}]", x,y)
}

//  -45592992 -> "-4.5592992" 
//  541397825 -> "54.1397825"
fn convert_to_degree_string(number_ten_power_minus_eight : i32) -> String{
    let number_string = number_ten_power_minus_eight.to_string();
    let mut degree_string = "".to_string();
    let mut character_index = 0;
    for character in number_string.chars(){
        degree_string.push_str(&character.to_string());

        if point_hast_do_be_set(character_index, number_string.len()){
            degree_string.push_str(".");
        }

        character_index = character_index + 1 ;
    }
    degree_string
}

fn point_hast_do_be_set(character_index :usize, total_length : usize)-> bool{
    total_length - character_index == 8
}

fn append_or_create(geo_json_string : &mut String, json_snippet : String){
    if geo_json_string.len() > 1{
        //not empty
        //append
        geo_json_string.push_str(&format!(",{}", &json_snippet));
    }
    else{
        geo_json_string.push_str(&json_snippet);
    }
}

fn reverse_order(polygon : &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut reversed :Vec<(i32, i32)> = Vec::new(); 
    for index in (0..polygon.len()).rev(){
        reversed.push(*polygon.get(index).unwrap());
    }
    reversed
}


//Poi-Area-Factory Struct
struct PoiAreaFactory {
    areas: Vec<PoiArea>,
    segments: HashMap<SegmentId, Segment>,
    nodes: HashMap<NodeId, Node>,
}

//Konstruktor
impl PoiAreaFactory {
    fn new() -> Self {
        PoiAreaFactory {
            areas: Vec::new(),
            segments: HashMap::new(),
            nodes: HashMap::new(),
        }
    }
}

//Interface for AreaFactory
impl parsers::AreaFactory for PoiAreaFactory {
    type Area = PoiArea;

    fn is_valid(&self, tags: &Tags) -> bool {
        return tags.get("name").is_some()
            && (tags.get("amenity").is_some()
                || tags.get("building").is_some()
                || tags.get("emergency").is_some()
                || tags.get("historic").is_some()
                || tags.get("leisure").is_some()
                || tags.get("man_made").is_some()
                || tags.get("military").is_some());
    }

    fn to_area(
        &self,
        rel: &Relation,
        inner_id_sender: &Sender<SegmentId>,
        outer_id_sender: &Sender<SegmentId>,
    ) -> Option<PoiArea> {
        assert!(self.is_valid(&rel.tags));

        //Parse ID
        let osmid = rel.id;

        //Parse name
        let name = match rel.tags.get("name") {
            Some(name) => name,
            None => {
                eprintln!("Could not get name of admin area {}", osmid.0);
                return None;
            }
        };

        //Parse Paths
        let mut inner: Vec<SegmentId> = Vec::new();
        let mut outer: Vec<SegmentId> = Vec::new();
        for r in &rel.refs {
            match r.role.as_str() {
                "inner" => match r.member {
                    osmpbfreader::OsmId::Way(oid) => {
                            inner_id_sender.send(oid).unwrap();
                            inner.push(r.member.way().unwrap());
                        }
                    _ => {
                        eprintln!("Inner relation id is not a WayId in area {}", osmid.0);
                    }
                },
                "outer" => match r.member {
                    osmpbfreader::OsmId::Way(oid) => {
                            outer_id_sender.send(oid).unwrap();
                            outer.push(r.member.way().unwrap());
                        }
                    _ => eprintln!("Inner relation id is not a WayId in area {}", osmid.0),
                },
                _ => ()//eprintln!("Ignoring relation role {}", r.role),
            }
        }

        Some(PoiArea {
            osmid: osmid,
            tags: rel.tags.clone(),
            name: name.clone(),
            inner: inner,
            outer: outer,
        })
    }

    fn set_segments(&mut self, segments: Vec<Segment>, nodes: Vec<Node>) {
        // TODO: remove clone
        self.segments
            .extend(segments.into_iter().map(|seg| (seg.osmid, seg)));
        self.nodes
            .extend(nodes.into_iter().map(|node| (node.osmid, node)));
    }
}


//------------------------------------------- PoiNode Komponenten -----------------------------
//Poi Node Struce
struct PoiNode {
    pub osmid: NodeId,
    pub tags: Tags,
    pub name: String,
    pub decimicro_lat: Latitude,
    pub decimicro_lon: Longitude,
}

//Poi-Node-Factory Struct
struct PoiNodeFactory {
    nodes: Vec<PoiNode>,
    nodes_map: HashMap<NodeId, Node>,
}

impl PoiNode{
    
    pub fn lon(&self) -> f64 {
        let mut result : f64 = self.decimicro_lon.into();
        result = result / 10_000_000.0;
        result
    }
    pub fn lat(&self) -> f64 {
        let mut result : f64 = self.decimicro_lat.into();
        result = result / 10_000_000.0;
        result
    }

}


//Konstruktor
impl PoiNodeFactory {
    fn new() -> Self {
        PoiNodeFactory {
            nodes: Vec::new(),
            nodes_map: HashMap::new()
        }
    }
}

//Interface for NodeFactory
impl PoiNodeFactory {

    fn is_valid(&self, tags: &Tags) -> bool {
        return tags.get("name").is_some()
            && (tags.get("amenity").is_some()
                || tags.get("building").is_some()
                || tags.get("emergency").is_some()
                || tags.get("historic").is_some()
                || tags.get("leisure").is_some()
                || tags.get("man_made").is_some()
                || tags.get("military").is_some());
    }

    fn to_poi_node(
        &self,
        osm_node: &osmpbfreader::Node,
    ) -> Option<PoiNode> {
        assert!(self.is_valid(&osm_node.tags));

        //Parse ID
        let osmid = osm_node.id;

        //Parse name
        let name = match osm_node.tags.get("name") {
            Some(name) => name,
            None => {
                eprintln!("Could not get name of node with OSM-ID {}", osmid.0);
                return None;
            }
        };

        Some(PoiNode {
            osmid: osmid,
            tags: osm_node.tags.clone(),
            name: name.clone(),
            decimicro_lat: osm_node.decimicro_lat,
            decimicro_lon: osm_node.decimicro_lon,
        })
    }
}




//------------------------------------------- import-calls ----------------------------------
pub fn import_admin_areas(path: &String) {
    let mut factory = AdminAreaFactory::new();

    let now = Instant::now();
    parsers::import_areas(&path, &mut factory);
    let runtime = now.elapsed();

    print_results(
        factory.areas.len(),
        factory.segments.len(),
        factory.nodes.len(),
        runtime,
    );
}

pub fn import_poi_areas(path: &String) {
    //TODO: make import_**_areas generic
    let mut factory = PoiAreaFactory::new();
    let now = Instant::now();

    let (areas, segments, nodes) = parsers::import_areas(&path, &mut factory);
    let runtime = now.elapsed();


    let anzahl_ausgaben = 2;
    println!("Ausgabe der Ergebnisse (beispielhaft {} Stück): ", anzahl_ausgaben);

    //ab hier nur beispielhafte Ausgabe. Später löschen.
    for i in 1..anzahl_ausgaben {
        println!("Area Nr {}:", i);

        let current_area_struct: &PoiArea = match areas.get(i) {
            Some(x) => x,
            _ => panic!("Out of Index: dieser Index existiert nicht"),
        };
        
        println!("------------------------\n toString: \n{}\n-----------------------------", current_area_struct.to_string());
        println!("GeoJSON:\n{}\n-----------------------------", current_area_struct.generate_geojson(&segments, &nodes));
        
    }

    print_results(
        factory.areas.len(),
        factory.segments.len(),
        factory.nodes.len(),
        runtime,
    )
}

pub fn import_poi_nodes(path : &String){
    let file = File::open(&path).expect("Could not open input file! Exiting!");
  
    let now = Instant::now();
    let factory = PoiNodeFactory::new();

    let mut pbf_reader = osmpbfreader::OsmPbfReader::new(file);
    let poi_nodes : Vec<PoiNode>= pbf_reader.par_iter()
                .filter_map(|obj| obj.ok())
                .filter(|obj| obj.is_node())
                .filter(|obj| factory.is_valid(obj.tags()))
                .filter_map(|obj|{
                    if let Some(node) = obj.node(){
                        return factory.to_poi_node(node)
                    }
                    None
                }).collect();
    let runtime = now.elapsed();
    println!("{} POI nodes found in {:?}", poi_nodes.len(), runtime) ;  

    //Export to geojson-Format


    //Example Output:
    let obj = poi_nodes.get(4).expect("Out of index");
    println!("{:?} {} {:?} {}", obj.osmid, obj.name, obj.tags, obj.lat());
    println!("{}", poi_note_to_geojson_feature_string(obj));

}

//------------------------------------------- Hilfsfunktionen ----------------------------------

fn poi_note_to_geojson_feature_string(node_struct : &PoiNode)-> String{
    // point-orientation: [lon, lat]
    let mut properties = String::from("");
    let mut is_first_round = true;
    for (key, value) in node_struct.tags.iter() {
       if is_first_round {
           is_first_round = false; 
       }
       else {
           properties.push_str(", ");
       }
       properties.push_str( &format!("\"{}\": \"{}\"", key, value));
    }
    let lon = node_struct.lon();
    let lat = node_struct.lat();
    let geo_jon_format  = format!(
    "{{
        \"type\": \"Feature\",
        \"geometry\": {{
            \"type\": \"Point\",
            \"coordinates\": [{}, {}]
        }},
        \"properties\": {{
                {}
        }}
    }}", lon, lat, properties);

    geo_jon_format
}


fn print_results(area_count: usize, segment_count: usize, nodes_count: usize, runtime: Duration) {
    //TODO: Use Factroy-Generic instead of single values.
    println!(
        "Imported {} areas, {} segments and {} nodes in {:?}",
        area_count, segment_count, nodes_count, runtime
    );
}

fn segment_vec_to_string(vec: &Vec<SegmentId>) -> String {
    let mut inner_str: String = "[".to_string();
    let mut is_first_round = true;

    for seg in vec {
        if is_first_round {
            is_first_round = false;
        } else {
            inner_str.push_str(&", ".to_string());
        }

        inner_str.push_str(&seg.0.to_string());
    }

    inner_str.push_str("]");
    inner_str
}

/*
Return-format:
    {"building"="yes", ... ,"name"="Isle of Man Government"}
*/
fn tags_to_string(tags: &Tags) -> String {
    let mut tags_str: String = "".to_string();
    let mut is_first_round = true;

    for (key, value) in tags.iter() {
        if is_first_round {
            is_first_round = false;
        } else {
            tags_str.push_str(&", ".to_string());
        }
        tags_str.push_str(&format!("\"{}\" : \"{}\"", key, value));
    }
    
    tags_str
}

//------------------------------------------- Tests ----------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn benchmark_stuttgart() {
        import_admin_areas(&"resources/pbfs/stuttgart-regbez-latest.osm.pbf".to_string());
    }
}



