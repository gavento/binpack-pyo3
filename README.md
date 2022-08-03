# Bin packing storage and algorithms

![GitHub release (latest by date)](https://img.shields.io/github/v/release/gavento/binpack-pyo3)
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/gavento/binpack-pyo3/CI)

Written in Rust with a Python interface through [PyO3](https://pyo3.rs/).
Python wheels available in [releases](https://github.com/gavento/binpack-pyo3/releases).

Created by Tomáš Gavenčiak for a project of Matej Lieskovský.

## Item set storage

`ItemSets()` is a container for a collections of set of items to be bin-packed.
The motivations for implementing this structure in Rust are storege efficiency
(only having one Python object per collection, rather per item set or per item),
and for fast comparison between a given item set and the entire collection (saving
Python->Rust calls and allowing [Rayon](https://docs.rs/rayon/latest/rayon/) parallelism
of queries).

The item sets are internally represented
as an array *counts* of items of every size: `c[i]` is the number of items of size `i`, `c[0]=0` as a convention.
Use `ItemSets.push_counts()`, `ItemSets[i]` and `ItemSets.all_counts()` to add item set, read i-th item set or get
all item sets (all represented as counts). Removing of sets is not supported now (but could be easily added).

Alternatively, the items can be also inserted and read as list of item *sizes* (with repetitions and in any order) using
`ItemSets.push_sizes()` and `ItemSets.all_counts()`.

The type `C` of the counts is fixed during compilation, currently it is `u8` (internal computations use `i32`).
The number of memory bytes used by the structure can be checked with `ItemSet.memory_used()`.

## Bin packing algorithms

Implements two algorithms. In both of them, we take the difference of the count vectors (packing items
into exactly fitting bins is optimal WLOG) and try to eliminate all positive counts by packing them into larger negative counts.
Both algorithms also eliminate negative counts smaller than any positive count, as tohse may never be used again.
If the overall sum of (item size)*(count) gets positive, we fail immediately.

* **Best fit** goes from smallest positive counts and fits them into smallest fitting negative count, breaing it down into a smaller negative count. This best fit is also prepended by checking the largest positive count and if only one negative count is larger, fitting those two (necessary), repeated until this is not possible. (In benchmarking, this actually showed a difference of matches found.)

* **Branching** is an approximation of exhaustive search with a given bound on the number of branchings. The algorithm
goes from largest positive count and branches on the negative counts larger than it, recursing on the subproblems.
The branching budget is then divided euqally among the branches. Whenever the branching budget is depleted to 1 or 0, that branch
switches to best fit algorithm above. Large enough values of branching (e.g. `usize::MAX`) should amount to exhaustive search in practice
but that is not implemented separately (could be easily, though).

With `par=True`, the bin packings against the collection of stored item sets are ran in parallel. No parallelism happens inside the 1-into-1 bin packing algorithm itself.
Parallelism is by default using as many threads as the number of cores (Rayon default), which may be inefficient; set a limit with e.g. env var `RAYON_NUM_THREADS=4`.

## Building

First of all, you can skip building and just install a released wheel with `pip install https://github.com/gavento/binpack-pyo3/releases/download/v0.3.2/binpack_pyo3-0.3.2-cp39-cp39-manylinux_2_5_x86_64.manylinux1_x86_64.whl` - update the URL with the right python version from the [latest release](https://github.com/gavento/binpack-pyo3/releases/latest)).

Test with `cargo test`. Python package builds are using [maturin](https://maturin.rs/): `maturin develop`
creates debug builds and install them (virtual env recommended), `maturin buid` builds a release package.
The release builds are created in [Github CI](https://github.com/gavento/binpack-pyo3/blob/main/.github/workflows/CI.yml).

## Example usage

```shell
pip install https://github.com/gavento/binpack-pyo3/releases/download/v0.3.2/binpack_pyo3-0.3.2-cp39-cp39-manylinux_2_5_x86_64.manylinux1_x86_64.whl
```

```python
import binpack_pyo3
s = binpack_pyo3.ItemSets() # Empty collection
s.push_sizes([3,3,3,4]) # Insert as list of item sizes
s.push_counts([0,1,0,0,3]) # Insert as vector of counts of items of every size (from 0), here items [4,4,4,1]
assert s.s2c([4,4,4,1]) == [0,1,0,0,3] # Quick conversion counts<->sizes
print(s.all_counts()) # List all as vectors of counts
print(s.memory_used()) # Estimate bytes used
assert s.any_fit_into_given([0,0,0,0,0,1,0,0,1]) == True # 2nd fits into [5,8]
assert s.all_fit_into_given([0,0,0,0,0,1,0,0,1]) == False # 1st does not fit into [5,8]
assert s.given_fits_into_how_many(s.s2c([2,3,3,3,1])) == 1 # Counting, with conversion from size set to count vector
```

## Benchmark on AMD EPYC 7302

Running with `RAYON_NUM_THREADS=8 python3.9 bench.py`:

```
Running creating 10000+1000 instances ...
  ... creating 10000+1000 instances took 0.358989 s

Running creating ItemsSet ...
  stored item sets have on average 19.3521 items
  using estimated 793240 bytes for 10000 items of len 40
  ... creating ItemsSet took 0.04649 s

## Any item in ItemSet fits into a given item - best fit only (baseline)

Running bestfit_any_fit_into_given(trim_upper=False) ...
  matching test item sets: 875 out of 1000
  ... bestfit_any_fit_into_given(trim_upper=False) took 0.362071 s for 10000000 pairs (0.0362071 us / pair)

## Any item in ItemSet fits into a given item

Running any_fit_into_given ...
  matching test item sets: 894 out of 1000
  ... any_fit_into_given took 0.406453 s for 10000000 pairs (0.0406453 us / pair)

Running any_fit_into_given(par=True) ...
  matching test item sets: 894 out of 1000
  ... any_fit_into_given(par=True) took 0.0705073 s for 10000000 pairs (0.00705073 us / pair)

Running any_fit_into_given(branchings=10) ...
  matching test item sets: 961 out of 1000
  ... any_fit_into_given(branchings=10) took 0.313754 s for 10000000 pairs (0.0313754 us / pair)

Running any_fit_into_given(branchings=10, par=True) ...
  matching test item sets: 961 out of 1000
  ... any_fit_into_given(branchings=10, par=True) took 0.0589352 s for 10000000 pairs (0.00589352 us / pair)

Running any_fit_into_given(branchings=100) ...
  matching test item sets: 965 out of 1000
  ... any_fit_into_given(branchings=100) took 0.458785 s for 10000000 pairs (0.0458785 us / pair)

Running any_fit_into_given(branchings=10000) ...
  matching test item sets: 965 out of 1000
  ... any_fit_into_given(branchings=10000) took 1.02657 s for 10000000 pairs (0.102657 us / pair)

## Other single match (and single-mismatch) finding functions

Running all_fit_into_given(par=True) ...
  matching test item sets: 0 out of 1000
  ... all_fit_into_given(par=True) took 0.0171547 s for 10000000 pairs (0.00171547 us / pair)

Running given_fits_into_any(par=True) ...
  matching test item sets: 635 out of 1000
  ... given_fits_into_any(par=True) took 0.137894 s for 10000000 pairs (0.0137894 us / pair)

Running given_fits_into_all(par=True) ...
  matching test item sets: 0 out of 1000
  ... given_fits_into_all(par=True) took 0.0146987 s for 10000000 pairs (0.00146987 us / pair)

## Counting all matching pairs

Running given_fits_into_how_many ...
  total matching pairs found in item sets: 41689 out of 10000000
  ... given_fits_into_how_many took 1.80745 s for 10000000 pairs (0.180745 us / pair)

Running given_fits_into_how_many(par=True) ...
  total matching pairs found in item sets: 41689 out of 10000000
  ... given_fits_into_how_many(par=True) took 0.324619 s for 10000000 pairs (0.0324619 us / pair)

Running given_fits_into_how_many(branchings=10) ...
  total matching pairs found in item sets: 251237 out of 10000000
  ... given_fits_into_how_many(branchings=10) took 5.61113 s for 10000000 pairs (0.561113 us / pair)

Running given_fits_into_how_many(branchings=100) ...
  total matching pairs found in item sets: 334163 out of 10000000
  ... given_fits_into_how_many(branchings=100) took 15.4004 s for 10000000 pairs (1.54004 us / pair)

Running given_fits_into_how_many(branchings=100, par=True) ...
  total matching pairs found in item sets: 334163 out of 10000000
  ... given_fits_into_how_many(branchings=100, par=True) took 2.08171 s for 10000000 pairs (0.208171 us / pair)

Running given_fits_into_how_many(branchings=1000, par=True) ...
  total matching pairs found in item sets: 344709 out of 10000000
  ... given_fits_into_how_many(branchings=1000, par=True) took 5.19272 s for 10000000 pairs (0.519272 us / pair)

Running given_fits_into_how_many(branchings=10000, par=True) ...
  total matching pairs found in item sets: 345378 out of 10000000
  ... given_fits_into_how_many(branchings=10000, par=True) took 11.9784 s for 10000000 pairs (1.19784 us / pair)
```
