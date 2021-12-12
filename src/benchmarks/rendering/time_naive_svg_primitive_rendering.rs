use crate::benchmarks::{
    Benchmark, BenchmarkBuilder, BenchmarkData, BenchmarkFn,
};
use crate::Result;
use crate::{log_assert, util};
use benchmark_macro_derive::BenchmarkData;
use erased_serde::Serialize;
use log::{debug, info, trace, warn};
use rendering_util::benching::output::NaivePrimitiveRenderTime;
use std::path::PathBuf;
use svg_generator::Primitive;
use tessellation_util::backends::Tessellator;

#[derive(Debug, BenchmarkData)]
pub struct TimeNaiveSVGPrimitiveRendering {
    backends: Vec<Box<dyn Tessellator>>,
    primitives: Vec<Primitive>,
    primitive_count: u32,
    frames: usize,
    csv_output: Option<&'static str>,
    plot_output: Option<&'static str>,
}

impl TimeNaiveSVGPrimitiveRendering {
    pub fn new() -> Self {
        TimeNaiveSVGPrimitiveRendering {
            backends: Vec::new(),
            primitives: Vec::new(),
            primitive_count: 0,
            frames: 0,
            csv_output: None,
            plot_output: None,
        }
    }

    pub fn backend(mut self, backend: Box<dyn Tessellator>) -> Self {
        self.backends.push(backend);
        self
    }

    pub fn primitive(mut self, primitive: Primitive) -> Self {
        self.primitives.push(primitive);
        self
    }

    pub fn primitives<I>(mut self, primitives: I) -> Self
    where
        I: IntoIterator<Item = Primitive>,
    {
        self.primitives.extend(primitives);
        self
    }

    pub fn primitive_count(mut self, primitive_count: u32) -> Self {
        self.primitive_count = primitive_count;
        self
    }

    pub fn frames(mut self, frames: usize) -> Self {
        self.frames = frames;
        self
    }

    pub fn to_csv(mut self, path: &'static str) -> Self {
        self.csv_output = Some(path);
        self
    }

    pub fn to_plot(mut self, path: &'static str) -> Self {
        self.plot_output = Some(path);
        self
    }
}

impl BenchmarkBuilder for TimeNaiveSVGPrimitiveRendering {
    fn build(self: Box<Self>) -> Result<BenchmarkFn> {
        // Input check
        if let Some(path) = self.csv_output {
            log_assert!(
                PathBuf::from(path).is_relative(),
                "{path} is not a relative path"
            );
        } else {
            warn!("no output path was provided; results will be dropped");
        }
        if let Some(path) = self.plot_output {
            log_assert!(
                PathBuf::from(path).is_relative(),
                "{} is not a relative path",
                path
            );
            log_assert!(
                self.csv_output.is_some(),
                "you cannot save a plot without an output path set"
            )
        }
        log_assert!(self.backends.len() > 0, "no backends were provided");
        log_assert!(self.primitives.len() > 0, "no primitive were provided");
        log_assert!(
            self.primitive_count > 0,
            "primitive count must be greater than 0"
        );
        log_assert!(self.frames > 0, "frames must be greater than 0");

        // Write benchmark
        BenchmarkFn::from(move |options| {
            trace!(
                "commencing naive SVG primitive rendering frametime capture"
            );
            debug!("options: {:?}", self);

            // Bandaid fix: wgpu uses the same logger - Disable logging
            // temporarily
            let prev_level = log::max_level();
            log::set_max_level(log::LevelFilter::Off);
            // Collect results
            let mut results: Vec<NaivePrimitiveRenderTime> = Vec::new();
            for mut backend in self.backends {
                let backend: &mut dyn Tessellator = backend.as_mut();
                for primitive in &self.primitives {
                    results.extend(
                        rendering_util::benching::timing::time_naive_primitive(
                            backend,
                            primitive.to_owned(),
                            self.primitive_count,
                            self.frames,
                        )?,
                    );
                }
            }
            // Bandaid removal
            log::set_max_level(prev_level);

            // Write results
            if let Some(path) = self.csv_output {
                let path = options.benchmark_dir().join(path);
                let rows: Vec<Box<dyn Serialize>> = results
                    .into_iter()
                    .map(|x| -> Box<dyn Serialize> { Box::new(x) })
                    .collect();
                util::write_csv(&path, &rows)?;
                info!("output CSV data to '{}'", &path.display());
            }

            // Plot results
            if let Some(plot_output) = self.plot_output {
                let mut csv_path =
                    options.benchmark_dir().join(self.csv_output.unwrap());
                csv_path.set_extension("csv");

                let _proc_output = util::call_program(
                    "python3",
                    [
                        "tools/plotter/plot_naive_frametimes_primitives.py",
                        csv_path.to_str().unwrap(),
                        options.benchmark_dir().to_str().unwrap(),
                        plot_output,
                    ],
                )?;
                info!(
                    "output plot to '{}'",
                    options.benchmark_dir().join(plot_output).display()
                );
            }

            trace!("completed naive SVG primitive rendering frametime capture");
            Ok(())
        })
    }
}
