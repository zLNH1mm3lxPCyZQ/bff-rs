use crate::soup::PolygonSoup;

pub struct Vertex {
    position: nalgebra::Point3<f32>,
    normal: nalgebra::Vector3<f32>,
}
pub struct Edge {
    index: u32,
    half_edge_1: u32,
    halg_edge_2: u32,
}
pub struct Face {
    index: u32,
    is_boundary: bool,
}
pub struct HalfEdge {
    next_half_edge: u32,
    previous_half_edge: u32,
    flipped_half_edge: u32,
    origin_vertex: u32,
    edge: u32,
    face: u32,
    oppsosite_corner: u32,
    is_boundary: bool,
}

pub struct Mesh {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
    faces: Vec<Face>,
    half_edges: Vec<HalfEdge>,
}

impl Mesh {
    fn build(soup: &PolygonSoup) {}
}
