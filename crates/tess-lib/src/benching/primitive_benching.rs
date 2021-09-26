use std::{fs::File, path::PathBuf};

use svg_gen::Primitive;

use super::Result;
use crate::{
    artifacts::PrimitiveTimeResult,
    targets::{SVGDocument, TessellationTarget},
    Tessellator,
};

pub fn time_primitive<P>(
    prim_name: String,
    primitive: Primitive,
    output: P,
    trials: u32,
) -> Result<()>
where
    P: Into<PathBuf>,
{
    let output_file = File::create(output.into())?;
    let mut csv_wtr = csv::Writer::from_writer(output_file);

    // For each backend, tessellate the files
    for mut backend in crate::backends::backends() {
        let backend: &mut dyn Tessellator = &mut *backend; // Unwrap & Shadow
        let max: u32 = 10_000;
        let step: u32 = 1000;
        let counts = std::iter::once(1 as u32).chain((step..=max).step_by(step as usize));
        for count in counts.clone() {
            let mut target: SVGDocument =
                SVGDocument::from(svg_gen::generate_svg(primitive, count, true));
            for _ in 0..trials {
                let (init_time, tess_time) = target.time(Box::new(backend));

                let result = PrimitiveTimeResult {
                    tessellator: backend.name().to_owned(),
                    primitive: prim_name.to_owned(),
                    amount: count,
                    init_time: init_time.as_nanos(),
                    tess_time: tess_time.as_nanos(),
                };
                csv_wtr.serialize(result)?;
            }
        }
    }

    csv_wtr.flush()?;

    Ok(())
}
