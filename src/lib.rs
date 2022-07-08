use pyo3::prelude::*;
use std::cmp::max;
use std::cmp::min;

pub type C = u8;

/// Conver a count vector into a list of item sizes, decreasing
pub fn counts_to_sizes(counts: &[C]) -> Vec<C> {
    assert!(counts.len() < C::MAX as usize);
    assert!(counts.len() == 0 || counts[0] == 0);
    let mut res = Vec::new();
    for (i, &val) in counts.iter().enumerate().rev() {
        for _ in 0..val {
            res.push(i as u8);
        }
    }
    res
}

/// Conver a count list of item sizes into a count vector
pub fn sizes_to_counts(sizes: &[C]) -> Vec<C> {
    let m = *sizes.iter().max().unwrap_or(&0);
    let mut res = vec![0; m as usize + 1];
    for s in sizes {
        res[*s as usize] += 1;
    }
    res
}

/// Given counts of individual items, return the sum of their sizes
pub fn item_sum<T: Into<i64> + Copy>(counts: &[T]) -> i64 {
    counts
        .iter()
        .enumerate()
        .map(|(i, val)| (i as i64) * ((*val).into()))
        .sum()
}

/// Check if "items" `a` fit into "bins" `b` via best-fit matching.
/// Both `a` and `b` are given as counts of items of every size.
/// `a` and `b` must have the same length.
///
/// This works by substracting `d=a-b` and then trying to eliminate all
/// positive numbers in `d` from smallest indices up.
/// ```
/// a-b = ....+.+...-...-..+..---.++.-....
///           ^     ^                    
///           pi    ni
/// ```
/// During the matching, `pi` and `ni` point to the lowest index of a
/// positive resp. negative count. All negative counts under `pi`
/// are irellevant and thus erased. `sd` track the sum of the item
/// sizes and is updated by forgotten negative bins, so we can stop if
/// it is ever positive.
#[allow(non_snake_case)]
pub fn fits_into_BF(a: &[C], b: &[C]) -> bool {
    let l = max(a.len(), b.len());
    // Difference and its sum
    let mut d: Vec<i32> = vec![0; l];
    for i in 0..l {
        d[i] += *a.get(i).unwrap_or(&0) as i32;
        d[i] -= *b.get(i).unwrap_or(&0) as i32;
    }
    let mut sd: i32 = item_sum(&d) as i32;

    let mut ni = 0;
    for pi in 0..l {
        debug_assert!(item_sum(&d) as i32 == sd);
        // Check if total volume fits
        if sd > 0 {
            return false;
        }
        // pi points to a negative number -> forget it and remove from the sum
        if d[pi] < 0 {
            sd -= (pi as i32) * d[pi];
            d[pi] = 0;
        }
        // pi points to items in need of packing -> resolve with best fit
        while d[pi] > 0 {
            debug_assert!(item_sum(&d) as i32 == sd);
            // Find next negative count, return false if all counts are non-neg
            while d[ni] >= 0 {
                ni += 1;
                if ni >= l {
                    assert!(sd > 0);
                    return false;
                }
            }
            debug_assert!(ni > pi);
            // How many of pi pieces fit into a ni slot
            let moved = min(d[pi], (ni as i32) / (pi as i32));
            debug_assert!(moved > 0);
            // Remaining size of the ni slot
            let rem = (ni as i32) - moved * (pi as i32);
            d[ni] += 1;
            d[pi] -= moved;
            debug_assert!(rem != pi as i32);
            if rem > pi as i32 {
                // Remainder is larger than pi, remember it
                d[rem as usize] -= 1;
                ni = min(rem as usize, ni);
            } else {
                // Remainder is smaller than pi, forget it and remove it from sd
                sd += rem;
            }
        }
    }
    assert!(sd == 0);
    return true;
}

/// Each element is a set of items (Matej calls them "History"),
/// encoded as counts of each size: the number of items of
/// size `size` is `item_set.0[size]`
#[pyclass]
#[derive(Debug, Clone, Default)]
pub struct ItemsSet(Vec<Vec<C>>);

#[pymethods]
impl ItemsSet {
    #[new]
    pub fn new() -> Self {
        Default::default()
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
        self.0.get(idx).cloned().ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Out of bounds"))
    }

    #[allow(non_snake_case)]
    pub fn any_fits_into_BF(&self, counts: &PyAny) -> PyResult<bool> {
        let cs: Vec<C> = counts.extract()?;
        assert!(cs.len() < C::MAX as usize);
        assert!(cs.len() == 0 || cs[0] == 0);
        Ok(self.0.iter().any(|c| fits_into_BF(c, &cs)))
    }
}

// Init

#[pymodule]
fn binpacs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ItemsSet>()?;
    Ok(())
}

// Misc

pub fn try_into_vec<T, U: TryInto<T> + Clone>(a: &[U]) -> Result<Vec<T>, U::Error> {
    let mut v = vec![];
    for x in a {
        v.push(U::try_into(x.clone())?);
    }
    Ok(v)
}

// Tests

mod test {
    #[allow(unused_imports)]
    use crate::{counts_to_sizes, fits_into_BF, item_sum, sizes_to_counts, C};

    #[allow(non_snake_case, dead_code)]
    fn assert_sizes_fit_BF(sa: &[C], sb: &[C], expect: bool) {
        assert_eq!(
            fits_into_BF(&sizes_to_counts(sa), &sizes_to_counts(sb)),
            expect
        );
    }

    #[test]
    fn test_unit() {
        assert_eq!(counts_to_sizes(&[]), vec![]);
        assert_eq!(counts_to_sizes(&[0, 2, 1]), vec![2, 1, 1]);
        assert_eq!(sizes_to_counts(&[5, 1, 1, 2]), vec![0, 2, 1, 0, 0, 1]);
        assert_eq!(item_sum(&[0]), 0);
        assert_eq!(item_sum(&[0, 3, 1, 0, 0, 1]), 10);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_fits_into_BF() {
        assert_eq!(fits_into_BF(&[], &[]), true);
        assert_eq!(fits_into_BF(&[0, 0, 0, 0, 0, 0], &[0, 0, 0, 0, 0, 0]), true);
        assert_eq!(fits_into_BF(&[0, 0, 0, 1], &[0, 0, 0, 0]), false);
        assert_eq!(fits_into_BF(&[0, 0, 0, 1, 0, 0], &[0, 0, 0, 1, 1, 0]), true);
        assert_eq!(fits_into_BF(&[0, 0, 3], &[0, 0, 0, 2]), false);
        assert_sizes_fit_BF(&[], &[], true);
        assert_sizes_fit_BF(&[2, 2, 2], &[3, 3], false);
        assert_sizes_fit_BF(&[1, 2, 3, 4], &[10], true);
        assert_sizes_fit_BF(&[1, 2, 3, 4], &[11], true);
        assert_sizes_fit_BF(&[1, 2, 3, 4, 5, 6, 7], &[10, 18], true);
        assert_sizes_fit_BF(&[3, 3, 3, 3], &[4, 4, 4], false);
        assert_sizes_fit_BF(&[3, 3, 2, 5], &[6, 7], false); // BF fails even if solvable
        assert_sizes_fit_BF(&[3, 3, 2, 5], &[6, 8], true);
    }
}
