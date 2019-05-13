extern crate library;

fn main() {
    println!("Hello from main function");
    /*
    Admin areas
        library::import_admin_areas(&"resources/pbfs/isle-of-man-latest.osm.pbf".to_string());
        library::import_admin_areas(&"resources/pbfs/stuttgart-regbez-latest.osm.pbf".to_string());
    POI Areas
        library::import_poi_areas(&"resources/pbfs/stuttgart-regbez-latest.osm.pbf".to_string());
    POI Nodes
        library::import_poi_nodes(&"resources/pbfs/stuttgart-regbez-latest.osm.pbf".to_string());
    
    */ 
    
    //library::test_geojson_geration();
    //println!("{:?}", polygonlist);
    //library::test_polygon_sorting();
    library::import_poi_areas(&"resources/pbfs/isle-of-man-latest.osm.pbf".to_string());
    //library::test_a_star();
   
    //library::test_polylabel_calculation();        
    //library::test_polylabel_calculation(); 
    //library::import_poi_nodes(&"resources/pbfs/isle-of-man-latest.osm.pbf".to_string());
    //library::test_polygon_skeleton_centrality();
    //library::test_clean_and_get_center_point();
  
     //library::measure_algorithm_times();
   
}

 