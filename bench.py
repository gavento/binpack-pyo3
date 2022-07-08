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
    print(f"Running {msg} ...")
    t0 = time.time()
    yield
    t1 = time.time()
    t = t1 - t0
    if units > 1:
        print(f"  ... {msg} took {t:.6} s for {units} units ({t / units * 1000:.6} ms / unit)")
    else:
        print(f"  ... {msg} took {t:.6} s")

def main():
    K = 40
    S = 200
    NB = 1000
    NT = 1000
    with timed(f"Creating {NB}+{NT} instances"):
        base = gen_items(NB, K, S)
        test = gen_items(NT, K, S)
    with timed(f"Creating ItemsSet"):
        iset = binpack_pyo3.ItemsSet(base)
        print(f"  using estimated {iset.memory_used()} bytes for {NB} items of len {K}, {np.mean(np.sum(base, axis=1))} mean items")
    with timed(f"Checking for {NT} similar items", units=NT*NB):
        tot = 0
        for t in test:
            tot += int(iset.any_fits_into_BF(t))
        print(f"  tests found to contain some base itemsets: {tot} / {NT}")
    with timed(f"Checking for {NT} similar items (rayon parallel)", units=NT*NB):
        tot = 0
        for t in test:
            tot += int(iset.par_any_fits_into_BF(t))
        print(f"  Tests found to contain some base itemsets: {tot} / {NT}")


if __name__ == '__main__':
    main()
