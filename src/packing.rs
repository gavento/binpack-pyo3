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
