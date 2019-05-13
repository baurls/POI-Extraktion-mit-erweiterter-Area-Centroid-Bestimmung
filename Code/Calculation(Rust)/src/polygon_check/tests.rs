//--------------------------------------tests-----------------------------------------------------
#[cfg(test)]
    use super::*;

    #[test]
    fn test_all(){
        polygon_is_closed_test(); 
        all_points_are_unique_test();
        check_polygon_test();
    }
    
    #[test]
    fn polygon_is_closed_test() {
        let polygon1 = vec![(0f64, 0f64),(5f64, 0f64),(2.5f64, 3f64),(0f64, 0f64)];
        let polygon2 = vec![(0f64, 0f64),(5f64, 0f64),(2.5f64, 3f64), (2.5f64,4f64), (2.5f64, 6f64) ,(0f64, 0f64)];
        let polygon3 = vec![(1f64, 1f64), (4f64, 1f64), (4f64, 2f64), (4f64, 3f64), (5f64, 3f64),(6f64, 3f64), (6f64, 6f64), (4f64, 5f64),(4f64, 8f64), (2f64, 5f64), (-1f64, 4f64),(-3f64, 2f64), (-2f64, -2f64), (0f64, -5f64),(4f64, -3f64), (4f64, -1f64), (1f64, 1f64)];
        let polygon4 = vec![(4f64, 1f64), (4f64, 2f64), (4f64, 3f64), (5f64, 3f64),(6f64, 3f64), (6f64, 6f64), (4f64, 5f64),(4f64, 8f64), (2f64, 5f64), (-1f64, 4f64),(-3f64, 2f64), (-2f64, -2f64), (0f64, -5f64),(4f64, -3f64), (4f64, -1f64), (1f64, 1f64)];
       
        let polygon5 = vec![(0f64, 0f64), (0f64, 0f64)];

        assert_eq!(true,    polygon_is_closed(&polygon1));
        assert_eq!(true,    polygon_is_closed(&polygon2));
        assert_eq!(true,    polygon_is_closed(&polygon3));
        assert_eq!(false,   polygon_is_closed(&polygon4));
        assert_eq!(true,    polygon_is_closed(&polygon5));
   }


    #[test]
    fn all_points_are_unique_test(){
        let polygon1 = vec![(0f64, 0f64),(5f64, 0f64),(2.5f64, 3f64),(0f64, 0f64)];
        let polygon2 = vec![(0f64, 0f64),(5f64, 0f64),(2.5f64, 3f64), (2.5f64,4f64), (2.5f64, 6f64) ,(0f64, 0f64)];
        let polygon3 = vec![(1f64, 1f64), (4f64, 1f64), (4f64, 2f64), (4f64, 3f64), (5f64, 3f64),(6f64, 3f64), (6f64, 6f64), (4f64, 5f64),(4f64, 8f64), (2f64, 5f64), (-1f64, 4f64),(-3f64, 2f64), (-2f64, -2f64), (0f64, -5f64),(4f64, -3f64), (4f64, -1f64), (1f64, 1f64)];
        let polygon4 = vec![(4f64, 1f64), (4f64, 2f64), (4f64, 3f64), (5f64, 3f64),(6f64, 3f64), (6f64, 6f64), (4f64, 5f64),(4f64, 8f64), (2f64, 5f64), (-1f64, 4f64),(-3f64, 2f64), (-2f64, -2f64), (0f64, -5f64),(4f64, -3f64), (4f64, -1f64), (1f64, 1f64)];
        let polygon5 = vec![(0f64, 0f64), (0f64, 0f64)];
        let polygon6 = vec![(0f64, 0f64)];

        let polygon7 = vec![(1f64, 1f64), (4f64, 1f64), (4f64, 2f64), (4f64, 3f64), (5f64, 3f64),(6f64, 3f64), (6f64, 6f64), (4f64, 5f64),(4f64, 8f64), (2f64, 5f64), (-1f64, 4f64),(-3f64, 2f64), (4f64, 2f64),(-2f64, -2f64), (0f64, -5f64),(4f64, -3f64), (4f64, -1f64), (1f64, 1f64)];
        let polygon8 = vec![(0f64, 0f64),(5f64, 0f64),(2.5f64, 3f64), (0f64, 0f64), (2.5f64,4f64), (2.5f64, 6f64) ,(0f64, 0f64)];
       
        assert_eq!(true,    all_points_are_unique(&polygon1));
        assert_eq!(true,    all_points_are_unique(&polygon2));
        assert_eq!(true,    all_points_are_unique(&polygon3));
        assert_eq!(true,    all_points_are_unique(&polygon4));
        assert_eq!(true,    all_points_are_unique(&polygon5));
        assert_eq!(true,    all_points_are_unique(&polygon6));
        assert_eq!(false,    all_points_are_unique(&polygon7));
        assert_eq!(false,    all_points_are_unique(&polygon8));
    }
   #[test]
    fn check_polygon_test(){
        //OK
        let polygon1 = vec![(0f64, 0f64),(5f64, 0f64),(2.5f64, 3f64),(0f64, 0f64)];
        let polygon2 = vec![(0f64, 0f64),(5f64, 0f64),(2.5f64, 3f64), (2.5f64,4f64), (2.5f64, 6f64) ,(0f64, 0f64)];
        let polygon3 = vec![(1f64, 1f64), (4f64, 1f64), (4f64, 2f64), (4f64, 3f64), (5f64, 3f64),(6f64, 3f64), (6f64, 6f64), (4f64, 5f64),(4f64, 8f64), (2f64, 5f64), (-1f64, 4f64),(-3f64, 2f64), (-2f64, -2f64), (0f64, -5f64),(4f64, -3f64), (4f64, -1f64), (1f64, 1f64)];
        
        //not closed
        let polygon4 = vec![(4f64, 1f64), (4f64, 2f64), (4f64, 3f64), (5f64, 3f64),(6f64, 3f64), (6f64, 6f64), (4f64, 5f64),(4f64, 8f64), (2f64, 5f64), (-1f64, 4f64),(-3f64, 2f64), (-2f64, -2f64), (0f64, -5f64),(4f64, -3f64), (4f64, -1f64), (1f64, 1f64)];
        
        //to short
        let polygon5 = vec![(0f64, 0f64), (0f64, 0f64)];
        let polygon6 = vec![(0f64, 0f64)];


        //not unique
        let polygon7 = vec![(1f64, 1f64), (4f64, 1f64), (4f64, 2f64), (4f64, 3f64), (5f64, 3f64),(6f64, 3f64), (6f64, 6f64), (4f64, 5f64),(4f64, 8f64), (2f64, 5f64), (-1f64, 4f64),(-3f64, 2f64), (4f64, 2f64),(-2f64, -2f64), (0f64, -5f64),(4f64, -3f64), (4f64, -1f64), (1f64, 1f64)];
        let polygon8 = vec![(0f64, 0f64),(5f64, 0f64),(2.5f64, 3f64), (0f64, 0f64), (2.5f64,4f64), (2.5f64, 6f64) ,(0f64, 0f64)];


        //empty
        let polygon9 = vec![];

        //mixed
        let polygon10 = vec![(0f64, 0f64),(5f64, 0f64),(2.5f64, 3f64), (2.5f64,7f64), (2.5f64, 6f64) ,(0f64, 0f64)];
        let polygon11 = vec![(61962337.0f64, 506585001.0f64), (61846953.0f64, 506493844.0f64), (61962337.0f64, 506585001.0f64)];
        let polygon12 = vec![(61962337.0f64, 506585001.0f64), (61846953.0f64, 506493844.0f64), (61962337.0f64, 506585001.0f64), (61846953.0f64, 506493844.0f64)];
        let polygon13 = vec![(62485110.0f64,  505976658.0f64)];
        
        //true
        assert_eq!(PolygonShapeState::Ok,                   check_polygon(&polygon1));
        assert_eq!(PolygonShapeState::Ok,                   check_polygon(&polygon2));
        assert_eq!(PolygonShapeState::Ok,                   check_polygon(&polygon3));
        assert_eq!(PolygonShapeState::NotClosed,            check_polygon(&polygon4));
        assert_eq!(PolygonShapeState::LessThan4Points,      check_polygon(&polygon5));
        assert_eq!(PolygonShapeState::OnlySinglePoint,      check_polygon(&polygon6));
        assert_eq!(PolygonShapeState::PointsAreNotUnique,   check_polygon(&polygon7));
        assert_eq!(PolygonShapeState::PointsAreNotUnique,   check_polygon(&polygon8));
        assert_eq!(PolygonShapeState::EmptyStrucure,        check_polygon(&polygon9));
    
        //false
        assert_ne!(PolygonShapeState::Ok,                   check_polygon(&polygon10));
        assert_ne!(PolygonShapeState::Ok,                   check_polygon(&polygon11));
        assert_ne!(PolygonShapeState::Ok,                   check_polygon(&polygon12));
        assert_ne!(PolygonShapeState::Ok,                   check_polygon(&polygon13));
        
    }
