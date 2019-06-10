
use serde::Deserialize;
use derive_builder::Builder;
use fuselage::{DataFrame, Reader, ReaderBuilder};


#[derive(Deserialize)]
struct Flower {
    sepal_length: f32,
    sepal_width: f32,
    petal_length: f32,
    petal_width: f32,
    species: String
}


fn main() -> Result<(), Box<std::error::Error>> {

    let reader = ReaderBuilder::default()
        .delimiter(b',')
        .has_headers(true)
        .path(format!("{}/data/iris.csv", env!("CARGO_MANIFEST_DIR")))
        .build()?;

    let df: DataFrame<Flower> = reader.read()?;

    assert_eq!(df.len(), 150);

    Ok(())
}
