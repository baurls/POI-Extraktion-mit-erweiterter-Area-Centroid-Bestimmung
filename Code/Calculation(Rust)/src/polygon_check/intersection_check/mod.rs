/****************************************************************************
*  ++ LUKAS BAUR, Bachelorarbeit 2018/2019 ++
*
*  This module checks the poylgon on intersections. 
*
*****************************************************************************/

//------------------------------Structs----------------------------------------------------------------
#[derive(Debug,PartialEq)]
pub enum IntersectionType {
    T_Intersection,     //T-Kante: Ein Endpunkt liegt inmitten einer anderen Kante
    InALine,            //linien sind parallel und liegen Teileweise übereinander
    InAEndpoint,        //Beim Vergleich von AB und BC
    InAPointInbetween   //"klassischer" Schnitt: Linien kreuzen sich in der Mitte
}



#[derive(Debug,PartialEq)]
pub enum IntersectionResult{
    HasNoIntersection,
    HasIntersection(IntersectionType)
}

//-------------------------------Import----------------------------------------------------------------
mod line_line_tests;
mod polygon_intersection_tests;


//------------------------------Publics----------------------------------------------------------------
//
//     o-----------o                              
//     .            .                                                  
//     .             .                   o                              
//     .              .              .   .                              
//     .               .         .       .                              
//     .                .    .           .                              
//     .                 x               .                              
//     .             .    .              .                             
//     .          .         o------------o                                             
//     o------o                                                         
//    
//[tested]                                                                   
pub fn polygon_is_self_intersecting(polygon_ring : &Vec<(f64,f64)>) -> IntersectionResult{
    //preconditions:
    //  - at leat a triangle
    //  - every point is usesd only once (except: first)
    //      -> implies: every line has a length > 0
    assert!(polygon_ring.len() > 3); 


    for i in 0..polygon_ring.len()-1{
        let current_line = (polygon_ring[i], polygon_ring[(i+1) % polygon_ring.len()]);
        for j in i+1..polygon_ring.len()-1{
            let test_line = (polygon_ring[j], polygon_ring[(j+1) % polygon_ring.len()]);
            
            //perform test
            let intersect_check = are_intersecting(current_line, test_line);
            
            //evaulate Result
            match intersect_check{
                IntersectionResult::HasNoIntersection => continue,
                IntersectionResult::HasIntersection(intersection_type) => {
                    //Intersection is not "real", if it's an intersection in B:     A----B-----C
                    match intersection_type{
                        IntersectionType::InAEndpoint => {
                            if (j == i+1) || ((j+2) % polygon_ring.len()) == i{
                                continue;
                            }
                            return IntersectionResult::HasIntersection(IntersectionType::InAEndpoint);
                        },
                        _ => return IntersectionResult::HasIntersection(intersection_type)
                    }
                }
            };
        }
    }

    IntersectionResult::HasNoIntersection
}

//------------------------------Top-Level-Private----------------------------------------------------------------
//      o  
//        .                               o                                       
//          .                       .                                             
//            .               .                                                  
//              .       .                                                       
//                x                                                          
//          o       .                                                         
//                    o          
//[tested]                                            
fn are_intersecting(current_line: ((f64, f64), (f64, f64)), testline: ((f64, f64), (f64, f64)) ) -> IntersectionResult{ 
    if endpoints_intersect(current_line, testline){
        return IntersectionResult::HasIntersection(IntersectionType::InAEndpoint);
    }
    let lines_are_parallel = are_parallel(current_line, testline);
    if lines_are_parallel {
        if parallel_segments_intersect(current_line, testline){
            return  IntersectionResult::HasIntersection(IntersectionType::InALine);
        }
        return IntersectionResult::HasNoIntersection;
    }
    
    assert!(lines_are_parallel == false);
    
    non_parallel_intersect(current_line, testline) 
}



//------------------------------Second-Top-Level-Private----------------------------------------------------------------


//      o  
//        .                               o                                       
//          .                       .                                             
//            .               .                                                  
//              .       .                                                       
//                o               
//[tested]                                           
fn endpoints_intersect(current_line: ((f64, f64), (f64, f64)), test_line: ((f64, f64), (f64, f64))) -> bool{
    let current_endpoint1 =  current_line.0;
    let current_endpoint2 =  current_line.1;

    let other_endpoint1 = test_line.0;
    let other_endpoint2 = test_line.1;

    if current_endpoint1 == other_endpoint1 || current_endpoint1 == other_endpoint2 || current_endpoint2 == other_endpoint1 || current_endpoint2 == other_endpoint2{
        return true;
    }

    false
}



