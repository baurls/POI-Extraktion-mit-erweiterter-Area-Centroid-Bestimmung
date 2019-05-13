#[derive(Debug)]
pub struct Vertex{
    pub id : usize,
    pub pfad_vorgaenger_edge_id : usize,
    pub ausgehende_kanten_ids : Vec<usize>,
    pub position : (i64,i64),
    pub cost_from_s : u64,
    pub is_endvertex : bool
}
impl Vertex {
    pub fn beeline_distance_to(&self, other: &Vertex) -> u64{
        let xdiff = self.position.0 - other.position.0;
        let ydiff = self.position.1 - other.position.1;

        let a_squared = (xdiff * xdiff) as u64;
        let b_squared = (ydiff * ydiff) as u64;
        ((a_squared + b_squared) as f64).sqrt() as u64
    }
    pub fn add_edge(&mut self, edge_nr: usize){
        self.ausgehende_kanten_ids.push(edge_nr);
    }
    pub fn set_path_vorgaenger_edge(&mut self, edge_nr: usize){
        self.pfad_vorgaenger_edge_id = edge_nr;
    }
    pub fn new(id:usize,position:(i64,i64), endpoint : bool) -> Vertex{
        Vertex{
            id : id,
            pfad_vorgaenger_edge_id : id,
            ausgehende_kanten_ids : Vec::new(),
            position : position,
            cost_from_s : 0,
            is_endvertex : endpoint
        }
    }
    pub fn get_adjacient_vertices(&self, all_edges: &Vec<Edge>) -> Vec<usize>{
        let mut adjacient_vertices : Vec<usize> = Vec::new();

        for ausgehende_kanten_id in &self.ausgehende_kanten_ids{
            let kante = &all_edges[*ausgehende_kanten_id];
            let other_vertex_id = kante.get_opposide_vertex(self.id);
            if !adjacient_vertices.contains(&other_vertex_id){
                adjacient_vertices.push(other_vertex_id);
            }
        }
        

        adjacient_vertices 
    }
}
impl Clone for Vertex{
    fn clone(&self) -> Vertex{
        Vertex{
            id : self.id,
            pfad_vorgaenger_edge_id : self.pfad_vorgaenger_edge_id,
            ausgehende_kanten_ids : self.ausgehende_kanten_ids.clone(),
            position : self.position,
            cost_from_s : self.cost_from_s,
            is_endvertex : self.is_endvertex
        }
    }
}

#[derive(Debug)]
pub struct Edge{
    pub id : usize,
    pub connects : (usize, usize),
    pub cost : u64
}
impl Edge {
    pub fn get_opposide_vertex(&self, request_vertex : usize) -> usize{
        if self.connects.0 == request_vertex{
            return self.connects.1;
        }
        if self.connects.1 == request_vertex{
            return self.connects.0;
        }
        panic!("Vertex has to be part of the edge.");
    }
} 

