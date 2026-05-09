pub type Vertex = nalgebra::Point3<f32>;
pub type Index = u32;

pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<Index>,
}
