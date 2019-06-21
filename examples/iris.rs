
use serde::Deserialize;
use fuselage::{DataFrame, CsvReaderBuilder};


// This represents a row within the dataframe
#[derive(Deserialize)]
struct Flower {
    sepal_length: f32,
    sepal_width: f32,
    petal_length: f32,
    petal_width: f32,
    species: String
}




fn main() -> Result<(), Box<std::error::Error>> {

    // Construct the reader
    let reader = CsvReaderBuilder::default()
        .delimiter(b',')
        .has_headers(true)
        .path(format!("{}/data/iris.csv", env!("CARGO_MANIFEST_DIR")))
        .build()?;

    // Read the dataframe
    let df: DataFrame<Flower> = reader.read()?;

    // Get the groups of species
    let species: GroupBy<Flower> = groupby!(&df.species);

    assert_eq!(df.len(), 150);

    Ok(())
}
