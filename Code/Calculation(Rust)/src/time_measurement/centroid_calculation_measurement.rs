//-------------------------------Import----------------------------------------------------------------

//from local
use crate::parsers::polygon_from_file::load_from_fmi_lon_lat_format;
use crate::centroid_calculation::simple_algorithms::{get_centroid, get_geometrical_center};
use crate::centroid_calculation::point_of_inaccessibility::get_point_of_inaccessibility;
use crate::centroid_calculation::skeleton_centerpoint::skeleton_graph_generation::{Skeletongraph, generate_polygon_skeleton_graph, SkeletonType};
use crate::centroid_calculation::skeleton_centerpoint::triangulation::{generate_polygon_triangulation, Polygontriangulation};
use crate::centroid_calculation::skeleton_centerpoint::centrality_scores_calculation::{get_most_central_points, CentralityVariant,SearchingMethod, Weights};

//from external
use std::time::{Instant};

//-------------------------------Constants--------------------------------------------------------------


//------------------------------Structs----------------------------------------------------------------


//---------------------------------public API-function---------------------------------------------------

pub fn measure_algorithm_times(){
    let folder_path = String::from("resources/testpolygons/lon_lat_FMI_format/");
    let filenames  = ["triangulation_skeleton_graph", "bw", "brandenburg", "bremen", "niedersachsen", "nordrhein-westfahlen", "rheinlandpfalz", "saarland"];
    let file_ending = ".txt";

    for filename in filenames.iter(){
        //print name
        println!("\n----- Meassure: {} ----------", filename);

        //get full name
        let full_path = get_full_filename(&folder_path, filename, file_ending);

        //get input 
        let input = load_input(full_path);

        //meassure different calculations
        measure_centroid_calculation(&input);
        measure_geometric_center_calculation(&input);
        measure_point_of_inaccessibility_calculation(&input);
        measure_skeleton_centroid_calculation(&input);

    }
}

//---------------------------------private functions----------------------------------------------------

fn measure_geometric_center_calculation(input :  &Vec<Vec<Vec<(f64,f64)>>>){
    let pre_calculating_time = Instant::now();

    //calculate points
    for polygon_family in input{
        let (x,y) = get_geometrical_center(&polygon_family);
        //println!("({} {})", x,y);
    }

    let post_calculating_time = Instant::now();
    print_calculation_time(pre_calculating_time, post_calculating_time, "Geometric Center-Calculation");

}

fn measure_centroid_calculation(input :  &Vec<Vec<Vec<(f64,f64)>>>){
    let pre_calculating_time = Instant::now();
    
    //calculate points
    for polygon_family in input{
        let (x,y) = get_centroid(&polygon_family);
        //println!("({} {})", x,y);
    }

    let post_calculating_time = Instant::now();
    print_calculation_time(pre_calculating_time, post_calculating_time, "Centroid-Calculation");

}

fn measure_skeleton_centroid_calculation(input :  &Vec<Vec<Vec<(f64,f64)>>>){
    let pre_calculating_time = Instant::now();
    //triangulate
    let mut triangulations : Vec<Polygontriangulation> = Vec::new();
    for polygon_family in input{
        //calculate points
        let triangulation : Polygontriangulation = generate_polygon_triangulation(&polygon_family);
        triangulations.push(triangulation);
    }
    let post_triangulation_calculating_time = Instant::now();

    //calc skeleton-graph
    let mut skeletongraphs : Vec<Skeletongraph> = Vec::new(); 
    for triangulation in triangulations{
        let skeletongraph : Skeletongraph = generate_polygon_skeleton_graph(&triangulation, SkeletonType::AignerApproach);
        skeletongraphs.push(skeletongraph);
    } 
    let post_skeleton_calculating_time = Instant::now();

    
    print_calculation_time(pre_calculating_time, post_triangulation_calculating_time, "Triangulation");
    print_calculation_time(post_triangulation_calculating_time, post_skeleton_calculating_time, "Skeleton");
    
    //calc most central point(s)    
    let methods = [
        CentralityVariant::Betweenness(SearchingMethod::AStar1To1),
        CentralityVariant::Betweenness(SearchingMethod::Disjkstra1ToAll),
        CentralityVariant::Closeness(SearchingMethod::Disjkstra1ToAll, Weights::CoverLength),
    ];
    meassure_all_most_central_point_methods(&skeletongraphs, &methods);
    let post_calculating_time = Instant::now();

    print_calculation_time(post_skeleton_calculating_time, post_calculating_time, "total Skeleton Centroid-Calculation");
}

fn meassure_all_most_central_point_methods(skeletongraphs : &Vec<Skeletongraph>, methods: &[CentralityVariant]){    
    for method in methods{
        let pre_method_calculating_time = Instant::now();
        for skeletongraph in skeletongraphs{
                let most_central_points : Vec<(f64, f64)>  = get_most_central_points(&skeletongraph, &method);
                //println!("{:?}", most_central_points);
        }
        let post_method_calculating_time = Instant::now();
        let label = format!("most central points:  {:?}", method);
        print_calculation_time(pre_method_calculating_time, post_method_calculating_time, &label );
    }

}

fn measure_point_of_inaccessibility_calculation(input :  &Vec<Vec<Vec<(f64,f64)>>>){
    let pre_calculating_time = Instant::now();
    
    //calculate points
    let precision = 0.1 as f64;
    for polygon_family in input{
        let (x,y,distance) = get_point_of_inaccessibility(&polygon_family, precision);
        //println!("({} {}) Distance: {}", x,y, distance);
    }

    let post_calculating_time = Instant::now();
    let labeling = format!("Point of Inaccessibility-Calculation ({})", precision);
    print_calculation_time(pre_calculating_time, post_calculating_time, &labeling);
}
   



//----------------Helper-functions-------------------------

fn get_full_filename(folder_path : &String, filename : &str, file_ending: &str) -> String{
     //get full Name
    let mut full_path = folder_path.clone();
    full_path.push_str(filename);
    full_path.push_str(file_ending);
    
    full_path
}

fn count_points_of_structure(input : &Vec<Vec<Vec<(f64,f64)>>>) -> usize{
    let mut counter = 0;
    for polygon_family in input{
        for polygon in polygon_family{
                counter = counter + polygon.len();
         }
    }

    counter
}

fn print_calculation_time(pre : Instant, post :Instant, method : &str){
    let difference = post.duration_since(pre);
    println!("Time needed for {} \n     {:?}", method, difference);
}

fn load_input(full_path :String) -> Vec<Vec<Vec<(f64,f64)>>> {
    //load Input
    let pre_loading = Instant::now();
    let input : Vec<Vec<Vec<(f64,f64)>>>  = load_from_fmi_lon_lat_format(&full_path);
    let after_loading = Instant::now();
    
    //count points
    let count_points =  count_points_of_structure(&input);

    //output time needed
    let label = &format!("Loading Polygon ({} Points)", count_points);
    print_calculation_time(pre_loading, after_loading, label);
    
    input
}