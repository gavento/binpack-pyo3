use pyo3::prelude::*;

pub type C = u8;

mod items_set;
pub use items_set::ItemsSet;

mod packing;
pub use packing::{counts_to_sizes, fits_into_BF, item_sum, sizes_to_counts};

//mod threadpool;
//pub use threadpool::RayonThreadPool;

// Init

#[pymodule]
fn binpacs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ItemsSet>()?;
    Ok(())
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
