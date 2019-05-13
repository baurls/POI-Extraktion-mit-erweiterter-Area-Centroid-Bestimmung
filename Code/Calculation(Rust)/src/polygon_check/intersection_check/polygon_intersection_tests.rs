//--------------------------------------tests-----------------------------------------------------
#[cfg(test)]
    use super::polygon_is_self_intersecting;
    use super::IntersectionResult;
    use super::IntersectionType;

    #[test]
    fn test_all(){
        simple_polygon_is_self_intersecting_test();
        polygon_is_self_intersecting_test1();
        polygon_is_self_intersecting_test2();
        polygon_is_self_intersecting_test3();
        polygon_is_self_intersecting_test4();
      
    }

    #[test]

     fn simple_polygon_is_self_intersecting_test(){
       //No Intersection
       let polygon1 = vec![(0f64, 0f64),(5f64, 0f64),(2.5f64, 3f64),(0f64, 0f64)];
       let polygon2 = vec![(0f64, 0f64),(5f64, 0f64),(2.5f64, 3f64), (2.5f64,4f64), (2.5f64, 6f64) ,(0f64, 0f64)];
       let polygon3 = vec![(1f64, 1f64), (4f64, 1f64), (4f64, 2f64), (4f64, 3f64), (5f64, 3f64),(6f64, 3f64), (6f64, 6f64), (4f64, 5f64),(4f64, 8f64), (2f64, 5f64), (-1f64, 4f64),(-3f64, 2f64), (-2f64, -2f64), (0f64, -5f64),(4f64, -3f64), (4f64, -1f64), (1f64, 1f64)];
       // let points = vec![(f64, f64),(f64, f64),(f64, f64),(f64, f64)];
       let result1  = polygon_is_self_intersecting(&polygon1);
       assert_eq!(result1, IntersectionResult::HasNoIntersection);
       let result2  = polygon_is_self_intersecting(&polygon2);
       assert_eq!(result2, IntersectionResult::HasNoIntersection);
       let result3  = polygon_is_self_intersecting(&polygon3);
       assert_eq!(result3, IntersectionResult::HasNoIntersection);
    }
    

    #[test]
     fn polygon_is_self_intersecting_test1(){
        //in a Endpoint
       let polygon = vec![(0f64, 0f64),(1f64, 1f64),(3f64, 1f64),(3f64, 3f64),(2f64, 3f64),(3f64, 1f64),(0f64, 0f64)];
       let polygon2 = vec![(1f64, 1f64), (-1f64, 4f64), (4f64, 2f64), (4f64, 3f64), (5f64, 3f64),(6f64, 3f64), (6f64, 6f64), (4f64, 5f64),(4f64, 8f64), (2f64, 5f64), (-1f64, 4f64),(-3f64, 2f64), (-2f64, -2f64), (0f64, -5f64),(4f64, -3f64), (4f64, -1f64), (1f64, 1f64)];
       
       let result  = polygon_is_self_intersecting(&polygon);
       assert_eq!(result, IntersectionResult::HasIntersection(IntersectionType::InAEndpoint));
       let result2  = polygon_is_self_intersecting(&polygon2);
       assert_eq!(result2, IntersectionResult::HasIntersection(IntersectionType::InAEndpoint));
    }


    #[test]
     fn polygon_is_self_intersecting_test2(){
        //"real" intersection
       let polygon = vec![(0f64, 0f64),(5f64, 0f64),(2f64, 2f64),(2f64, -2f64),(0f64, 0f64)];
       let polygon2 = vec![(-5f64, -5f64), (4f64, 1f64), (4f64, 2f64), (4f64, 3f64), (5f64, 3f64),(6f64, 3f64), (6f64, 6f64), (4f64, 5f64),(4f64, 8f64), (2f64, 5f64), (-1f64, 4f64),(-3f64, 2f64), (-2f64, -2f64), (0f64, -5f64),(4f64, -3f64), (4f64, -1f64), (-5f64, -5f64)];
       
       
       let result  = polygon_is_self_intersecting(&polygon);
       assert_eq!(result, IntersectionResult::HasIntersection(IntersectionType::InAPointInbetween));
       let result2  = polygon_is_self_intersecting(&polygon2);
       assert_eq!(result2, IntersectionResult::HasIntersection(IntersectionType::InAPointInbetween));
    }

    
    #[test]
     fn polygon_is_self_intersecting_test3(){
        //in a line
       let polygon = vec![(2f64, 0f64),(2f64, 2f64),(4f64, 2f64),(2f64, -5f64),(2f64, 10f64),(0f64, 0f64),(2f64, 0f64),];
       
       let result  = polygon_is_self_intersecting(&polygon);
       assert_eq!(result, IntersectionResult::HasIntersection(IntersectionType::InALine));
    }

    
    #[test]
     fn polygon_is_self_intersecting_test4(){
        //in a T point
       let polygon = vec![(0f64, 0f64),(2f64, 2f64),(2f64, 0f64),(1f64, 1f64),(0f64, 3f64),(0f64, 0f64)];
       let polygon2 = vec![(1f64, 1f64), (4f64, 1f64), (4f64, 2f64), (4f64, 3f64), (5f64, 3f64),(6f64, 3f64), (6f64, 6f64), (4f64, 5f64),(4f64, 8f64), (2f64, 5f64), (-1f64, 4f64),(-3f64, 2f64), (-2f64, -2f64), (2.5f64, 0f64),(4f64, -3f64), (4f64, -1f64), (1f64, 1f64)];
       
       
       let result  = polygon_is_self_intersecting(&polygon);
       assert_eq!(result, IntersectionResult::HasIntersection(IntersectionType::T_Intersection));
       let result2  = polygon_is_self_intersecting(&polygon2);
       assert_eq!(result2, IntersectionResult::HasIntersection(IntersectionType::T_Intersection));
    }

    