//a
// .
//  .      p
//   .      .
//    .      .
//     .      .
//      .      .
//       b      .
//               q      
//[tested]
fn are_parallel(current_line: ((f64, f64), (f64, f64)), testline: ((f64, f64), (f64, f64)) ) -> bool{
    //parallel = they have the same gradient
    let a = current_line.0;
    let b = current_line.1; 

    let p = testline.0;
    let q = testline.1; 

    let current_dx = b.0 - a.0;
    let current_dy = b.1 - a.1;

    let other_dx = q.0 - p.0;
    let other_dy = q.1 - p.1;

    let outer_product = current_dy * other_dx - current_dx*other_dy;
    if outer_product == 0f64{
        return true
    }

    false
}


//[tested]
fn non_parallel_intersect(current_line: ((f64, f64), (f64, f64)), testline: ((f64, f64), (f64, f64)) ) -> IntersectionResult{
    let x1 = (current_line.0).0;
    let y1 = (current_line.0).1;
    let x2 = (current_line.1).0;
    let y2 = (current_line.1).1;
    let x3 = (testline.0).0;
    let y3 = (testline.0).1;
    let x4 = (testline.1).0;
    let y4 = (testline.1).1;
    
   
    let alpha_num    = (x1 - x3)*(y3 - y4) - (y1 - y3)*(x3 - x4);
    let alpha_de_num = (x1 - x2)*(y3 - y4) - (y1 - y2)*(x3 - x4);

    let beta_num    = -( (x1-x2)*(y1-y3) - (y1-y2)*(x1-x3) );
    let beta_de_num =    (x1-x2)*(y3-y4) - (y1-y2)*(x3-x4)   ;

    assert!(alpha_de_num != 0f64);
    assert!(beta_de_num != 0f64);

    //Check for T-Edge:     T Edge   iff.  a = 0 or a = 1 (ß analoge)
    let alpha_is_0_or_1 = (alpha_num == 0f64 || alpha_num == alpha_de_num);
    let beta_is_0_or_1 = (beta_num == 0f64 || beta_num == beta_de_num ); 
    
    let alpha_larger0 = is_larger_than_0(alpha_num, alpha_de_num);
    let alpha_smaller1 = is_smaller_than_1(alpha_num, alpha_de_num);
    
    let beta_larger0 = is_larger_than_0(beta_num, beta_de_num);
    let beta_smaller1 = is_smaller_than_1(beta_num, beta_de_num);

    let beta_echt_zwischen_0_und_1 = beta_larger0 && beta_smaller1;
    let alpha_echt_zwischen_0_und_1 = alpha_larger0 && alpha_smaller1;


    //+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
    //1.)Check T Edge:
    // 1.1) 0 < a < 1 && (ß = 1 || ß = 0)
    // 1.2) 0 < ß < 1 && (a = 1 || a = 0)
    // 
    //or
    //
    //2.) Check "real" intersection (Point between both segment-endpoints)
    //      0 < a < 1 && 0 < ß < 1
    //+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++



    //1.1
    let t_edge_on_alpha = alpha_echt_zwischen_0_und_1 && beta_is_0_or_1;
    //2.2
    let t_edge_on_beta  = beta_echt_zwischen_0_und_1 && alpha_is_0_or_1;

    //1.)
    if (t_edge_on_alpha || t_edge_on_beta){
        return IntersectionResult::HasIntersection(IntersectionType::T_Intersection);
    }

    //2.
    if alpha_echt_zwischen_0_und_1 && beta_echt_zwischen_0_und_1{
        return IntersectionResult::HasIntersection(IntersectionType::InAPointInbetween);
    }

    IntersectionResult::HasNoIntersection
}


//|----------Case 1.3-------------------|   |----------Case 1.2---------------|    |-------------------------------Case 1.1-----------------------------|
//
// yes           yes            no                no              yes                        yes                    yes                       no
//                                                                                                                                                         
// A             A             A                  A                A                  A - - p - - q - - B       A - - p - - B - - q       A - - B   p - - q
//  .             .             .                 .                .   
//   p             p             B                .                p
//    .             .                             B                .
//     .             .                                             .
//      q             B             p             p                B
//       .             .             .            .                .
//        B             q             q           q                q           
//                  

