//Dokumentation

/*

~ ~ ~ ~ for the whole module the following holds: ~ ~ ~ ~


INPUT: <polygonfamily> = [<outer_polygon>, <inner_polygon>, <inner_polygon>, ..., <inner_polygon>]
        
        WHERE
        <outer_polygon> = <polygon>       
        <inner_polygon> = <polygon> 
        <polygon> = [(x,y), (x,y), ..., (x,y)]
        x,y = f64

OUTPUT: <center_point> = (x, y)
        x,y = f64
*/


//---------------------------------public API-function---------------------------------------------------
pub fn get_geometrical_center(polygonfamily : &Vec<Vec<(f64,f64)>>) -> (f64, f64){
    assert!(polygonfamily.len() > 0);
    let outer_polygon = polygonfamily.get(0).unwrap();
    let (outer_x,outer_y,outer_area) = calculate_geometrical_polygon_center(outer_polygon);
    
    let initial_vector : (f64, f64,f64) = (0 as f64, 0 as f64, 0 as f64);
    let (mut inner_x_sum, mut inner_y_sum, mut inner_area_sum) = initial_vector;
    
    for i in 1..polygonfamily.len(){
        let current_inner_polygon = polygonfamily.get(i).unwrap(); 
        let (current_inner_x, current_inner_y, current_inner_area) = calculate_geometrical_polygon_center(current_inner_polygon);
        inner_x_sum = inner_x_sum + current_inner_x * current_inner_area;
        inner_y_sum = inner_y_sum + current_inner_y * current_inner_area;
        inner_area_sum = inner_area_sum + current_inner_area;
    }
    let x = outer_x * outer_area - inner_x_sum;
    let y = outer_y * outer_area - inner_y_sum;
    let area_without_holes = outer_area - inner_area_sum;

    let (x_c, y_c) = (x / area_without_holes, y / area_without_holes);
    
     (x_c, y_c)
}


pub fn get_centroid(polygonfamily : &Vec<Vec<(f64,f64)>>) -> (f64, f64){
    let mut counter = 0;
    let mut x_count = 0 as f64;
    let mut y_count = 0 as f64;
    for polygon in polygonfamily{
        for i in 0..polygon.len()-1{    //ignore last, because last=first
            counter = counter +1;
            x_count = x_count + polygon.get(i).unwrap().0;
            y_count = y_count + polygon.get(i).unwrap().1;
        }
    }
    let (x,y) = (x_count / (counter as f64), y_count / (counter as f64));

    (x,y)
}

/*  + + + + + +                + + + + + + 
   +           + + + + + + + + +          + 
   +                                         + 
    +          o o o o                          +
     +       o         o        o o                +
      +     o           o     o      o                +
      +    o            o      o      o               +
      +     o           o        o o o                 +
        +     o  o  o  o                               +
         +                                            +  
          + + + + + + + + +                          +       
                           + + + + + + + + + + + + + 
 */
pub fn get_polygonfamily_area(polygonfamily : &Vec<Vec<(f64,f64)>>) -> f64{
    if polygonfamily.len() == 0{
        return 0f64;
    }
    let outer_polygon = &polygonfamily[0];
    let outer_area = calculate_polygon_area(outer_polygon);
    let mut total_inner_area = 0f64;
    for i in 1..polygonfamily.len(){
        let inner_polygon = &polygonfamily[i];
        let inner_area = calculate_polygon_area(inner_polygon);
        total_inner_area = total_inner_area + inner_area;
    }
    let difference = outer_area - total_inner_area;
    assert!(difference >= 0f64);

    difference
}

//---------------------------------private functions----------------------------------------------------
fn calculate_geometrical_polygon_center(polygon : &Vec<(f64,f64)>) -> (f64, f64, f64){
    let area = calculate_polygon_area(polygon);   
    if area == (0 as f64) || polygon.len() < 4{
        return (0 as f64, 0 as f64, 0 as f64)
    }
    let different_points = polygon.len()-1;
    let (x_0, y_0) = polygon.get(0).unwrap();
    let (x_1, y_1) = polygon.get(1).unwrap();

    let a0 = x_0 * y_1 - x_1*y_0; 
    let mut s_x = (x_0 + x_1) * a0;
    let mut s_y = (y_0 + y_1) * a0;

    for i in 1..different_points{
        let (x_i, y_i) = polygon.get(i).unwrap();
        let (x_i_1, y_i_1) = polygon.get(i+1).unwrap();
        let a_i = x_i*y_i_1 - x_i_1*y_i;
        s_x = s_x + (x_i+x_i_1)*a_i;
        s_y = s_y + (y_i+y_i_1)*a_i;
    }
    s_x = s_x / (6 as f64 * area);
    s_y = s_y / (6 as f64 * area);
    
    (s_x, s_y, area)
}

fn calculate_polygon_area(polygon: &Vec<(f64,f64)>) -> f64{
    //the smallest polygon with an area > 0 is the triangle [p0, p1, p2, p0]
    if polygon.len() < 4{
        return 0 as f64;
    }
    
    let different_points = polygon.len()-1;

    let mut a = 0 as f64; 
    for i in 0..different_points{
        let (x_i, y_i) = polygon.get(i).unwrap();
        let (x_i_1, y_i_1) = polygon.get(i+1).unwrap();
        let a_i = x_i * y_i_1 - x_i_1 * y_i;

        a = a + a_i;
    }
    let area = a / (2 as f64);
    
    area.abs()
}



#[test]
fn get_polygonfamily_area_test() {
    let polygon1 = vec![(0f64, 0f64),(10f64, 0f64),(10f64, 10f64),(0f64, 10f64),(0f64, 0f64)];
    let polygon2 = vec![(1f64, 1f64),(3f64, 1f64),(3f64, 3f64),(1f64, 3f64),(1f64, 1f64)];
    let polygon3 = vec![(5f64, 5f64),(6f64, 5f64),(6f64, 6f64),(5f64, 6f64),(5f64, 5f64)];

    let poly_fam = vec![polygon1.clone(), polygon2.clone(), polygon3.clone()];

    assert_eq!(100f64, calculate_polygon_area(&polygon1));
    assert_eq!(4f64, calculate_polygon_area(&polygon2));
    assert_eq!(1f64, calculate_polygon_area(&polygon3));

    assert_eq!(95f64, get_polygonfamily_area(&poly_fam)); 
} 