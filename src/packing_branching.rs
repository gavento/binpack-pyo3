use crate::packing_bestfit::fits_into_bestfit_internal;
use crate::packing_common::{item_sum, prep_diff, trim_upper_bins};
use crate::C;
use std::cmp::min;

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
pub fn fits_into_branching(a: &[C], b: &[C], branchings: usize, trim_upper: bool) -> bool {
    let mut d = prep_diff(a, b);
    let sd = item_sum(&d) as i32;
    fits_into_branching_internal(&mut d, sd, 0, 0, branchings, trim_upper)
}

/// Recursive, `d` may be edited in any way.
pub fn fits_into_branching_internal(
    d: &mut [i32],
    sd: i32,
    pi0: usize,
    ni0: usize,
    branchings: usize,
    trim_upper: bool,
) -> bool {
    if branchings == 0 {
        return fits_into_bestfit_internal(d, sd, trim_upper);
    }

    unimplemented!();
    let mut ni = ni0;
    let mut sd = sd;
    for pi in pi0..d.len() {
        debug_assert_eq!(sd, item_sum(d) as i32);
        if trim_upper {
            if !trim_upper_bins(d) {
                return false;
            }
            debug_assert_eq!(sd, item_sum(d) as i32);
        }
        // Check if total volume fits
        if sd > 0 {
            return false;
        }
        // pi points to a negative number -> forget it and remove from the sum
        if d[pi] < 0 {
            sd -= (pi as i32) * d[pi];
            d[pi] = 0;
        }
        // pi points to items in need of packing -> resolve with fit to all highers
        while d[pi] > 0 {
            debug_assert!(item_sum(&d) as i32 == sd);

            // Find next negative count, return false if all counts are non-neg
            while d[ni] >= 0 {
                ni += 1;
                if ni >= d.len() {
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
