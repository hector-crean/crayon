use ply_rs::ply::{Ply, DefaultElement, Encoding, ElementDef, PropertyDef, PropertyType};
use std::fs::File;
use std::io::BufWriter;
use ply_rs::writer::Writer;
use las::Reader; // Ensure the las crate is used

pub fn convert_laz_to_ply(laz_path: &str, ply_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Open the LAZ file
    let laz_file = File::open(laz_path)?;

    // Prepare to write the PLY file
    let ply_file = File::create(ply_path)?;
    let mut ply_writer = BufWriter::new(ply_file);

    // Define the PLY structure
    let mut ply = Ply::<DefaultElement>::new();
    ply.header.encoding = Encoding::Ascii; // Set encoding to ASCII or Binary as needed

    // Declare the vertex element and its properties
    let mut laz_reader = Reader::new(laz_file)?;
    let mut vertex_def = ElementDef::new("vertex".to_string());
    vertex_def.properties.insert("x".to_string(), PropertyDef::new("x".to_string(), PropertyType::Scalar(ply_rs::ply::ScalarType::Float)));
    vertex_def.properties.insert("y".to_string(), PropertyDef::new("y".to_string(), PropertyType::Scalar(ply_rs::ply::ScalarType::Float)));
    vertex_def.properties.insert("z".to_string(), PropertyDef::new("z".to_string(), PropertyType::Scalar(ply_rs::ply::ScalarType::Float)));
    vertex_def.properties.insert("red".to_string(), PropertyDef::new("red".to_string(), PropertyType::Scalar(ply_rs::ply::ScalarType::UShort)));
    vertex_def.properties.insert("green".to_string(), PropertyDef::new("green".to_string(), PropertyType::Scalar(ply_rs::ply::ScalarType::UShort)));
    vertex_def.properties.insert("blue".to_string(), PropertyDef::new("blue".to_string(), PropertyType::Scalar(ply_rs::ply::ScalarType::UShort)));
    ply.header.elements.insert("vertex".to_string(), vertex_def);

    ply.payload.insert("vertex".to_string(), Vec::new()); // Initialize vertex list

    // Iterate over points in the LAZ file
    for point in laz_reader.points() {
        let point = point?; // Handle potential errors


        // In the point loop:
        if let Some(vertex_list) = ply.payload.get_mut("vertex") {
            let mut element = DefaultElement::new();
            element.insert("x".to_string(), ply_rs::ply::Property::Float(point.x as f32));
            element.insert("y".to_string(), ply_rs::ply::Property::Float(point.y as f32));
            element.insert("z".to_string(), ply_rs::ply::Property::Float(point.z as f32));
            match point.color {
                Some(color) => {
                    element.insert("red".to_string(), ply_rs::ply::Property::UShort(color.red));
                    element.insert("green".to_string(), ply_rs::ply::Property::UShort(color.green));
                    element.insert("blue".to_string(), ply_rs::ply::Property::UShort(color.blue));
                },
                None => {
                    element.insert("red".to_string(), ply_rs::ply::Property::UShort(255));
                    element.insert("green".to_string(), ply_rs::ply::Property::UShort(0));
                    element.insert("blue".to_string(), ply_rs::ply::Property::UShort(0));
                }
            }
          
         
            vertex_list.push(element);
        }
    }

    // Write the PLY structure to the file
    let writer = Writer::new();
    writer.write_ply(&mut ply_writer, &mut ply)?;

    Ok(())
}