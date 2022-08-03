use pyo3::prelude::*;

// Fixed types for elements
pub type C = u8;
// Type of vectors of elements *for storage only* - other interfaces can still use Vec<C>
pub type CVec = Vec<C>;

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
