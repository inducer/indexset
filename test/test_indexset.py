from indexset import IndexSet, FrozenIndexSet


def test_basics():
    s = IndexSet()
    for i in [0, 0, 0, 0, 1]:
        s.add(i)
    assert len(s) == 2

    assert list(s) == [0, 1]
