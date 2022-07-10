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



## Building and benchmarking

Test with `cargo test`. Python package builds are using [maturin](https://maturin.rs/): `maturin develop`
creates debug builds and install them (virtual env recommended), `maturin buid` builds a release package.
The release builds are created in [Github CI](https://github.com/gavento/binpack-pyo3/blob/main/.github/workflows/CI.yml).
