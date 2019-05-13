//--------------------------------------tests-----------------------------------------------------
#[cfg(test)]
    use super::*;

    #[test]
    fn test_all(){
        get_point_and_paths_test1();
        get_point_and_paths_test2();
        are_parallel_test();
        are_parallel_test2();
        parallel_segments_intersect_case_1_3_test();
        parallel_segments_intersect_case_1_2_test();
        parallel_segments_intersect_case_1_1_test();
        non_parallel_intersect_test();
        endpoints_intersect_test();
        are_intersecting_test();

    }
    
    #[test]
    fn get_point_and_paths_test1() {
        let A = (0f64, 5f64);
        let B = (5f64, 0f64);

        let p = (0f64, 3f64);
        let q = (3f64, 0f64);

        assert_eq!((A,B), get_upper_path((A,B), (p,q) ) );
        assert_eq!((p,q), get_lower_path((A,B), (p,q) ) ); 

        assert_eq!(B, get_bottom(A,B));
        assert_eq!(A, get_top(A,B));
        assert_eq!(A, get_left(A,B));
        assert_eq!(B, get_right(A,B));  

        assert_eq!(q, get_bottom(p,q));
        assert_eq!(p, get_top(p,q));
        assert_eq!(p, get_left(p,q));
        assert_eq!(q, get_right(p,q));  
    }

    #[test]
    fn get_point_and_paths_test2() {
        let A = (0f64, 10f64);
        let B = (0f64, 0f64);

        let p = (0f64, 13f64);
        let q = (0f64, 5f64);

        assert_eq!((p,q), get_upper_path((A,B), (p,q) ) );
        assert_eq!((A,B), get_lower_path((A,B), (p,q) ) ); 

        assert_eq!(B, get_bottom(A,B));
        assert_eq!(A, get_top(A,B));
        assert_eq!(A.0, get_left(A,B).0);
        assert_eq!(B.0, get_left(A,B).0);
        assert_eq!(B.0, get_right(A,B).0);
        assert_eq!(A.0, get_right(A,B).0);  

        assert_eq!(q, get_bottom(p,q));
        assert_eq!(p, get_top(p,q));
        assert_eq!(p.0, get_left(p,q).0);
        assert_eq!(q.0, get_left(p,q).0);
        assert_eq!(q.0, get_right(p,q).0);
        assert_eq!(p.0, get_right(p,q).0);
    }

      #[test]
    fn are_parallel_test() {
        let A = (0f64, 10f64);
        let B = (0f64, 0f64);

        let p = (0f64, 13f64);
        let q = (0f64, 5f64);

        let line1 = (B,A);
        let line2 = (p,q);

        assert!(are_parallel(line1, line2));
    }
    
      #[test]
    fn are_parallel_test2() {
        let A = (1f64, 1f64);
        let B = (4f64, 4f64);

        let p = (4f64, 0f64);
        let q = (5f64, 1f64);

        assert!(are_parallel( (A,B) , (p,q) ));
        assert!(!are_parallel( (A,p) , (B,q) ));
        assert!(!are_parallel( (A,q) , (B,q) ));
        assert!(!are_parallel( (A,q) , (B,p) ));
        assert!(!are_parallel( (A,p) , (B,p) ));
        assert!(are_parallel( (B,A) , (p,q) ));
        assert!(are_parallel( (B,A) , (q,p) ));
        assert!(are_parallel( (B,A) , (B,A) ));
        assert!(are_parallel( (p,q) , (p,q) ));
        assert!(are_parallel( (p,q) , (p,q) ));
    }


    #[test]
    fn parallel_segments_intersect_case_1_3_test(){
        let A = (1f64, 1f64);
        let B = (4f64, 4f64);

        let p = (4f64, 0f64);
        let q = (5f64, 1f64);

        let l = (1f64, 1f64);
        let m = (3f64, 2f64);

        let x = (-1f64, 0f64);
        let y = (5f64, 3f64);

        let s = (-1f64, 0f64);
        let t = (0f64, 0.5f64);

        
        assert!(! parallel_segments_intersect( (A,B), (p,q) )   );
        assert!(! parallel_segments_intersect( (B,A), (p,q) )   );
        assert!( parallel_segments_intersect( (l,m), (x,y) )   );
        assert!(! parallel_segments_intersect( (s,t), (l,m) )   );
        assert!( parallel_segments_intersect( (t,s), (x,y) )   );
       
    }


    #[test]
    fn parallel_segments_intersect_case_1_2_test(){
        let A = (1f64, 1f64);
        let B = (1f64, 4f64);

        let l = (3.5f64, 2f64);
        let m = (3.5f64, 5f64);

        let s = (1f64, 1.5f64);
        let t = (1f64, 3f64);

        
        assert!( parallel_segments_intersect( (A,B), (t,s) )   );
        assert!(! parallel_segments_intersect( (B,t), (s,A) )   );
        assert!(! parallel_segments_intersect( (A,B), (l,m) )   );
        assert!(! parallel_segments_intersect( (s,t), (l,m) )   );
        assert!(! parallel_segments_intersect( (t,A), (l,m) )   );
        assert!(! parallel_segments_intersect( (s,A), (l,m) )   );
       
    }


    #[test]
    fn parallel_segments_intersect_case_1_1_test(){
        let A = (1f64, 1f64);
        let B = (4f64, 1f64);

        let l = (2f64, 3.5f64);
        let m = (5f64, 3.5f64);

        let s = (1.5f64, 1f64);
        let t = (3f64, 1f64);

        
        assert!( parallel_segments_intersect( (A,B), (t,s) )   );
        assert!(! parallel_segments_intersect( (B,t), (s,A) )   );
        assert!(! parallel_segments_intersect( (A,B), (l,m) )   );
        assert!(! parallel_segments_intersect( (s,t), (l,m) )   );
        assert!(! parallel_segments_intersect( (t,A), (l,m) )   );
        assert!(! parallel_segments_intersect( (s,A), (l,m) )   );
       
    }
    
    #[test]
    fn non_parallel_intersect_test(){
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InAPointInbetween),  non_parallel_intersect(((1f64, 1f64), (6f64, 2f64)), ((3f64, 0f64), (8f64, 7f64))));
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InAPointInbetween),  non_parallel_intersect(((3f64, 0f64), (8f64, 7f64)), ((1f64, 5f64), (8f64, 5f64))));
        
        assert_eq!(IntersectionResult::HasNoIntersection, non_parallel_intersect(((1f64, 5f64), (8f64, 5f64)), ((7f64, 4f64), (11f64, 1f64))));
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::T_Intersection), non_parallel_intersect(((1f64, 5f64), (8f64, 5f64)), ((6f64, 5f64), (11f64, 1f64))));

        assert_eq!(IntersectionResult::HasNoIntersection, non_parallel_intersect(((1f64, 1f64), (6f64, 2f64)), ((1f64, 5f64), (8f64, 5f64))));
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InAPointInbetween), non_parallel_intersect(((2f64, 1f64), (6f64, 2f64)), ((8f64, 7f64), (3f64, 0f64))));
        assert_eq!(IntersectionResult::HasNoIntersection, non_parallel_intersect(((2f64, 1f64), (11f64, 1f64)), ((6f64, 2f64), (8f64, 7f64))));
    }  
 

    #[test]
    fn endpoints_intersect_test(){
        assert_eq!(false, endpoints_intersect(((1f64, 1f64), (6f64, 2f64)), ((3f64, 0f64), (8f64, 7f64))));
        assert_eq!(false,  endpoints_intersect(((3f64, 0f64), (8f64, 7f64)), ((1f64, 5f64), (8f64, 5f64))));
        assert_eq!(false, endpoints_intersect(((1f64, 5f64), (8f64, 5f64)), ((7f64, 4f64), (11f64, 1f64))));
        assert_eq!(false, endpoints_intersect(((1f64, 5f64), (8f64, 5f64)), ((6f64, 5f64), (11f64, 1f64))));
        assert_eq!(false, endpoints_intersect(((1f64, 1f64), (6f64, 2f64)), ((1f64, 5f64), (8f64, 5f64))));
        assert_eq!(false, endpoints_intersect(((2f64, 1f64), (6f64, 2f64)), ((8f64, 7f64), (3f64, 0f64))));
        assert_eq!(false, endpoints_intersect(((2f64, 1f64), (11f64, 1f64)), ((6f64, 2f64), (8f64, 7f64))));

        assert_eq!(true, endpoints_intersect(((2f64, 1f64), (11f64, 1f64)), ((11f64, 1f64), (8f64, 7f64))));    //10
        assert_eq!(true, endpoints_intersect(((2f64, 1f64), (11f64, 1f64)), ((2f64, 1f64), (8f64, 7f64))));     //00
        assert_eq!(true, endpoints_intersect(((2f64, 1f64), (11f64, 1f64)), ((22f64, 13f64), (2f64, 1f64))));   //01
        assert_eq!(true, endpoints_intersect(((2f64, 1f64), (11f64, 1f64)), ((22f64, 13f64), (11f64, 1f64)))); //11


        assert_eq!(true, endpoints_intersect(((2f64, 1f64), (11f64, 1f64)), ((2f64, 1f64), (11f64, 1f64)))); //xx
    }  


     #[test]
    fn are_intersecting_test(){
        
        //Intersection Non-Parallel
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InAPointInbetween),  are_intersecting(((1f64, 1f64), (6f64, 2f64)), ((3f64, 0f64), (8f64, 7f64))));
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InAPointInbetween),  are_intersecting(((3f64, 0f64), (8f64, 7f64)), ((1f64, 5f64), (8f64, 5f64))));
        
        assert_eq!(IntersectionResult::HasNoIntersection, are_intersecting(((1f64, 5f64), (8f64, 5f64)), ((7f64, 4f64), (11f64, 1f64))));
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::T_Intersection), are_intersecting(((1f64, 5f64), (8f64, 5f64)), ((6f64, 5f64), (11f64, 1f64))));

        assert_eq!(IntersectionResult::HasNoIntersection, are_intersecting(((1f64, 1f64), (6f64, 2f64)), ((1f64, 5f64), (8f64, 5f64))));
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InAPointInbetween), are_intersecting(((2f64, 1f64), (6f64, 2f64)), ((8f64, 7f64), (3f64, 0f64))));
        assert_eq!(IntersectionResult::HasNoIntersection, are_intersecting(((2f64, 1f64), (11f64, 1f64)), ((6f64, 2f64), (8f64, 7f64))));
    
        //Intersection Parallel
        
            //Fall "__"
                //davor
        assert_eq!(IntersectionResult::HasNoIntersection, are_intersecting(((0f64, 5f64), (5f64, 5f64)), ((-3f64, 5f64), (-0.5f64, 5f64))));
        assert_eq!(IntersectionResult::HasNoIntersection, are_intersecting(((0f64, 5f64), (5f64, 5f64)), ((-0.5f64, 5f64), (-3f64, 5f64))));
                //rechts_drin
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InALine), are_intersecting(((0f64, 5f64), (5f64, 5f64)), ((-0.5f64, 5f64), (2f64, 5f64))));
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InALine), are_intersecting(((0f64, 5f64), (5f64, 5f64)), ((2f64, 5f64), (-0.25f64, 5f64))));
                //beide drin
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InALine), are_intersecting(((0f64, 5f64), (5f64, 5f64)), ((1f64, 5f64), (2f64, 5f64))));
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InALine), are_intersecting(((0f64, 5f64), (5f64, 5f64)), ((4f64, 5f64), (2f64, 5f64))));
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InALine), are_intersecting(((0f64, 5f64), (5f64, 5f64)), ((3f64, 5f64), (4.99f64, 5f64))));
                //links drin
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InALine), are_intersecting(((0f64, 5f64), (5f64, 5f64)), ((3f64, 5f64), (20f64, 5f64))));
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InALine), are_intersecting(((0f64, 5f64), (5f64, 5f64)), ((15f64, 5f64), (4.893f64, 5f64))));
                //danach
        assert_eq!(IntersectionResult::HasNoIntersection, are_intersecting(((0f64, 5f64), (5f64, 5f64)), ((6f64, 5f64), (12f64, 5f64))));
        assert_eq!(IntersectionResult::HasNoIntersection, are_intersecting(((0f64, 5f64), (5f64, 5f64)), ((87f64, 5f64), (14f64, 5f64))));
                //überdeckend
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InALine), are_intersecting(((0f64, 5f64), (5f64, 5f64)), ((-2f64, 5f64), (10f64, 5f64))));
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InALine), are_intersecting(((0f64, 5f64), (5f64, 5f64)), ((-20f64, 5f64), (5.001f64, 5f64))));
            
            //Fall "|"
                 //davor
        assert_eq!(IntersectionResult::HasNoIntersection, are_intersecting(((5f64, 0f64), (5f64, 5f64)), ((5f64, -3f64), (-5f64, -0.5f64))));
        assert_eq!(IntersectionResult::HasNoIntersection, are_intersecting(((5f64, 0f64), (5f64, 5f64)), (( 5f64,-0.5f64), (5f64,-3f64))));
                //rechts_drin
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InALine), are_intersecting(((5f64, 0f64), (5f64, 5f64)), ((5f64, -0.5f64), (5f64, 2f64))));
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InALine), are_intersecting(((5f64, 0f64), (5f64, 5f64)), ((5f64, 2f64), (5f64, -0.25f64))));
                //beide drin
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InALine), are_intersecting(((5f64, 0f64), (5f64, 5f64)), ((5f64, 1f64), (5f64, 2f64))));
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InALine), are_intersecting(((5f64, 0f64), (5f64, 5f64)), ((5f64, 4f64), (5f64, 2f64))));
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InALine), are_intersecting(((5f64, 0f64), (5f64, 5f64)), ((5f64, 3f64), (5f64, 4.99f64))));
                //links drin
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InALine), are_intersecting(((5f64, 0f64), (5f64, 5f64)), ((5f64, 3f64,), (5f64, 20f64))));
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InALine), are_intersecting(((5f64, 0f64), (5f64, 5f64)), ((5f64, 15f64), (5f64, 4.893f64))));
                //danach
        assert_eq!(IntersectionResult::HasNoIntersection, are_intersecting(((5f64, 0f64), (5f64, 5f64)), ((5f64, 6f64), (5f64, 12f64,))));
        assert_eq!(IntersectionResult::HasNoIntersection, are_intersecting(((5f64, 0f64), (5f64, 5f64)), ((5f64, 87f64), (5f64,14f64))));
                //überdeckend
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InALine), are_intersecting(((5f64, 0f64), (5f64, 5f64)), ((5f64, -2f64), (5f64, 10f64))));
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InALine), are_intersecting(((5f64, 0f64), (5f64, 5f64)), ((5f64, -20f64), (5f64, 5.001f64))));
            
            // Fälle "\"" oder "/""
         //davor
        assert_eq!(IntersectionResult::HasNoIntersection, are_intersecting(((0f64, 0f64), (4f64, 2f64)), ((-4f64, -2f64), (-2f64, -1f64))));
                //rechts_drin
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InALine), are_intersecting(((0f64, 0f64), (4f64, 2f64)), ((-2f64, -1f64), (2f64, 1f64))));
                //beide drin
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InALine), are_intersecting(((0f64, 0f64), (4f64, 2f64)), ((2f64, 1f64), (3f64, 1.5f64))));
                //links drin
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InALine), are_intersecting(((0f64, 0f64), (4f64, 2f64)), ((2f64, 1f64,), (7f64, 3.5f64))));
                //danach
        assert_eq!(IntersectionResult::HasNoIntersection, are_intersecting(((0f64, 0f64), (4f64, 2f64)), ((10f64, 5f64), (11f64, 5.5f64,))));
                //überdeckend
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InALine), are_intersecting(((0f64, 0f64), (4f64, 2f64)), ((-2f64, -1f64), (12f64, 6f64))));
        //davor
        assert_eq!(IntersectionResult::HasNoIntersection, are_intersecting(((0f64, 0f64), (-4f64, 2f64)), ((4f64, -2f64), (2f64, -1f64))));
                //rechts_drin
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InALine), are_intersecting(((0f64, 0f64), (-4f64, 2f64)), ((2f64, -1f64), (-2f64, 1f64))));
                //beide drin
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InALine), are_intersecting(((0f64, 0f64), (-4f64, 2f64)), ((-2f64, 1f64), (-3f64, 1.5f64))));
                //links drin
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InALine), are_intersecting(((0f64, 0f64), (-4f64, 2f64)), ((-2f64, 1f64,), (-7f64, 3.5f64))));
                //danach
        assert_eq!(IntersectionResult::HasNoIntersection, are_intersecting(((0f64, 0f64), (-4f64, 2f64)), ((-10f64, 5f64), (-11f64, 5.5f64,))));
                //überdeckend
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InALine), are_intersecting(((0f64, 0f64), (-4f64, 2f64)), ((2f64, -1f64), (-12f64, 6f64))));
     

        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InALine), are_intersecting(((3f64, 0f64), (8f64, 7f64)), ((5.5f64, 3.5f64), (13f64, 14f64))));
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InALine), are_intersecting(((3f64, 0f64), (8f64, 0f64)), ((3.1f64, 0f64), (13f64, 0f64))));
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InALine), are_intersecting(((3f64, 0f64), (8f64, 0f64)), ((3.1f64, 0f64), (7.5f64, 0f64))));
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InALine), are_intersecting(((3f64, 0f64), (8f64, 0f64)), ((2.1f64, 0f64), (13f64, 0f64))));
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InALine), are_intersecting(((3f64, 0f64), (8f64, 0f64)), ((2.1f64, 0f64), (4f64, 0f64))));
    
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InALine), are_intersecting(((1f64, 3f64), (1f64, 8f64)), ((1f64, 3.1f64), (1f64, 13f64))));
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InALine), are_intersecting(((1f64, 3f64), (1f64, 8f64)), ((1f64, 3.1f64), (1f64, 7f64))));
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InALine), are_intersecting(((1f64, 3f64), (1f64, 8f64)), ((1f64, 2.1f64), (1f64, 13f64))));
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InALine), are_intersecting(((1f64, 3f64), (1f64, 8f64)), ((1f64, 2.1f64), (1f64, 4f64))));

        
        
        //Intersection T-Edge
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::T_Intersection), are_intersecting(((1f64, 5f64), (8f64, 5f64)), ((0f64, 0f64), (3f64, 5f64))));
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::T_Intersection), are_intersecting(((1f64, 5f64), (8f64, 5f64)), ((3f64, 5f64), (10f64, 50f64))));
        assert_eq!(IntersectionResult::HasNoIntersection, are_intersecting(((1f64, 5f64), (8f64, 5f64)), ((9f64, 5f64), (10f64, 50f64))));

        //Intersection in endpoint
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InAEndpoint), are_intersecting(((1f64, 5f64), (8f64, 5f64)), ((8f64, 5f64), (10f64, 50f64))));
        assert_eq!(IntersectionResult::HasIntersection(IntersectionType::InAEndpoint), are_intersecting(((1f64, 5f64), (8f64, 5f64)), ((1f64, 5f64), (3f64, 4f64))));

        
        

    }  
