import pytest

from typed_lists import TypedList


@pytest.mark.parametrize(
    "list1, expected",
    [
        (TypedList([3, 2, 1]), TypedList([1, 2, 3])),
        (TypedList("acbdfe"), TypedList("abcdef")),
    ],
)
def test_sort_inplace(list1, expected):
    list1.sort_inplace()
    assert (list1 == expected).all()


@pytest.mark.parametrize(
    "list1, expected",
    [
        (TypedList([3, 2, 1]), TypedList([1, 2, 3])),
        (TypedList("acbdfe"), TypedList("abcdef")),
    ],
)
def test_sort(list1, expected):
    assert (list1.sort() == expected).all()


@pytest.mark.parametrize(
    "list1, expected",
    [
        (TypedList([3, 2, 1]), TypedList([2, 1, 0])),
        (TypedList("abc"), TypedList([0, 1, 2])),
        (TypedList([0.2, 0.1, 0.3]), TypedList([1, 0, 2])),
    ],
)
def test_argsort(list1, expected):
    assert (list1.argsort() == expected).all()
