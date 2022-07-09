use rayon::iter::ParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use crate::C;
use crate::{counts_to_sizes, fits_into_bestfit, sizes_to_counts};
use pyo3::prelude::*;

/// Each element is a set of items (Matej calls them "History"),
/// encoded as counts of each size: the number of items of
/// size `size` is `item_set.0[size]`
#[pyclass]
#[derive(Debug, Clone, Default)]
pub struct ItemsSet(Vec<Vec<C>>);

#[pymethods]
impl ItemsSet {
    /// Creates a new instance for the given item set
    /// (given as an iterable of count vectors)
    #[new]
    pub fn new(items_set: &pyo3::types::PySequence) -> PyResult<Self> {
        let mut s: Self = Default::default();
        for counts in items_set.iter()? {
            s.push_counts(counts?)?;
        }
        Ok(s)
    }

    /// Insert new item given by counts
    pub fn push_counts(&mut self, counts: &PyAny) -> PyResult<()> {
        let cs: Vec<C> = counts.extract()?;
        assert!(cs.len() < C::MAX as usize);
        assert!(cs.len() == 0 || cs[0] == 0);
        self.0.push(cs);
        Ok(())
    }

    /// Insert new item given by counts
    pub fn push_sizes(&mut self, sizes: &PyAny) -> PyResult<()> {
        let ss: Vec<C> = sizes.extract()?;
        assert!(ss.len() < C::MAX as usize);
        self.0.push(sizes_to_counts(&ss));
        Ok(())
    }

    pub fn all_counts(&self) -> Vec<Vec<C>> {
        self.0.clone()
    }

    pub fn all_sizes(&self) -> Vec<Vec<C>> {
        self.0.iter().map(|c| counts_to_sizes(c)).collect()
    }

    pub fn __len__(&self) -> usize {
        self.0.len()
    }

    pub fn __getitem__(&self, idx: usize) -> PyResult<Vec<C>> {
        self.0
            .get(idx)
            .cloned()
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Out of bounds"))
    }

    pub fn __repr__(&self) -> String {
        format!("ItemsSet({:?})", self.0)
    }

    /// Return an estimate of the memory used by the ItemsSet.
    /// Also includes the vector headers and any unused vector capacity, does not include padding.
    pub fn memory_used(&self) -> usize {
        // Size of a Vec header, same for Vec<Vec<C>>
        let vs = std::mem::size_of::<Vec<C>>();
        return vs
            + self.0.capacity() * vs
            + self
                .0
                .iter()
                .map(|v| v.capacity() * std::mem::size_of::<C>())
                .sum::<usize>();
    }

    #[allow(non_snake_case)]
    pub fn any_fits_into_bestfit(&self, counts: &PyAny) -> PyResult<bool> {
        let cs: Vec<C> = counts.extract()?;
        assert!(cs.len() < C::MAX as usize);
        assert!(cs.len() == 0 || cs[0] == 0);
        Ok(self.0.iter().any(|c| fits_into_bestfit(c, &cs)))
    }

    #[allow(non_snake_case)]
    pub fn par_any_fits_into_bestfit(&self, counts: &PyAny) -> PyResult<bool> {
        let cs: Vec<C> = counts.extract()?;
        assert!(cs.len() < C::MAX as usize);
        assert!(cs.len() == 0 || cs[0] == 0);
        Ok(self.0.par_iter().any(|c| fits_into_bestfit(c, &cs)))
    }
}
