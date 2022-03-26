#[allow(unused_imports)]
use naive_renderer::NaiveRenderer;
use renderer::targets::{SVGDocument, SVGFile};
use renderer::Renderer;
use std::path::PathBuf;
use vgpu_bench::macros::measurement;
use vgpu_bench::prelude::*;

#[measurement]
struct RenderTime {
    filename: String,
    frame: usize,
    time_ns: u128,
}

pub fn main() -> Result<()> {
    // Init logging
    vgpu_bench::util::logging::init_default();

    let bm_fn = move |_files| {
        let files: Vec<PathBuf> = _files;
        println!("{files:?}");
        let mut measurements: Measurements<RenderTime> = Measurements::new();
        // Benchmarking
        for file in files.iter() {
            let f = SVGFile::from(file);
            let d = SVGDocument::try_from(f).unwrap();
            for _ in 0..10 {
                let prev_level = log::max_level();
                log::set_max_level(log::LevelFilter::Off);
                let mut renderer = Box::new(NaiveRenderer::new());
                renderer.init().unwrap();
                renderer.stage(&d).unwrap();
                let results = renderer.render(50).unwrap().frame_times.clone();
                drop(renderer);
                log::set_max_level(prev_level);

                for frame in 0..results.len() {
                    measurements.push(RenderTime {
                        filename: file
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string(),
                        frame: frame + 1,
                        time_ns: results.get(frame).unwrap().as_nanos(),
                    })
                }
            }
        }
        Ok(measurements)
    };

    let files = vgpu_bench::util::io::get_files_with_extension(
        "assets/svg/examples",
        false,
        "svg",
    );
    let bm_fn = BenchmarkFn::new(move || bm_fn(files));
    let mut bm_ = Benchmark::new(BenchmarkMetadata::new("Frametimes"), bm_fn);
    let bundle1 = bm_.run(&DriverOptions::default()).unwrap();
    bundle1.write("output").unwrap();

    Ok(())
}