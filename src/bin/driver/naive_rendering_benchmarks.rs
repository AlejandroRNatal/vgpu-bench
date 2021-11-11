use std::path::PathBuf;

use crate::dictionary::*;
use const_format::concatcp;
use log::{debug, error, info, trace};
use vgpu_bench::benchmarks::rendering::naive_primitive_rendering::PrimitiveNaiveRenderingOptions;
use vgpu_bench::benchmarks::rendering::naive_svg_rendering::SVGNaiveRenderingOptions;
use vgpu_bench::{benchmarks, util};

pub fn frametimes_svg_examples<P>(input_dir_path: P)
where
    P: Into<PathBuf>,
{
    let input_files = util::get_files_with_extension(input_dir_path, false, "svg");
    let output_path = concatcp![OUTPUT_DIR, DATA, EXAMPLES, SVG, "naive_frametimes.csv"];
    let writer = util::csv_writer(output_path).expect("Could not create output file");
    let backend = tessellation_util::backends::default();
    let frames = 500;
    let options = SVGNaiveRenderingOptions::new()
        .writer(writer)
        .assets(input_files)
        .backend(backend)
        .frames(frames);
    debug!("Options: {:?}", options);

    trace!("Commencing naive SVG example rendering for frametime capture");
    match benchmarks::rendering::naive_svg_rendering::write_frametimes(options) {
        Ok(_) => {
            trace!("Completed naive SVG example rendering for frametime capture");
            info!(
                "Completed naive SVG example rendering for frametime capture. Output to '{}'",
                output_path
            );
        }
        Err(err) => error!("{:?}", err),
    }
}

pub fn frametimes_svg_primitives() {
    let output_path = concatcp![OUTPUT_DIR, DATA, PRIMITIVES, SVG, "naive_frametimes.csv"];
    let writer = util::csv_writer(output_path).expect("Could not create output file");
    let backend = tessellation_util::backends::default();
    let primitives = svg_generator::primitives::default();
    let primitive_count = 1;
    let frames = 500;
    let options = PrimitiveNaiveRenderingOptions::new()
        .writer(writer)
        .primitives(primitives)
        .primitive_count(primitive_count)
        .backend(backend)
        .frames(frames);
    debug!("Options: {:?}", options);

    trace!("Commencing naive SVG primitive rendering for frametime capture");
    match benchmarks::rendering::naive_primitive_rendering::write_frametimes(options) {
        Ok(_) => {
            trace!("Completed naive SVG primitive rendering for frametime capture");
            info!(
                "Completed naive SVG primitive rendering for frametime capture. Output to '{}'",
                output_path
            );
        }
        Err(err) => error!("{:?}", err),
    }
}
