//-------------------------------Dokumentation----------------------------------------------------------
/*
INPUT
    <polgon>        = [<outerRing>, <innerRing>, <innerRing>, ..., <innerRing>]
    <outerRing>     = <linearRing>
    <innerRing>     = <linearRing>
    <linearRing>    = [<position_1>, <position_2>, <position_3>, ..., <position_n>, <position_1>] for n > 2
    <position_i>    = <position>
    <position>      = (x, y)
OUTPUT
    <position>      = (x, y, radius)
WHERE
    (x, y) = (f64, f64),
    (x, y, radius) = (f64, f64, f64),
    [<*>, .. <*>] = Vec<*>
*/


//-------------------------------Import----------------------------------------------------------------
use std::f64::INFINITY;
use std::f64::consts;
use std::collections::binary_heap::BinaryHeap;
use std::cmp::{Ordering, PartialOrd, Ord, Eq};


//-------------------------------Constants--------------------------------------------------------------
const SQRT2 : f64 = consts::SQRT_2 as f64;


//------------------------------Structs----------------------------------------------------------------
struct Cell{
    h : f64,    // half cell size
    y : f64,    //cell center y
    x : f64,    //cell center x
    d : f64,    //distance from cell center to next polygon
    max : f64   //maximal distance to a polygon within a cell
}
impl Cell{
    fn new(x: f64, y:f64,h:f64, polygon : &Vec<Vec<(f64,f64)>>) -> Cell{
        let d : f64 = point_to_polygon_dist(x,y, polygon);
        Cell{
            h : h,
            x : x,
            y : y,
            d : d,
            max : d + h *  (SQRT2 as f64) 
        }
    }
}
impl Ord for Cell{
    fn cmp(&self, other: &Cell) -> Ordering {
        if self.max > other.max{
            return Ordering::Greater;
        }    
        if self.max < other.max{
            return Ordering::Less;
        }    
        Ordering::Equal    
    }
}
impl Eq for Cell {
}
impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Cell) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Cell {
    fn eq(&self, other: &Cell) -> bool {
        self.max == other.max
    }
}
impl Clone for Cell{
    fn clone(&self) -> Cell{
        Cell{
            h : self.h,    
            y : self.y,    
            x : self.x,   
            d : self.d,   
            max : self.max}
        }
}


//---------------------------------public API-function---------------------------------------------------
pub fn get_point_of_inaccessibility(polygon : &Vec<Vec<(f64,f64)>>, precision : f64) -> (f64, f64, f64){
    let (min_x, min_y, max_x, max_y) = calculate_outer_bounding_box(polygon);
    let (width, height, cell_size, h) = calculte_gridsize(min_x, min_y, max_x, max_y);
    
    //early reject
    if cell_size == 0 as f64{
        return (min_x, min_y, 0 as f64);
    }

    //create Prio-Queue
    let mut priority_queue : BinaryHeap<Cell> = BinaryHeap::new();

    //fill initially
    let mut x :f64  = min_x;
    let mut y :f64  = min_y;

    while  x < max_x{
        while  y < max_y{
            priority_queue.push(Cell::new(x + h, y+h, h, polygon));
            y = y + cell_size;
        }    
        x = x + cell_size;
        y = min_y;
    }

    //center of bounding box
    let bbox_center_cell = Cell::new(min_x + width/(2 as f64), min_y + height/(2 as f64), 0 as f64, polygon);
   
    //center of outer polygon
    let outer_center_cell = get_centroid_cell(polygon);

    //initialize first guess with better result
    let mut best_cell = match outer_center_cell.d > bbox_center_cell.d {
        true  => outer_center_cell,
        false => bbox_center_cell,
    };

    //iterate until no cell is left
    while priority_queue.len() > 0 {
        //take best cell so far
        let current_cell = priority_queue.pop().unwrap();
        
        //update iff current cell better
        if current_cell.d > best_cell.d{
            best_cell = current_cell.clone();
        }

        if current_cell.max - best_cell.d > precision{
            //split cell
            let new_h = current_cell.h / 2 as f64;
            priority_queue.push(Cell::new(current_cell.x - new_h, current_cell.y - new_h,  new_h, polygon));
            priority_queue.push(Cell::new(current_cell.x - new_h, current_cell.y + new_h,  new_h, polygon));
            priority_queue.push(Cell::new(current_cell.x + new_h, current_cell.y - new_h,  new_h, polygon));
            priority_queue.push(Cell::new(current_cell.x + new_h, current_cell.y + new_h,  new_h, polygon));
        }
    };
       
    (best_cell.x as f64,best_cell.y as f64, best_cell.d)
}


