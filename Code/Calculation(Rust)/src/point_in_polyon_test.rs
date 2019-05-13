/*


++ LUKAS BAUR, Bachelorarbeit 2018/2019 ++



INPUT:  1) Polygon as [<Pointlist>] with
            <Pointlist> = <Point>, <Point>, ..., <Point>
            <Point>     = (x_i, y_i)

        2) Point as (x1, y1)

OUTPUT: PointInPolygonRelation-enum entry with
            isInside:   Point is inside the polygon
            isOutside:  Point is outside the polygon
            onBorder:   Point is on a border edge of the polygon

Note: Polygons must fullfill (x_0,y_0) = (x_n, y_n)



*/

#[derive(Debug)]
pub enum PointInPolygonRelation {
    isInside,
    isOutside,
    onBorder,
}

pub fn is_inside(polygon : &Vec<(i32,i32)>, point :(f32,f32)) -> PointInPolygonRelation{
    let mut inside_outside_match : i8 = -1;
    let n = polygon.len();

    for i in 0..n-1{
        let current_vertex = polygon[i];
        let next_vertex = polygon[i+1];
        
        inside_outside_match = inside_outside_match * check_edge(current_vertex, next_vertex, point);
        if inside_outside_match == 0{
            return PointInPolygonRelation::onBorder;
        }
    }
    if inside_outside_match < 0{
        return PointInPolygonRelation::isOutside;
    }
    if inside_outside_match > 0{
        return PointInPolygonRelation::isInside;
    }
    PointInPolygonRelation::onBorder 
}

fn check_edge(input_current :(i32,i32), input_next:(i32,i32), point:(f32,f32)) -> i8{
    let current_x = input_current.0 as f32;
    let current_y = input_current.1 as f32;
    let current_vertex : (f32, f32) = (current_x, current_y);

    let next_x = input_next.0 as f32;
    let next_y = input_next.1 as f32;
    let next_vertex : (f32, f32) = (next_x, next_y);


    let point_x = point.0;
    let point_y = point.1;


    //are the parallel to y-line?
    if current_y == next_y && current_y  == point_y{
        if (current_x <= point_x && point_x <= next_x) || (next_x <= point_x && point_x <= current_x){
            return 0; // Point is on the current-next-line
        }    
        else { 
            return 1; //outside
        }
    } 
    //are the points equal?
    if point == current_vertex{
        return 0; //on boarder
    }
    let (lower_x, lower_y)  = match current_y > next_y {
        true => next_vertex,
        false => current_vertex,
    };
    let (higher_x, higher_y) = match current_y > next_y {
        true => current_vertex,
        false => next_vertex,
    };
    if point_y <= lower_y || point_y > higher_y{
        return 1; //outside
    }
    //calulate triangle-area sign
    let double_area = (lower_x-point_x)*(higher_y-point_y) - (lower_y-point_y)*(higher_x-point_x);
    if double_area > 0 as f32{
        return -1; //Punkt liegt links von Linie
    }
    else if double_area < 0 as f32{
        return 1; //Punkt liegt rechts von Linie, Dreiecksfläche ist negativ
    }
    //ansonsen: Fläche ist 0 -> Abstand zu Linie ist 0 -> Punkt auf Linie
    0 
}