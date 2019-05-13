#[cfg(test)]
    use super::*;

#[test]
fn get_polygons_from_segments_test1() {
    let mut linesegments : Vec<(usize, usize,  usize, usize)> = Vec::new();
    linesegments.push( (0,0,0,5) );
    linesegments.push( (0,5,5,5) );
    linesegments.push( (5,5,5,0) );
    linesegments.push( (5,0,0,0) );

    get_polygons_from_segments(linesegments);

} 

//Polygon 001
  #[test]
fn get_polygons_from_segments_test2() {
    let mut linesegments : Vec<(isize, isize,  isize, isize)> = Vec::new();

    println!("----------Test beginnt----------");
 
    linesegments.push((1, 5, 3, 7));
    linesegments.push((1, 1, 7, 7));
    linesegments.push((7, 7, 8, -3));
    linesegments.push((8, -3, 3, -3));
    linesegments.push((3, -3, 1, 1));
    linesegments.push((3, 1, 6, 4));
    linesegments.push((6, 4, 6, 1));
    linesegments.push((6, 1, 3, 1));
    linesegments.push((4, 2, 5, 3));
    linesegments.push((5, 3, 5, 2));
    linesegments.push((5, 2, 4, 2));
    linesegments.push((9, 1, 9, 8));
    linesegments.push((9, 8, 14, 7));
    linesegments.push((14, 7, 9, 1));
    linesegments.push((10, 7, 11, 7));
    linesegments.push((11, 7, 11, 5));
    linesegments.push((11, 5, 10, 4));
    linesegments.push((10, 4, 10, 7));
    linesegments.push((9, 1, 14, 7));
    linesegments.push((14, 7, 15, 2));
    linesegments.push((15, 2, 9, 1));
    linesegments.push((11, 2, 13, 3));
    linesegments.push((13, 3, 14, 2));
    linesegments.push((14, 2, 11, 2));
    linesegments.push((13, 3, 14, 6));
    linesegments.push((14, 6, 14, 2));
    linesegments.push((14, 2, 13, 3));

    get_polygons_from_segments(linesegments);

}   

//Polygon 002
  #[test]
fn get_polygons_from_segments_test3() {
    let mut linesegments : Vec<(isize, isize,  isize, isize)> = Vec::new();

    println!("----------Test beginnt----------");
 
    linesegments.push((1,1,12,2));
    linesegments.push((12,2,14,5));
    linesegments.push((14,5,2,8));
    linesegments.push((2,8,1,1));

    linesegments.push((14,1,15,1));
    linesegments.push((15,1,16,2));
    linesegments.push((16,2,15,2));
    linesegments.push((15,2,14,1));

    linesegments.push((3,2,6,2));
    linesegments.push((6,2,6,5));
    linesegments.push((6,5,3,7));
    linesegments.push((3,7,3,2));

    linesegments.push((7,6,7,3));
    linesegments.push((7,3,10,3));
    linesegments.push((10,3,7,6));

    linesegments.push((11,3,12,4));
    linesegments.push((12,4,11,5));
    linesegments.push((11,5,10,5));
    linesegments.push((10,5,11,3));

    linesegments.push((4,5,5,4));
    linesegments.push((5,4,5,3));
    linesegments.push((5,3,4,3));
    linesegments.push((4,3,4,5));
    

    get_polygons_from_segments(linesegments);

}   


