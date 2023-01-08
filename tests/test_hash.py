import pytest

from typed_lists import TypedList


@pytest.mark.parametrize(
    "list1, expected",
    [
        (TypedList([1, 2, 3]), TypedList([1, 2, 3])),
        (TypedList([1, 1, 1]), TypedList([1])),
        (TypedList("aaa"), TypedList("a")),
        (TypedList("abc"), TypedList("abc")),
    ],
)
def test_unique(list1, expected):
    assert (list1.unique().sort() == expected.sort()).all()


@pytest.mark.parametrize(
    "list1, expected",
    [
        (TypedList([1, 2, 3]), {1: 1, 2: 1, 3: 1}),
        (TypedList("aaabbc"), {"a": 3, "b": 2, "c": 1}),
        (TypedList([1, 1, 1]), {1: 3}),
    ],
)
def test_count(list1, expected):
    assert list1.count_all() == expected
