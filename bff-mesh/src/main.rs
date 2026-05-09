pub mod mesh;
pub mod soup;

fn main() -> Result<(), String> {
    let soup = soup::PolygonSoup::load_obj("resources/hemisphere.obj")?;

    println!("{:?}", soup);

    Ok(())
}
