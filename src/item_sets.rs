use crate::packing_branching::fits_into_branching;
use crate::packing_common::{counts_to_sizes, sizes_to_counts};
use crate::C;
use pyo3::prelude::*;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

/// Each element is a set of items (Matej calls them "History"),
/// encoded as counts of each size: the number of items of
/// size `size` is `item_set.0[size]`
#[pyclass]
#[derive(Debug, Clone, Default)]
pub struct ItemSets(Vec<Vec<C>>);

#[pymethods]
impl ItemSets {
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

    /// Check if any of the stored item sets fit into the item set given by `counts`.
    ///
    /// `par` invokes parallelism (set ), `branching=0` means best-fit, higher values
    /// do a partial exhaustive search limiting the branch count (switching to best-fit afterwards).
    #[args(par = false, branching = 0)]
    #[pyo3(text_signature = "($self, counts, /, par=False, branching=0)")]
    pub fn any_fit_into_given(
        &self,
        counts: &PyAny,
        par: bool,
        branching: usize,
    ) -> PyResult<bool> {
        self.any_f_helper(counts, par, |sc, gc| fits_into_branching(sc, gc, branching))
    }

    /// Check if the item set given by `counts` fits into any of the stored item sets.
    ///
    /// `par` invokes parallelism (set ), `branching=0` means best-fit, higher values
    /// do a partial exhaustive search limiting the branch count (switching to best-fit afterwards).
    #[args(par = false, branching = 0)]
    #[pyo3(text_signature = "($self, counts, /, par=False, branching=0)")]
    pub fn given_fits_into_any(
        &self,
        counts: &PyAny,
        par: bool,
        branching: usize,
    ) -> PyResult<bool> {
        self.any_f_helper(counts, par, |sc, gc| fits_into_branching(gc, sc, branching))
    }

    /// Check if all of the stored item sets fit into the item set given by `counts`.
    ///
    /// `par` invokes parallelism (set ), `branching=0` means best-fit, higher values
    /// do a partial exhaustive search limiting the branch count (switching to best-fit afterwards).
    #[args(par = false, branching = 0)]
    #[pyo3(text_signature = "($self, counts, /, par=False, branching=0)")]
    pub fn all_fit_into_given(
        &self,
        counts: &PyAny,
        par: bool,
        branching: usize,
    ) -> PyResult<bool> {
        self.all_f_helper(counts, par, |sc, gc| fits_into_branching(sc, gc, branching))
    }

    /// Check if the item set given by `counts` fits into all of the stored item sets.
    ///
    /// `par` invokes parallelism (set ), `branching=0` means best-fit, higher values
    /// do a partial exhaustive search limiting the branch count (switching to best-fit afterwards).
    #[args(par = false, branching = 0)]
    #[pyo3(text_signature = "($self, counts, /, par=False, branching=0)")]
    pub fn given_fits_into_all(
        &self,
        counts: &PyAny,
        par: bool,
        branching: usize,
    ) -> PyResult<bool> {
        self.all_f_helper(counts, par, |sc, gc| fits_into_branching(gc, sc, branching))
    }
}

impl ItemSets {
    fn any_f_helper<F>(&self, counts: &PyAny, par: bool, f: F) -> PyResult<bool>
    where
        F: Fn(&[C], &[C]) -> bool + Sync,
    {
        let gc: Vec<C> = counts.extract()?;
        assert!(gc.len() < C::MAX as usize);
        assert!(gc.len() == 0 || gc[0] == 0);
        if par {
            Ok(self.0.par_iter().any(|sc| f(sc, &gc)))
        } else {
            Ok(self.0.iter().any(|sc| f(sc, &gc)))
        }
    }

    fn all_f_helper<F>(&self, counts: &PyAny, par: bool, f: F) -> PyResult<bool>
    where
        F: Fn(&[C], &[C]) -> bool + Sync,
    {
        let gc: Vec<C> = counts.extract()?;
        assert!(gc.len() < C::MAX as usize);
        assert!(gc.len() == 0 || gc[0] == 0);
        if par {
            Ok(self.0.par_iter().all(|sc| f(sc, &gc)))
        } else {
            Ok(self.0.iter().all(|sc| f(sc, &gc)))
        }
    }
}
