use pyo3::prelude::*;

pub type C = u8;

mod item_sets;
mod packing_bestfit;
mod packing_branching;
mod packing_common;
pub use item_sets::ItemSets;

// Init

#[pymodule]
fn binpack_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ItemSets>()?;
    Ok(())
}

// Tests

//#[cfg(test)]
mod test;