pub fn point_to_polygon_dist(x : f64,y: f64, polygon : &Vec<Vec<(f64,f64)>>) -> f64{

    let mut point_is_inside :bool = false;
    let mut min_dist_sq : f64 = INFINITY;
    for k in 0..polygon.len(){
        let ring : &Vec<(f64,f64)> = &polygon[k];
        
        let len = ring.len();
        let  mut j = len - 1;
        for i in 0..len{
            let a : (f64,f64) = ring[i];
            let b : (f64,f64) = ring[j];

            let a_greater_y = a.1 > y;
            let b_greater_y = b.1 > y;
            let point_is_inbetween_Y_values = (a_greater_y && !b_greater_y) || (!a_greater_y && b_greater_y);
            
            if point_is_inbetween_Y_values{
                let x_val = (b.0 - a.0) * (y - a.1) / (b.1 - a.1) + a.0;
                if x < x_val{
                    //Point is left from border
                    point_is_inside = !point_is_inside;
                }
            }
            let current_sqrt_distance : f64 = get_segment_distance_sq(x,y,a,b);
            min_dist_sq = match min_dist_sq < current_sqrt_distance{
                true => min_dist_sq,
                false => current_sqrt_distance
            };

            j = i;   
        }
    }

    let abs_distance = min_dist_sq.sqrt();
    match point_is_inside{
        true => abs_distance,
        false => - abs_distance,
    }
}
//---------------------------------private functions----------------------------------------------------
fn calculate_outer_bounding_box(polygon : &Vec<Vec<(f64,f64)>>)->(f64, f64, f64, f64){
    let outer :  &Vec<(f64,f64)> = &polygon[0];
    calculate_bounding_box(outer)
}


fn calculte_gridsize(min_x : f64, min_y : f64, max_x : f64, max_y : f64)->(f64, f64, f64, f64){
    let width :f64 = max_x - min_x;
    let height :f64 = max_y - min_y;
    let cell_size = match width > height{
        true => height,
        false => width
    };
    let h : f64 = cell_size / 2 as f64;

    (width, height, cell_size, h)
}


fn calculate_bounding_box(polygon : &Vec<(f64,f64)>)-> (f64, f64, f64, f64){
    let initial_point =  polygon[0];
    let mut min_x : f64 = initial_point.0;
    let mut min_y : f64 = initial_point.1;
    let mut max_x : f64 = initial_point.0;
    let mut max_y : f64 = initial_point.1;


    for i in 1..polygon.len() {
        let current_point : (f64,f64) = polygon[i];

        if current_point.0 < min_x {
            min_x = current_point.0;
        } 
        if current_point.1 < min_y {
            min_y = current_point.1;
        } 
        if current_point.1 > max_y {
            max_y = current_point.1;
        } 
        if current_point.0 > max_x {
            max_x = current_point.0;
        } 
    }
    (min_x, min_y, max_x, max_y)
}




//calculate squared distance from a point (px, py) to a polygon-segment (a.0, a.1)-----(b.0, b.1)
fn get_segment_distance_sq(px :f64, py:f64, a:(f64,f64), b:(f64,f64)) -> f64{
    let mut x = a.0;
    let mut y = a.1;
    let mut dx = b.0 - x;
    let mut dy = b.1  -y;
    
    if dx != (0 as f64) || dy != (0 as f64){
        let t = ((px - x) * dx + (py - y) * dy) / (dx * dx + dy * dy);
         
        if t > 1 as f64 {
            x = b.0;
            y = b.1;
        } else if t > 0 as f64 {
            x =  x + dx * t;
            y =  y + dy * t;
        }
    } 
    dx = px - x;
    dy = py - y;
    let result :f64 = dx * dx + dy * dy;

    result
}


fn get_centroid_cell(polygon : &Vec<Vec<(f64,f64)>>) -> Cell{
    let mut area :f64 = 0 as f64;
    let mut x :f64 = 0 as f64;
    let mut y :f64 = 0 as f64;
    let outer_points : &Vec<(f64,f64)> = &polygon[0] ; //get center of outer polynom

    let len = outer_points.len();
    let mut j = len - 1;

    for i in 0..len{
        let a = outer_points[i];
        let b = outer_points[j];
        let f = a.0 * b.1 - b.0 * a.1;
        x = x + (a.0 + b.0) * f;
        y = y + (a.1 + b.1) * f;
        j = i;
        area = area + f * (3 as f64);
    }
    let first_point : (f64,f64) = outer_points[0];

    let result : Cell = match area == 0 as f64 {
        true => Cell::new(first_point.0, first_point.1, 0 as f64, polygon),
        false => Cell::new(x/area, y/area, 0 as f64, polygon),
    };
    
    result
}
