/*
++ LUKAS BAUR, Bachelorarbeit 2018/2019 ++




INPUT:  1) Polygons <inner_polygonList> and <outer_polygonList> with
            <inner_polygonList> =  [<polygon>, <polygon>, ..., <polygon>]
            <outer_polygonList> =  [<polygon>, <polygon>, ..., <polygon>]


            <polygon> = [<point>, <point>, ..., <point>]
            <point>     = (x_i, y_i)

        2) Point as (x1, y1)

OUTPUT: Polygon-hierarchy (2-layerd) <polygon_hierarchy>
            <polygon_hierarchy> = [<polygon-family>, <polygon-family>, ..., <polygon-family>]
            <polygon-family> = [<outer-polygon>, <inner-polygon>, <inner-polygon>, ..., <inner-polygon>]

            where <*-polygon> is from type <polygon> (see 'Input')    



Note: Polygons must fullfill (x_0,y_0) = (x_n, y_n)

*/

use crate::point_in_polyon_test;
use crate::point_in_polyon_test::PointInPolygonRelation;

#[derive(Debug)]
enum BorderType {
    Inner,
    Outer
}

#[derive(Debug)] 
struct Polygon {
    id : i32,       //id's with positive sign = outer, negative sign = inner
    border : BorderType,
    linkedPolygon : Vec<(i32,i32)>,
    ausgehende_Inner : Vec<i32>,
    ausgehende_Outer : Vec<i32>,
    eingehende_Inner : Vec<i32>,
    eingehende_Outer : Vec<i32>
}

pub fn sort_polygons_topologically (outer_polygonList: Vec<Vec<(i32,i32)>>, inner_polygonList: Vec<Vec<(i32,i32)>>) -> Vec<Vec<Vec<(i32,i32)>>>{
    let mut inners_template : Vec<Polygon>;
    let mut outers_template :  Vec<Polygon>;
    inners_template = convert_to_polygon_structs(inner_polygonList, true);
    outers_template = convert_to_polygon_structs(outer_polygonList, false);

 //  set_subset_of_relations(&mut inners_template,&mut outers_template);

    
    
    
    //implementation stopped after meeting on 11th of Feb.

    //using a method from the fmi instead. 


    let result_stack : Vec<Vec<Vec<(i32,i32)>>> =  Vec::new();
    result_stack
}

fn get_independent_outer(outer_list :& Vec<Polygon>) -> &Polygon{
    for polygon in outer_list{
        if polygon.ausgehende_Inner.len() == 0 && polygon.ausgehende_Outer.len() == 0{
            return polygon;
        }
    }
    panic!("No free, idependend outer-Polygon found.");
}



/*

//read: set "is-subset-of" relations
fn set_subset_of_relations(inners : &mut Vec<Polygon> , outers : &mut Vec<Polygon>){
    //Check all Inner->Outer and Inner-Inner relations
    
    for i in 0..inners.len() {
       for j in 0..inners.len(){
           if i == j{
               continue;
           }
           
       } 
    }


    let icounter : u32 = 0;
    for i in inners{
        let jcounter : u32 = 0;
        for j in inners.iter(){
            if i.id == j.id {
                jcounter = jcounter +1;
                continue;
            }
            if isSubsetOf(i, j){
                //objekt i = 
                let index_i = -1 * i.id
                let index_j = 

                inners[((-1)* i.id) -1 as usize].ausgehende_Inner.push(j.id);
                j.eingehende_Inner.push(i.id);
            }
            jcounter = jcounter +1;
        }
        for j in outers.iter(){
            if isSubsetOf(i, j){
                i.ausgehende_Outer.push(j.id);
                j.eingehende_Inner.push(i.id);
            }
        }
        icounter = icounter +1;
    }
    //Check all Outer->Outer and Outer-Inner relations
    for o in outers{
        for t in outers{
            if o.id == t.id{
                continue;
            }
            if isSubsetOf(o, t){
                o.ausgehende_Outer.push(t.id);
                t.eingehende_Outer.push(o.id);
            }
        }
        for t in inners {
            if isSubsetOf(o, t){
                o.ausgehende_Inner.push(t.id);
                t.eingehende_Outer.push(o.id);
            }
        }
    }
   
   
}
*/
fn isSubsetOf(a: &Polygon, b: &Polygon)-> bool{
    let testing_point : (f32, f32) = calculate_testing_point(a);
    let test_result = point_in_polyon_test::is_inside(&b.linkedPolygon, testing_point);
    let result = match test_result {
        PointInPolygonRelation::isInside => true,
        PointInPolygonRelation::isOutside => false,
        PointInPolygonRelation::onBorder => panic!("Testing Point (inside-outside-realation is on boarder.")
    };
    result
}

fn calculate_testing_point(a :& Polygon) -> (f32, f32) {
    let first_point : (i32,i32) = a.linkedPolygon[0];
    let second_point : (i32,i32) = a.linkedPolygon[1];

    let mut point : (f32, f32) = (first_point.0 as f32, first_point.1 as f32);

    let divider : f32 = 2 as f32;

    point.0 = (point.0 + (second_point.0 as f32)) / divider;
    point.1 = (point.1 + (second_point.1 as f32)) / divider;

    point
}

fn convert_to_polygon_structs(input_vec : Vec<Vec<(i32,i32)>>, inner:bool) -> Vec<Polygon>{
    let mut result : Vec<Polygon> = Vec::new();
    let mut i : i32 = 1;
    let sign : i32 = match inner {
        true => -1,
        _ => 1,
    };
    for poly_list in input_vec {
        
        let border_val : BorderType = match inner {
            true => BorderType::Inner,
            _    => BorderType::Outer,
        };

        result.push(Polygon{
            id : i * sign,
            border : border_val,
            linkedPolygon : poly_list,
            ausgehende_Inner : Vec::new(),
            ausgehende_Outer : Vec::new(),
            eingehende_Inner : Vec::new(),
            eingehende_Outer : Vec::new()
        });
        i = i+1;
    }
    result
}
 