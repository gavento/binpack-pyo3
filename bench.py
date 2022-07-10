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
            print(f"  matching test item sets: {tot} out of {NT}")

    tst("any_fit_into_given")
    tst("any_fit_into_given(par=True)", par=True)
    tst("any_fit_into_given(branchings=10)", branching=10)
    tst("any_fit_into_given(branchings=100)", branching=100)
    tst("any_fit_into_given(branchings=1000000)", branching=1000000)
    tst("any_fit_into_given(branchings=1000000, par=True)", branching=1000000, par=True)

    tst("all_fit_into_given(par=True)", fname="all_fit_into_given", par=True)
    tst("given_fits_into_any(par=True)", fname="given_fits_into_any", par=True)
    tst("given_fits_into_all(par=True)", fname="given_fits_into_all", par=True)

if __name__ == '__main__':
    main()
