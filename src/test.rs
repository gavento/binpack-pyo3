use crate::packing_bestfit::fits_into_bestfit;
use crate::packing_branching::fits_into_branching;
#[allow(unused_imports)]
use crate::packing_common::{counts_to_sizes, item_sum, sizes_to_counts, trim_upper_bins};
use crate::C;

#[allow(non_snake_case, dead_code)]
fn assert_sizes_fit_BF(sa: &[C], sb: &[C], expect: bool) {
    assert_eq!(
        fits_into_bestfit(&sizes_to_counts(sa), &sizes_to_counts(sb), false),
        expect
    );
}

#[allow(non_snake_case, dead_code)]
fn assert_sizes_fit_E(sa: &[C], sb: &[C], branchings: usize, expect: bool) {
    assert_eq!(
        fits_into_branching(&sizes_to_counts(sa), &sizes_to_counts(sb), branchings),
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
fn test_trim_upper_bins() {
    let mut d = vec![0i32, 0, 0, 0, 0, 0];
    assert_eq!(trim_upper_bins(&mut d), true);
    assert_eq!(&d, &[0, 0, 0, 0, 0, 0]);

    let mut d = vec![0i32, 0, 0, -1, 0, 1];
    assert_eq!(trim_upper_bins(&mut d), false);

    let mut d = vec![0i32, 0, 0, 0, 0, 1];
    assert_eq!(trim_upper_bins(&mut d), false);

    let mut d = vec![0i32, 0, 0, 1, 1, -1];
    assert_eq!(trim_upper_bins(&mut d), false);

    let mut d = vec![0i32, 0, 0, 7, 6, -1, -1];
    assert_eq!(trim_upper_bins(&mut d), true);

    let mut d = vec![0i32, 1, 0, 1, 1, 0, -2];
    assert_eq!(trim_upper_bins(&mut d), true);
    assert_eq!(&d, &[0, 1, -1, -1, 0, 0, 0]);

    let mut d = vec![0i32, 0, 0, 1, 0, 0, -1];
    assert_eq!(trim_upper_bins(&mut d), true);
    assert_eq!(&d, &[0, 0, 0, -1, 0, 0, 0]);
}

#[test]
fn test_fits_into_bestfit() {
    // Direct bestfit
    assert_eq!(fits_into_bestfit(&[], &[], false), true);
    assert_eq!(
        fits_into_bestfit(&[0, 0, 0, 0, 0, 0], &[0, 0, 0, 0, 0, 0], false),
        true
    );
    assert_eq!(
        fits_into_bestfit(&[0, 0, 0, 1], &[0, 0, 0, 0], false),
        false
    );
    assert_eq!(
        fits_into_bestfit(&[0, 0, 0, 1, 0, 0], &[0, 0, 0, 1, 1, 0], false),
        true
    );
    assert_eq!(fits_into_bestfit(&[0, 0, 3], &[0, 0, 0, 2], false), false);
    assert_eq!(fits_into_bestfit(&[0, 1, 0, 1], &[0, 0, 1, 1], false), true);

    // From sizes, no trim_upper
    assert_sizes_fit_BF(&[], &[], true);
    assert_sizes_fit_BF(&[2, 2, 2], &[3, 3], false);
    assert_sizes_fit_BF(&[1, 2, 3, 4], &[10], true);
    assert_sizes_fit_BF(&[1, 2, 3, 4], &[11], true);
    assert_sizes_fit_BF(&[1, 2, 3, 4, 5, 6, 7], &[10, 18], true);
    assert_sizes_fit_BF(&[3, 3, 3, 3], &[4, 4, 4], false);
    assert_sizes_fit_BF(&[3, 3, 2, 5], &[6, 7], false); // BF fails even if solvable
    assert_sizes_fit_BF(&[3, 3, 2, 5], &[6, 8], true);
}

#[test]
fn test_fits_into_branching() {
    assert_sizes_fit_E(&[], &[], 1000, true);
    assert_sizes_fit_E(&[2, 2, 2], &[3, 3], 2, false);
    assert_sizes_fit_E(&[1, 2, 3, 4], &[10], 2, true);
    assert_sizes_fit_E(&[1, 2, 3, 4], &[11], 2, true);
    assert_sizes_fit_E(&[1, 2, 3, 4, 5, 6, 7], &[10, 18], 2, true);
    assert_sizes_fit_E(&[1, 2, 3, 4, 5, 6, 7], &[10, 18], 1, true); // BF called
    assert_sizes_fit_E(&[3, 3, 3, 3], &[4, 4, 4], 2, false);
    assert_sizes_fit_E(&[3, 3, 2, 5], &[6, 7], 1, false); // BF fails
    assert_sizes_fit_E(&[3, 3, 2, 5], &[6, 7], 2, true); // BF would fail
    assert_sizes_fit_E(&[3, 3, 2, 5], &[6, 8], 2, true);
    assert_sizes_fit_E(&[1,2,3,4,5,6,7,8,9,10], &[17,14,11,13], 2, false); // BF would fail, 2 branches are not enough
    assert_sizes_fit_E(&[1,2,3,4,5,6,7,8,9,10], &[17,14,11,13], 10, true); // BF would fail
}