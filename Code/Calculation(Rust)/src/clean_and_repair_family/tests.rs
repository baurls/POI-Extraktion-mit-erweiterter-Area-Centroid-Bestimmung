//--------------------------------------tests-----------------------------------------------------
#[cfg(test)]
    use super::*;

    #[test]
    fn test_all(){
        reapir_or_clean_polygon_test();
        clean_and_repair_family_test();
    }
    
 #[test]
    fn reapir_or_clean_polygon_test(){
        //keep Polygons
        let polygon1 = vec![(0f64, 0f64),(5f64, 0f64),(2.5f64, 3f64),(0f64, 0f64)];
        let polygon2 = vec![(0f64, 0f64),(5f64, 0f64),(2.5f64, 3f64), (2.5f64,4f64), (2.5f64, 6f64) ,(0f64, 0f64)];
        let polygon3 = vec![(1f64, 1f64), (4f64, 1f64), (4f64, 2f64), (4f64, 3f64), (5f64, 3f64),(6f64, 3f64), (6f64, 6f64), (4f64, 5f64),(4f64, 8f64), (2f64, 5f64), (-1f64, 4f64),(-3f64, 2f64), (-2f64, -2f64), (0f64, -5f64),(4f64, -3f64), (4f64, -1f64), (1f64, 1f64)];
               
        assert_eq!(PolygonCleaningResult::Original, reapir_or_clean_polygon(&polygon1));
        assert_eq!(PolygonCleaningResult::Original, reapir_or_clean_polygon(&polygon2));
        assert_eq!(PolygonCleaningResult::Original, reapir_or_clean_polygon(&polygon3));



        //repair Polygons
        let polygon4 = vec![(0f64, 0f64),(5f64, 0f64),(2.5f64, 3f64), (2.5f64,4f64), (2.5f64, 6f64)];
        let polygon5 = vec![(0f64, 0f64),(5f64, 0f64),(2.5f64, 3f64), (2.5f64,4f64), (2.5f64, 6f64) ,(0f64, 0f64)];
        let polygon6 = vec![(0f64, 0f64),(5f64, 0f64),(2.5f64, 3f64)];
        let polygon6_repaired = vec![(0f64, 0f64),(5f64, 0f64),(2.5f64, 3f64),(0f64, 0f64)];

        assert_eq!(PolygonCleaningResult::Repaired(polygon2.clone()), reapir_or_clean_polygon(&polygon4));
        assert_ne!(PolygonCleaningResult::Repaired(polygon2.clone()), reapir_or_clean_polygon(&polygon5));
        assert_eq!(PolygonCleaningResult::Repaired(polygon6_repaired), reapir_or_clean_polygon(&polygon6));



        //unrepairable Polygons
        let mut polygon7 = polygon6.clone();
        polygon7.clear();
        let polygon8 = vec![(0f64, 0f64),(5f64, 0f64)];

        assert_eq!(PolygonCleaningResult::Empty, reapir_or_clean_polygon(&polygon7));
        assert_eq!(PolygonCleaningResult::Empty, reapir_or_clean_polygon(&polygon8));
        
    }

    #[test]
    fn clean_and_repair_family_test() {
        //OK
        let polygon1 = vec![(0f64, 0f64),(5f64, 0f64),(2.5f64, 3f64),(0f64, 0f64)];
        let polygon2 = vec![(0f64, 0f64),(5f64, 0f64),(2.5f64, 3f64), (2.5f64,4f64), (2.5f64, 6f64) ,(0f64, 0f64)];
        let polygon3 = vec![(1f64, 1f64), (4f64, 1f64), (4f64, 2f64), (4f64, 3f64), (5f64, 3f64),(6f64, 3f64), (6f64, 6f64), (4f64, 5f64),(4f64, 8f64), (2f64, 5f64), (-1f64, 4f64),(-3f64, 2f64), (-2f64, -2f64), (0f64, -5f64),(4f64, -3f64), (4f64, -1f64), (1f64, 1f64)];
        let polygon_family1 = vec![polygon1.clone(), polygon2.clone(), polygon3.clone()]; 
        let polygon_family2 = vec![polygon2.clone(), polygon2.clone(), polygon2.clone()]; 
        
        
        //empty testing
        let polygon5 = vec![(0f64, 0f64), (0f64, 0f64)];
        let polygon6 = vec![(0f64, 0f64)];
        let polygon_family3 =       vec![polygon1.clone(), polygon5.clone(), polygon6.clone()]; 
        let polygon_family3_res =   vec![polygon1.clone()]; 
        let polygon_family4 =       vec![polygon5.clone(), polygon1.clone(), polygon1.clone()]; 
        let polygon_family4_res : Vec<Vec<(f64, f64)>> =   Vec::new(); 

        //repairing testing
        let polygon7 = vec![(0f64, 0f64),(5f64, 0f64),(2.5f64, 3f64),(0f64, 0f64)];
        let polygon8 = vec![(0f64, 0f64),(7f64, 8f64),(7f64, 10f64), (0f64,20f64), (0f64, 0f64)];

        let polygon7_err = vec![(0f64, 0f64),(5f64, 0f64),(2.5f64, 3f64)];
        let polygon8_err = vec![(0f64, 0f64),(7f64, 8f64),(7f64, 10f64), (0f64,20f64)];
        

        let polygon_family5 =       vec![polygon7_err.clone(), polygon8_err.clone(), polygon5.clone()]; 
        let polygon_family5_res =   vec![polygon7.clone(), polygon8.clone()]; 


   
        //true
        assert_eq!(FamilyCleaningResult::Succeeded(polygon_family1.clone()),     clean_and_repair_family(&polygon_family1));
        assert_eq!(FamilyCleaningResult::Succeeded(polygon_family2.clone()),     clean_and_repair_family(&polygon_family2));

        assert_eq!(FamilyCleaningResult::Succeeded(polygon_family3_res.clone()), clean_and_repair_family(&polygon_family3));
        assert_eq!(FamilyCleaningResult::Failed,                                 clean_and_repair_family(&polygon_family4));


        assert_eq!(FamilyCleaningResult::Succeeded(polygon_family5_res.clone()), clean_and_repair_family(&polygon_family5));
}