//[tested]
//precondition: lines are paralell
fn parallel_segments_intersect(current_line: ((f64, f64), (f64, f64)), testline: ((f64, f64), (f64, f64)) ) -> bool{
   
    //now :    A     or  A   or   A                          
    //        .          .         .               
    //       B           B           B        
    let mut A = get_top(current_line.0, current_line.1);
    let mut B = get_bottom(current_line.0, current_line.1);

    //now :    p     or  p   or   p                          
    //        .          .         .               
    //       q           q           q     
    let mut p = get_top(testline.0, testline.1);
    let mut q = get_bottom(testline.0, testline.1);

    //Case 1.1: deviation = 0
    if A.1 == B.1{
        if A.1 != p.1{
            return false;
        }
        //y-values are the same -> A- - - - - B -> due to parallelism: p - - - - - q

        // A ------ B
        A = get_left(current_line.0, current_line.1);
        B = get_right(current_line.0, current_line.1);

        // p ------ q
        p = get_left(testline.0, testline.1);
        q = get_right(testline.0, testline.1);
    
     
        //check:  A-B p-q   no
        //        A p B q   yes
        //        A p q B   yes
        //        p A-B q   yes
        //        p A q B   yes
        //        p q A-B   no
        if (B.0 < p.0) || (q.0 < A.0){
            return false;
        }
        else {
            return true;
        }
    }

    let upper = get_upper_path((A, B), (p,q));
    let lower = get_lower_path((A, B), (p,q));

    A = upper.0;
    B = upper.1;

    p = lower.0;
    q = lower.1;

    //Case 1.2: deviation = inf. 
    if A.0 == B.0{
        if A.0 != p.0{
            return false;
        }
        //only suitable solutions:
        //A     or      p         
        //B             q
        //p             A    
        //q             B
        if (B.1 > p.1) || (p.1 > A.1){
            return false;
        }
        else{
            return true;
        }
    }

    //Case 1.3 both deviaton, but not "|" or "__"


    let ax = A.0;
    let ay = A.1;
    let bx = B.0;
    let by = B.1;
    let px = p.0;
    let py = p.1;

    let rx = bx - ax;
    let ry = by - ay;


    assert!(rx != 0f64);
    assert!(ry != 0f64);

    //Equation: A + alpha*(B-A) = P
    //      <=> A + alpha*(r)   = P

    //      <=> ax + alpha*rx   = px                und     ay + alpha*ry   = py
    //      <=> alpha*rx        = (px - ax)         und     alpha*ry        = (py - ay)
    //      <=> alpha           = (px - ax)/rx      und     alpha           = (py - ay)/ry

    let alpha1 = (px - ax) / rx;
    let alpha2 = (py - ay) / ry;



    if alpha1 != alpha2{
        return false;
    }
    assert!(alpha1 == alpha2); 
    assert!(alpha1 > 0f64);    //cannot be negativem because otherwise is p above A -> contradiction to upper test.



    // A                             
    //  .                            
    //   .                           
    //    B                          
    //      p                        
    //       .                       
    //        . 
    //         q                      
    if alpha1 > 1f64{
        return false;
    }       

    assert!(0f64 < alpha1 && alpha1 < 1f64);
    true

}


//--------------------------------------private funcitons--------------------------------------------------


//              A         or      B
//                                      A
//     B
// -> Returns A
//[tested]
fn get_right(a: (f64, f64), b: (f64, f64) ) -> (f64, f64){
    if a.0 < b.0{
        return b
    }
    
    a
}

//              B         or      A
//                                      B
//     A
// -> Returns B
//[tested]
fn get_left(a: (f64, f64), b: (f64, f64) ) -> (f64, f64){
    if a.0 < b.0{
        return a
    }
    
    b
}

//              A         or      A
//                                      B
//     B
// -> Returns A
//[tested]
fn get_top(a: (f64, f64), b: (f64, f64) ) -> (f64, f64){
    if b.1 < a.1{
        return a
    }
    
    b
}

//              A         or      A
//                                      B
//     B
// -> Returns A
//[tested]
fn get_bottom(a: (f64, f64), b: (f64, f64) ) -> (f64, f64){
    if b.1 < a.1{
        return b
    }
    
    a
}


//[tested]
fn get_upper_path((A, B): ((f64, f64), (f64, f64)), (p,q): ((f64, f64), (f64, f64)) )->  ((f64, f64), (f64, f64)){
    if A.1 > p.1{
        return (A,B);
    }
    return (p,q);
}

//[tested]
fn get_lower_path((A, B): ((f64, f64), (f64, f64)), (p,q): ((f64, f64), (f64, f64)) ) ->  ((f64, f64), (f64, f64)){
    if A.1 > p.1{
        return (p,q);
    }
    return (A,B);
}


// z.Z.:   a 
//        ---- > 0
//         b
fn is_larger_than_0(a:f64, b:f64) -> bool{
    if a == 0f64 {
        return false;
    }
    if (a > 0f64 && b > 0f64) || (a < 0f64 && b < 0f64){
        return true;
    }

    false
}

// z.Z.:   a 
//        ---- < 1
//         b
fn is_smaller_than_1(a:f64, b:f64) -> bool{
    if b < 0f64{
        return a > b;
    }
    return a < b;
}


