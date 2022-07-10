# Bin packing storage and packing heuristics for Python

Written in Rust with a Python interface through PyO3.
Python wheels available in [releases](https://github.com/gavento/binpack-pyo3/releases).

## Item set storage

`ItemSets()` is a container for a collections of set of items to be packed. The item sets are internally representex
as an array *counts* of items of every size: `c[i]` is the number of items of size `i`, `c[0]=0` as a convention.
Use `ItemSets.push_counts()`, `ItemSets[i]` and `ItemSets.all_counts()` to add item set, read i-th item set or get
all item sets (all represented as counts). Removing of sets is not supported now (but could be easily added).

Alternatively, the items can be also inserted and read as list of item *sizes* (with repetitions and in any order) using
`ItemSets.push_sizes()` and `ItemSets.all_counts()`.

The type `C` of the counts is fixed during compilation, currently it is `u8` (internal computations use `i32`).
The number of memory bytes used by the structure can be checked with `ItemSet.memory_used()`.


