/****************************************************************************
*  ++ LUKAS BAUR, Bachelorarbeit 2018/2019 ++
*
*  This module writes polygons to file.  
*
*****************************************************************************/

/*
INPUT: [<polygonfamily>, <polygonfamily>, ..., <polygonfamily>]
        
        WHERE
        <polygonfamily> = [<outer_polygon>, <inner_polygon>, <inner_polygon>, ..., <inner_polygon>]
        <outer_polygon> = <polygon>       
        <inner_polygon> = <polygon> 
        <polygon> = [(x,y), (x,y), ..., (x,y)]
        x,y = T
OUTPUT: <polgonfamily>\n
        <polgonfamily>\n
        ...
        <polgonfamily>
        
        
        WHERE
        <polgonfamily> =    <outer_polygom_lonlat_list>\n
                            k\n
                            <inner_polygom_lonlat_list_1>\n
                            <inner_polygom_lonlat_istt_2>\n
                            ...
                            <inner_polygom_lonlat_list_k>\n
                            
        <outer_polygom_lonlat_list>     = <polygom_lonlat_list>
        <inner_polygom_lonlat_list_i>   = <polygom_lonlat_list>
        <polygom_lonlat_list> = <point_1> <point_2> ... <point_n> <point_1>
        <point_i> = lon lat
*/

use std::fmt::Display;
use std::fs::File;
use std::io::Write;

use crate::create_empty_file;


//------------------------Public API functions------------------------------------
pub fn write_to_fmi_lon_lat_format_file<T>(path : &str, filename : &str, ending: &str, polygonfamily_collection : Vec<Vec<Vec<(T,T)>>>)  where T:  Display 
{
    let mut output_file : File = create_empty_file(path, filename, ending);    
    let mut writestring = String::new();

    let mut is_first_round = true;
    
    for polygonfamily in polygonfamily_collection{
        assert!(polygonfamily.len() > 0);
        let k = polygonfamily.len() -1;

        if is_first_round{
            is_first_round = false;
        }else {
            writestring.push_str("\n");
        }

        //add outer to file
        let outer_polygon = &polygonfamily[0];
        writestring.push_str(&format_polygon_to_string(outer_polygon)); 
        writestring.push_str("\n");

        //add k to file
        writestring.push_str(&format!("{}", k));

        //add inners to file
        for i in 1..polygonfamily.len(){
            let inner_polygon = &polygonfamily[i];
            writestring.push_str("\n");
            writestring.push_str(&format_polygon_to_string(inner_polygon));
        }
    }
    output_file.write_all(writestring.as_bytes());
}


//---------------------------------private functions----------------------------------------------------

fn format_polygon_to_string<T>(polygon : &Vec<(T,T)> ) -> String where T: Display {
    let mut writestring = "".to_string();
    let mut is_first_round = true;
    for (x,y) in polygon{
        if is_first_round{
            is_first_round = false;
        }else {
            writestring.push_str(" ");
        }
        writestring.push_str(&format!("{} {}", x,y));
    }

    writestring
}


