use crate::packing_bestfit::fits_into_bestfit;
use crate::packing_branching::fits_into_branching;
use crate::packing_common::{counts_to_sizes, sizes_to_counts};
use crate::{CVec, C};
use pyo3::prelude::*;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

/// Each element is a set of items (Matej calls them "History"),
/// encoded as counts of each size: the number of items of
/// size `size` is `item_set.0[size]`
#[pyclass]
#[derive(Debug, Clone, Default)]
#[pyo3(text_signature = "(all_counts=None, /, all_sizes=None)")]
pub struct ItemSets(Vec<CVec>);

#[pymethods]
impl ItemSets {
    /// Creates a new instance for the given item set
    /// (given as an iterable of count vectors)
    #[new]
    #[args(all_counts = "None", all_sizes = "None")]
    pub fn new(
        all_counts: Option<&pyo3::types::PySequence>,
        all_sizes: Option<&pyo3::types::PySequence>,
    ) -> PyResult<Self> {
        let mut s: Self = Default::default();
        if let Some(cs) = all_counts {
            for counts in cs.iter()? {
                s.push_counts(counts?)?;
            }
        }
        if let Some(ss) = all_sizes {
            for sizes in ss.iter()? {
                s.push_sizes(sizes?)?;
            }
        }
        Ok(s)
    }

    /// Convert count vector into sizes vector
    #[staticmethod]
    #[pyo3(text_signature = "(counts)")]
    pub fn c2s(py: Python, counts: &PyAny) -> PyResult<PyObject> {
        let cs: Vec<C> = counts.extract()?;
        Ok(counts_to_sizes(&cs).into_py(py))
    }

    /// Convert sizes vector into counts vector
    #[staticmethod]
    #[pyo3(text_signature = "(sizes)")]
    pub fn s2c(py: Python, sizes: &PyAny) -> PyResult<PyObject> {
        let ss: Vec<C> = sizes.extract()?;
        Ok(sizes_to_counts(&ss).into_py(py))
    }

    /// Insert new item given by counts
    pub fn push_counts(&mut self, counts: &PyAny) -> PyResult<()> {
        let cs: Vec<C> = counts.extract()?;
        assert!(cs.len() < C::MAX as usize);
        assert!(cs.len() == 0 || cs[0] == 0);
        self.0.push(cs.into());
        Ok(())
    }

    /// Insert new item given by counts
    pub fn push_sizes(&mut self, sizes: &PyAny) -> PyResult<()> {
        let ss: Vec<C> = sizes.extract()?;
        assert!(ss.len() < C::MAX as usize);
        let cs = sizes_to_counts(&ss);
        assert_eq!(cs[0], 0, "Items of size 0 not allowed.");
        self.0.push(cs.into());
        Ok(())
    }

    pub fn all_counts(&self) -> Vec<Vec<C>> {
        self.0.iter().map(|c| c.clone().into()).collect()
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
            .map(|c| c.into())
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Out of bounds"))
    }

    pub fn __repr__(&self) -> String {
        format!("ItemsSet(all_counts={:?})", self.0)
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
    /// `par` invokes parallelism, `branching=0` means best-fit, higher values
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
    /// `par` invokes parallelism, `branching=0` means best-fit, higher values
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
    /// `par` invokes parallelism, `branching=0` means best-fit, higher values
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
    /// `par` invokes parallelism, `branching=0` means best-fit, higher values
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

    /// Count how many of the stored item sets fit into the item set given by `counts`.
    ///
    /// `par` invokes parallelism, `branching=0` means best-fit, higher values
    /// do a partial exhaustive search limiting the branch count (switching to best-fit afterwards).
    #[args(par = false, branching = 0)]
    #[pyo3(text_signature = "($self, counts, /, par=False, branching=0)")]
    pub fn how_many_fit_into_given(
        &self,
        counts: &PyAny,
        par: bool,
        branching: usize,
    ) -> PyResult<usize> {
        self.count_f_helper(counts, par, |sc, gc| fits_into_branching(sc, gc, branching))
    }

    /// Count into how many of the stored item sets does the item set given by `counts` fit.
    ///
    /// `par` invokes parallelism, `branching=0` means best-fit, higher values
    /// do a partial exhaustive search limiting the branch count (switching to best-fit afterwards).
    #[args(par = false, branching = 0)]
    #[pyo3(text_signature = "($self, counts, /, par=False, branching=0)")]
    pub fn given_fits_into_how_many(
        &self,
        counts: &PyAny,
        par: bool,
        branching: usize,
    ) -> PyResult<usize> {
        self.count_f_helper(counts, par, |sc, gc| fits_into_branching(gc, sc, branching))
    }

    /// Check if any of the stored item sets fit into the item set given by `counts` (for benchmark only).
    ///
    /// This `par` invokes parallelism (set ), with `trim_upper=True` first looks for any necessary packing of
    /// largest items into the only one larger bins, repeatedly.
    #[args(par = false, trim_upper = false)]
    #[pyo3(text_signature = "($self, counts, /, par=False, trim_upper=False)")]
    pub fn bestfit_any_fit_into_given(
        &self,
        counts: &PyAny,
        par: bool,
        trim_upper: bool,
    ) -> PyResult<bool> {
        self.any_f_helper(counts, par, |sc, gc| fits_into_bestfit(sc, gc, trim_upper))
    }
}

impl ItemSets {
    fn count_f_helper<F>(&self, counts: &PyAny, par: bool, f: F) -> PyResult<usize>
    where
        F: Fn(&[C], &[C]) -> bool + Sync,
    {
        let gc: Vec<C> = counts.extract()?;
        assert!(gc.len() < C::MAX as usize);
        assert!(gc.len() == 0 || gc[0] == 0);
        if par {
            Ok(self.0.par_iter().filter(|sc| f(sc, &gc)).count())
        } else {
            Ok(self.0.iter().filter(|sc| f(sc, &gc)).count())
        }
    }

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
