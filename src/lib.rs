use pyo3::prelude::*;

pub type C = u8;

mod items_set;
mod packing_bestfit;
mod packing_branching;
mod packing_common;
pub use items_set::ItemSets;

// Init

#[pymodule]
fn binpack_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ItemSets>()?;
    Ok(())
}

// Tests

//#[cfg(test)]
mod test;
