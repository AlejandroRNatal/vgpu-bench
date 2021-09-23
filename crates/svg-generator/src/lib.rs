extern crate clap;
extern crate dynfmt;

mod commands;
mod primitives;
mod writer;

use crate::primitives::Primitive;
use crate::writer::Writer;
use std::{fs::File, io::Write, path::Path};

pub fn generate_svg(primitive: Primitive, count: i32, rotate: bool) -> String {
    let mut writer = Writer::default();
    writer.write_primitives(primitive, count, rotate);
    writer.get_document()
}

pub fn output_svg(primitive: Primitive, count: i32, rotate: bool, path: &Path) -> Result<(), std::io::Error> {
    let mut file = File::create(path)?;
    let contents = generate_svg(primitive, count, rotate);
    file.write_all(contents.as_bytes())?;
    Ok(())
}