use pyo3::prelude::*;
use rayon;

#[pyclass]
#[derive(Debug)]
pub struct RayonThreadPool(rayon::ThreadPool);

/// Opaque rayon::Threadpool python wrapper
#[pymethods]
impl RayonThreadPool {
    #[new]
    pub fn new(threads: usize) -> Self {
        RayonThreadPool(
            rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap(),
        )
    }
}
