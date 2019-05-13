/****************************************************************************
*  ++ LUKAS BAUR, Bachelorarbeit 2018/2019 ++
*
*  This module calculates a triangulation of a polygon
*
*****************************************************************************/

extern crate poly2tri;

//-------------------------------Dokumentation----------------------------------------------------------
/*
INPUT
    <polgon_family>  = [<linearRing>, <linearRing>, <linearRing>, ..., <linearRing>]
    <linearRing>     = [<position_1>, <position_2>, <position_3>, ..., <position_n>, <position_1>] for n > 2
    <position_i>     = <position>
    <position>       = [x, y]

OUTPUT
    <triangulation>  = (<triangles>, <outer_lines>)
WHERE
    <triangles>      = [<triangle>, <triangle>, <triangle>, <triangle>, .. , <triangle>]
    <triangle>       = [[x1, y1], [x2, y2], [x3, y3]]
    <outer_lines>    = [<outer_line>, <outer_line>, <outer_line>, ..., <outer_line>, <outer_line>] 
    <outer_line>     = (x1, y1, x2, y2)
*/


//-------------------------------Import----------------------------------------------------------------
    //global
use std::fs::File;
use std::io::Write;

//------------------------------Structs----------------------------------------------------------------

pub struct Polygontriangulation {
    polygon_outer_lines : Vec<(f64,f64,f64,f64)>, 
    triangles: Vec<[[f64; 2]; 3]>
}
impl  Polygontriangulation{
    pub fn new( polygon_outer_lines : Vec<(f64,f64,f64,f64)>, triangles: Vec<[[f64; 2]; 3]>) -> Polygontriangulation{
        Polygontriangulation{
            polygon_outer_lines,
            triangles
        }
    }
    pub fn get_triangles(&self) ->  Vec<[[f64; 2]; 3]>{
        self.triangles.clone()
    }
    pub fn get_outer_lines(&self) ->  Vec<(f64,f64,f64,f64)>{
        self.polygon_outer_lines.clone()
    }
}


//------------------------------public functions----------------------------------------------------------------
//--------------Main-----------------------------
pub fn generate_polygon_triangulation(polygonfamily : &Vec<Vec<(f64,f64)>>)-> Polygontriangulation{

    if polygonfamily.len() == 0{
        return get_empty_triangulation();
    }
    let outer_polygon = &polygonfamily[0];
    
    if outer_polygon.len() < 5{
        //Eingabe muss mindestens 5 (davon 4 unterschiedliche) Punkte enthalten (darf insbesondere kein Dreick sein)
        return get_empty_triangulation();
    }
    
    //create polygon
    let mut polygon = poly2tri::Polygon::new();
    let mut outer_lines : Vec<(f64,f64,f64,f64)> = Vec::new();
    
    for i in 0..outer_polygon.len()-1 {
        let (x,y) = outer_polygon[i]; 
        let (p,q) = outer_polygon[i+1];  
        outer_lines.push((x,y,p,q));   
        polygon.add_point(x,y);
        //println!("Segments: ({},{})({},{}),",x,y,p,q );
    }
    
    //triangulate and move to struct
    let cdt = poly2tri::CDT::new(polygon);
    let triangles = cdt.triangulate();
    let triangle_vec = build_triangle_vector(triangles);
    let triangulation = Polygontriangulation::new(outer_lines, triangle_vec);    
    
    triangulation
}



//--------------Public Helper functions------------------------------
pub fn write_diagonals_to_file(triangulation : &Polygontriangulation,  file: &mut File){
    let polygon_lines : &Vec<(f64,f64,f64,f64)> = &triangulation.polygon_outer_lines;
    let triangles : &Vec<[[f64; 2]; 3]> = &triangulation.triangles;


    for j in 0..triangles.len(){
        let triangle =  triangles[j];
        for i in 0..3{
            let point_i = triangle[i];
            let point_i1 = triangle[(i+1) % 3];
            let (a,b,c,d) = (point_i[0], point_i[1], point_i1[0],  point_i1[1]);
            let testline = (a,b,c,d);
            
            if is_a_diagonal(testline, polygon_lines){
                let file_content = format!("{} {} {} {} ", a,b,c,d);
                file.write_all(file_content.as_bytes());
            }
        }
    }
    file.write_all("\n".as_bytes());
}

//---

pub fn is_a_diagonal(testline : (f64,f64,f64,f64), polygon_lines : &Vec<(f64,f64,f64,f64)>) -> bool{
    //test (a,b), (c,d)
    if polygon_lines.contains(&testline){
            return false;
    }
    
    //test (c,d), (a,b)
    let (a,b,c,d) = testline;
        //permutate
    let new_testline = (c,d,a,b);
    if polygon_lines.contains(&new_testline){
            return false;
    }
    true
}


//---------------------------private functions -----------------------------------------------------

fn build_triangle_vector(triangles :  poly2tri::TriangleVec) -> Vec<[[f64; 2]; 3]>{
    let mut triangle_vec: Vec<[[f64; 2]; 3]> = Vec::new();
    for i in 0..triangles.size() {
        let tri = triangles.get_triangle(i);
        triangle_vec.push(tri.points);
    }
    
    triangle_vec
}

//---


fn get_empty_triangulation()-> Polygontriangulation{
    let empty_polygon_outer_lines : Vec<(f64,f64,f64,f64)> = Vec::new();
    let empty_triangulation_vec : Vec<[[f64; 2]; 3]> = Vec::new();
    
    let empty_triangulation = Polygontriangulation::new(empty_polygon_outer_lines, empty_triangulation_vec); 

    empty_triangulation
}
