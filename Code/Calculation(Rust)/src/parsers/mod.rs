pub(crate) mod polygonparser;
pub(crate) mod areaparser;
pub(crate) mod polygon_from_file;
pub(crate) mod polygon_to_file;
pub(crate) mod to_file;

use osmpbfreader;

use osmpbfreader::{Relation, Tags};
use std::sync::mpsc::Sender;
pub(crate) use polygonparser::PolygonParsingError;

pub type NodeId = osmpbfreader::NodeId;
pub type AreaId = osmpbfreader::RelationId;
pub type SegmentId = osmpbfreader::WayId;

pub type Tag = osmpbfreader::Tags;


pub type Latitude = i32;
pub type Longitude = i32;
pub type AdminLevel = u8;

pub(crate) use areaparser::import_areas;


#[derive(Debug)]
pub struct Node {
    pub osmid: NodeId,
    pub decimicro_lat: Latitude,
    pub decimicro_lon: Longitude,
}

impl std::string::ToString for Node {
    fn to_string(&self) -> String{
        format!("NodeId:{}, Lon:{}, Lat:{}", &self.osmid.0, &self.lon(), &self.lat()).to_string()
    }
}

impl Node {
    pub fn get_coordinates(&self) -> String{
        //gemäß Geo-JSON Notation
        format!("[{},{}]", &self.lon(), &self.lat()).to_string()
    }
    pub fn lon(&self) -> f64 {
        let mut result : f64 = self.decimicro_lon.into();
        result = result / 10_000_000.0;
        result
    }
    pub fn lat(&self) -> f64 {
        let mut result : f64 = self.decimicro_lat.into();
        result = result / 10_000_000.0;
        result
    }
}

#[derive(Debug)]
pub struct Segment {
    pub osmid: SegmentId,
    pub nodes: Vec<NodeId>,
}


pub(crate) trait AreaFactory {
    type Area;

    ///
    /// Check for a given tag set of a relation if it is a valid area
    /// description
    ///
    fn is_valid(&self, tags: &Tags) -> bool;

    ///
    /// Create an area from the given relation.
    /// The <inner> and <outer>_segment_sender are channels to send the ids
    /// to. The corresponding segments and ids will be collected from the pbf
    /// if possible
    ///
    fn to_area(
        &self,
        rel: &Relation,
        inner_id_sender: &Sender<SegmentId>,
        outer_id_sender: &Sender<SegmentId>,
    ) -> Option<Self::Area>;

    ///
    /// Set the segments and their referenced nodes which where imported from
    /// the pbf file.
    ///
    fn set_segments(&mut self, segments: Vec<Segment>, nodes: Vec<Node>);
}
