use crate::C;
use std::cmp::max;
use std::cmp::min;

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

/// Create a difference vector of a-b
pub fn prep_diff(a: &[C], b: &[C]) -> Vec<i32> {
    let mut d: Vec<i32> = vec![0; max(a.len(), b.len())];
    for i in 0..d.len() {
        d[i] += *a.get(i).unwrap_or(&0) as i32;
        d[i] -= *b.get(i).unwrap_or(&0) as i32;
    }
    d
}

/// Edit `d` and `sd` to reflect necessary matchings in upper bins,
/// returns whether matching may still be possible.
///
/// Fails if upper non-zero bin is positive (can't fit anywhere).
/// If top two bins are (neg, pos), cancels the positive bin with the negative one.
/// Does not change the `item_sum` of `d`.
pub fn trim_upper_bins(d: &mut [i32]) -> bool {
    let mut ni: usize = 0;
    trim_upper_bins_ni(d, &mut ni)
}

/// Same as `trim_upper_bins` but updates `ni` to point to the smallest-index neg value
pub fn trim_upper_bins_ni(d: &mut [i32], ni: &mut usize) -> bool {
    if d.len() == 0 {
        return true;
    }
    loop {
        // NOTE: would be a bit more effcient if remembered bounds for hpos, hneg, but meh
        let hpos = d.iter().rposition(|x| *x > 0);
        let hneg = d.iter().rposition(|x| *x < 0);
        match (hpos, hneg) {
            // Trivially fits
            (None, _) => return true,
            // Only non-zero bin positive
            (Some(_), None) => return false,
            // Largest non-zero bin positive
            (Some(hp), Some(hn)) if hp > hn => return false,
            // Largest non-zero bin negative
            (Some(hp), Some(hn)) => {
                // Is there any other negative value between hp and hn?
                if d[hp..hn].iter().any(|x| *x < 0) {
                    // Yes -> unknown how to resolve hp
                    return true;
                }
                // Put one hp into hn, (hn-hp) remains
                d[hn] += 1;
                d[hp] -= 1;
                d[hn - hp] -= 1;
                *ni = min(*ni, hn - hp);
            }
        }
    }
}
