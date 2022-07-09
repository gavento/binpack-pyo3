use crate::packing_bestfit::fits_into_bestfit_internal;
use crate::packing_common::{item_sum, prep_diff};
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
pub fn fits_into_branching(a: &[C], b: &[C], branchings: usize) -> bool {
    let mut d = prep_diff(a, b);
    let sd = item_sum(&d) as i32;
    if branchings <= 1 {
        fits_into_bestfit_internal(&mut d, sd, true)
    } else {
        fits_into_branching_internal(&mut d, sd, branchings)
    }
}

/// Recursive; `d` may be edited in any way.
/// Warning: recurses not just for branchings but also when an unique `(hpos->hneg)` update is found.
pub fn fits_into_branching_internal(d: &mut [i32], sd: i32, branchings: usize) -> bool {
    if branchings <= 1 {
        // trim_upper_bins is already done if at least one branching happened
        return fits_into_bestfit_internal(d, sd, false);
    }

    let mut sd = sd;
    loop {
        // filter lower negatives and check sd <= 0
        let lpos = d.iter().rposition(|x| *x > 0);
        let lneg = d.iter().rposition(|x| *x < 0);
        match (lpos, lneg) {
            (Some(lp), Some(ln)) if lp > ln => {
                sd -= d[ln] * ln as i32;
                d[ln] = 0;
                debug_assert_eq!(item_sum(&d) as i32, sd);
                if sd > 0 {
                    return false;
                }
                // Another loop to check for more low negs
                continue;
            }
            _ => {}
        };

        // find upper non-zero bins
        let hpos = d.iter().rposition(|x| *x > 0);
        let hneg = d.iter().rposition(|x| *x < 0);
        match (hpos, hneg) {
            // Trivially fits, all bins non-positive
            (None, _) => return true,
            // Only non-zero bin is positive, fail
            (Some(_), None) => return false,
            // Largest non-zero bin positive, fail
            (Some(hp), Some(hn)) if hp > hn => return false,
            // Largest non-zero bin negative, carry on
            (Some(hp), Some(hn)) => {
                assert!(hp < hn);
                // collect all negative values between hp and hn (incl.)
                let negs: Vec<usize> = d[hp..]
                    .iter()
                    .enumerate()
                    .filter_map(|(i, x)| if *x < 0 { Some(i + hp) } else { None })
                    .collect();
                debug_assert!(negs.len() > 0);
                debug_assert!(branchings > 1);
                // limit to how many branches we have, lower negs first
                let negs_b = &negs[..min(branchings, negs.len())];
                let part = (branchings + negs_b.len() - 1) / negs_b.len();
                let mut brs = branchings;
                for neg in negs_b {
                    debug_assert!(*neg > hp);
                    let mut d2: Vec<i32> = d.into();
                    d2[*neg] += 1;
                    d2[hp] -= 1;
                    d2[*neg - hp] -= 1;
                    debug_assert_eq!(item_sum(&d2) as i32, sd);
                    if fits_into_branching_internal(&mut d2, sd, min(brs, part)) {
                        return true;
                    }
                    brs -= min(brs, part);
                }
                return false;
            }
        }
    }
}
