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

With `par=True`, the bin packings against the collection of stored item sets are ran in parallel. No parallelism happens inside the
bin packing algorithms.
Parallelism may benefit from using fewer threads than the number of cores (Rayon default), set with env var `RAYON_NUM_THREADS=4`.

## Building

First of all, you can skip building and just install a released wheel with `pip install https://github.com/gavento/binpack-pyo3/releases/download/v0.3.1/binpack_pyo3-0.3.1-cp39-cp39-manylinux_2_5_x86_64.manylinux1_x86_64.whl` - update the URL with the right python version from the [latest release](https://github.com/gavento/binpack-pyo3/releases/latest)).

Test with `cargo test`. Python package builds are using [maturin](https://maturin.rs/): `maturin develop`
creates debug builds and install them (virtual env recommended), `maturin buid` builds a release package.
The release builds are created in [Github CI](https://github.com/gavento/binpack-pyo3/blob/main/.github/workflows/CI.yml).

## Benchmark on AMD EPYC 7302

Running with `RAYON_NUM_THREADS=4 python3.9 bench.py`:

```
Running creating ItemsSet ...
  stored item sets have on average 19.3521 items
  using estimated 793240 bytes for 10000 items of len 40
  ... creating ItemsSet took 0.0457718 s

Running bestfit_any_fit_into_given(trim_upper=False) ...
  matching test item sets: 875 out of 1000
  ... bestfit_any_fit_into_given(trim_upper=False) took 0.394095 s for 10000000 pairs (0.0394095 us / pair)

Running any_fit_into_given ...
  matching test item sets: 894 out of 1000
  ... any_fit_into_given took 0.412942 s for 10000000 pairs (0.0412942 us / pair)

Running any_fit_into_given(par=True) ...
  matching test item sets: 894 out of 1000
  ... any_fit_into_given(par=True) took 0.124982 s for 10000000 pairs (0.0124982 us / pair)

Running any_fit_into_given(branchings=10) ...
  matching test item sets: 961 out of 1000
  ... any_fit_into_given(branchings=10) took 0.328511 s for 10000000 pairs (0.0328511 us / pair)

Running any_fit_into_given(branchings=10, par=True) ...
  matching test item sets: 961 out of 1000
  ... any_fit_into_given(branchings=10, par=True) took 0.103553 s for 10000000 pairs (0.0103553 us / pair)

Running any_fit_into_given(branchings=100) ...
  matching test item sets: 965 out of 1000
  ... any_fit_into_given(branchings=100) took 0.476524 s for 10000000 pairs (0.0476524 us / pair)

Running any_fit_into_given(branchings=1000000) ...
  matching test item sets: 965 out of 1000
  ... any_fit_into_given(branchings=1000000) took 1.65671 s for 10000000 pairs (0.165671 us / pair)
```
