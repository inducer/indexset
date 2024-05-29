from time import process_time_ns

from indexset import IndexSet
n = 10_000_000

def test_create(tp):
    start = process_time_ns()
    for i in range(n):
        tp()
    stop = process_time_ns()

    print(f"create {tp}: {(stop-start)/n} ns/round")

def test_add(tp):
    s = tp()
    start = process_time_ns()
    for i in range(n):
        s.add(i)
    stop = process_time_ns()

    print(f"add: {tp}: {(stop-start)/n} ns/round")

if 1:
    for tp in [set, IndexSet]:
        test_create(tp)
        test_add(tp)
else:
    test_add(IndexSet)
