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
        print(f"  ... {msg} took {t:.6} s for {units} units ({t / units * 1000000:.6} us / unit)")
    else:
        print(f"  ... {msg} took {t:.6} s")

def main():
    K = 40
    S = 200
    NB = 1000
    NT = 100
    Rep = 1

    with timed(f"creating {NB}+{NT} instances"):
        base = gen_items(NB, K, S)
        test = gen_items(NT, K, S)

    with timed(f"creating ItemsSet"):
        iset = binpack_pyo3.ItemSets(base)
        print(f"  using estimated {iset.memory_used()} bytes for {NB} items of len {K}, {np.mean(np.sum(base, axis=1))} mean items")

    def tst(msg, **kwargs):
        with timed(f"{Rep}x {msg}", units=NT*NB*Rep):
            for _i in range(Rep):
                tot = 0
                for t in test:
                    tot += int(iset.any_fits_into_counts(t, **kwargs))
            print(f"  tests found to contain any base itemset: {tot} out of {NT}")

    tst("looking for base items fitting into test sets")
    tst("looking for base items fitting into test sets (trim_upper=False)", trim_upper=False)
    tst("looking for base items fitting into test sets (branchings=10)", branchings=10)
    tst("looking for base items fitting into test sets (branchings=100)", branchings=100)
    tst("looking for base items fitting into test sets (par=True)", par=True)

if __name__ == '__main__':
    main()
