import contextlib
import time

import binpack_pyo3
import numpy as np


def gen_items(n, k, sum, min_of=3):
    res = []
    for _ in range(n):
        s = np.random.randint(1, k, size=(min_of, sum))
        s = s.min(axis=0)
        assert s.shape == (sum,)
        ss = np.cumsum(s)
        si = np.argmax(ss > sum)
        r = [0] * k
        for i in s[:si]:
            r[i] += 1
        res.append(r)
    return res

@contextlib.contextmanager
def timed(msg, units=1):
    print(f"\nRunning {msg} ...")
    t0 = time.time()
    yield
    t1 = time.time()
    t = t1 - t0
    if units > 1:
        print(f"  ... {msg} took {t:.6} s for {units} pairs ({t / units * 1000000:.6} us / pair)")
    else:
        print(f"  ... {msg} took {t:.6} s")

def main():
    K = 40
    S = 200
    NB = 10000
    NT = 1000
    Rep = 1

    np.random.seed(1)
    with timed(f"creating {NB}+{NT} instances"):
        base = gen_items(NB, K, S)
        test = gen_items(NT, K, S)

    with timed(f"creating ItemsSet"):
        iset = binpack_pyo3.ItemSets(base)
        print(f"  stored item sets have on average {np.mean(np.sum(base, axis=1))} items")
        print(f"  using estimated {iset.memory_used()} bytes for {NB} items of len {K}")

    def tst(msg, fname="any_fit_into_given", **kwargs):
        with timed(f"{msg}", units=NT*NB*Rep):
            for _i in range(Rep):
                tot = 0
                f = getattr(iset, fname)
                for t in test:
                    tot += int(f(t, **kwargs))
            if "how_many" in msg:
                print(f"  total matching pairs found in item sets: {tot} out of {NT*NB}")
            else:
                print(f"  matching test item sets: {tot} out of {NT}")

    print("\n## Any item in ItemSet fits into a given item - best fit only (baseline)")

    tst("bestfit_any_fit_into_given(trim_upper=False)", fname="bestfit_any_fit_into_given", trim_upper=False)

    print("\n## Any item in ItemSet fits into a given item")

    tst("any_fit_into_given")
    tst("any_fit_into_given(par=True)", par=True)
    tst("any_fit_into_given(branchings=10)", branching=10)
    tst("any_fit_into_given(branchings=10, par=True)", branching=10, par=True)
    tst("any_fit_into_given(branchings=100)", branching=100)
    tst("any_fit_into_given(branchings=1000000)", branching=1000000)

    print("\n## Other single match (and single-mismatch) finding functions")

    tst("all_fit_into_given(par=True)", fname="all_fit_into_given", par=True)
    tst("given_fits_into_any(par=True)", fname="given_fits_into_any", par=True)
    tst("given_fits_into_all(par=True)", fname="given_fits_into_all", par=True)

    print("\n## Counting all matching pairs")

    tst("given_fits_into_how_many", fname="given_fits_into_how_many")
    tst("given_fits_into_how_many(par=True)", fname="given_fits_into_how_many", par=True)
    tst("given_fits_into_how_many(branchings=10)", fname="given_fits_into_how_many", branching=10)
    tst("given_fits_into_how_many(branchings=10, par=True)", fname="given_fits_into_how_many", par=True, branching=10)
    tst("given_fits_into_how_many(branchings=100)", fname="given_fits_into_how_many", branching=100)
    tst("given_fits_into_how_many(branchings=1000)", fname="given_fits_into_how_many", branching=1000)
    tst("given_fits_into_how_many(branchings=10000)", fname="given_fits_into_how_many", branching=10000)

if __name__ == '__main__':
    main()
