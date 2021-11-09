use crate::dictionary::{EXAMPLES_ASSETS_DIR, EXAMPLES_OUTPUT_DIR};
use crate::dictionary::{PRIMITIVES_ASSETS_DIR, PRIMITIVES_OUTPUT_DIR};
use const_format::concatcp;
use log::{debug, error, info, trace};
use vgpu_bench::benchmarks::rendering::naive_primitive_rendering::PrimitiveNaiveRenderingOptions;
use vgpu_bench::benchmarks::rendering::naive_svg_rendering::SVGNaiveRenderingOptions;
use vgpu_bench::{benchmarks, util};

pub fn frametimes_svg_examples() {
    let input_dir_path = EXAMPLES_ASSETS_DIR;
    let output_path = concatcp![EXAMPLES_OUTPUT_DIR, "naive_frametimes.csv"];
    let input_files = util::get_files_with_extension(input_dir_path, false, "svg");
    let output_file = util::create_file(output_path).expect("Could not create output file");
    let writer = csv::Writer::from_writer(output_file);
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
    let output_path = concatcp![PRIMITIVES_OUTPUT_DIR, "naive_frametimes.csv"];
    let output_file = util::create_file(output_path).expect("Could not create output file");
    let writer = csv::Writer::from_writer(output_file);
    let backend = tessellation_util::backends::default();
    let primitives = svg_generator::primitives();
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