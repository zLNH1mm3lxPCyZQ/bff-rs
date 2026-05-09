use std::io::BufRead;
use std::{fs::File, io::BufReader, path::PathBuf};

pub type Vertex = nalgebra::Point3<f32>;
pub type Index = u32;

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<Index>,
}

impl Mesh {
    pub fn load_obj(file: &str) -> Result<Self, String> {
        let path = PathBuf::from(&file);
        if !path.exists() {
            return Err("The file provided does not exists.".to_string());
        }
        let file = File::open(&path).map_err(|_| "Unable to open this file.".to_string())?;
        let reader = BufReader::new(file);

        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<Index> = Vec::new();

        for (line_no, line) in reader.lines().enumerate() {
            let line = line.map_err(|_| {
                format!("Invalid UTF-8 charachters in this line: line {}", line_no).to_string()
            })?;

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if line.starts_with("v ") {
                let parts: Vec<&str> = line.split_whitespace().collect();

                // 4 parts are "v", x, y, z
                if parts.len() != 4 {
                    return Err(format!(
                        "The file contains invalid format at line {}",
                        line_no
                    ));
                }

                let x: f32 = parts[1]
                    .parse()
                    .map_err(|_| format!("Invalid x value at line {}", line_no))?;
                let y: f32 = parts[2]
                    .parse()
                    .map_err(|_| format!("Invalid y value at line {}", line_no))?;
                let z: f32 = parts[3]
                    .parse()
                    .map_err(|_| format!("Invalid z value at line {}", line_no))?;

                vertices.push(Vertex::new(x, y, z));
            } else if line.starts_with("f ") {
                // For now, we do not evaluate obj files with face expressions as 0//2 or 0/12/34 for example

                let parts: Vec<&str> = line.split_whitespace().collect();

                // 4 parts are "f", i_1, i_2, i_3
                if parts.len() != 4 {
                    return Err(format!(
                        "The file contains invalid format at line {}",
                        line_no
                    ));
                }

                let i_1: Index = parts[1]
                    .parse()
                    .map_err(|_| format!("Invalid face value at line {}", line_no))?;
                let i_2: Index = parts[2]
                    .parse()
                    .map_err(|_| format!("Invalid face value at line {}", line_no))?;
                let i_3: Index = parts[1]
                    .parse()
                    .map_err(|_| format!("Invalid face value at line {}", line_no))?;

                indices.push(i_1);
                indices.push(i_2);
                indices.push(i_3);
            }
        }

        // sanity checks
        let n_vertices = vertices.len() as u32;
        if indices.iter().any(|index| *index >= n_vertices) {
            return Err(
                "Some faces indices are out of bounds with the vertices array.".to_string(),
            );
        }

        Ok(Self { vertices, indices })
    }
}

fn main() {}
