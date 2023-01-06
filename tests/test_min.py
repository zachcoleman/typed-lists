import pytest

from typed_lists import TypedList


@pytest.mark.parametrize(
    "list1, expected",
    [
        (TypedList([1, 2, 3]), 1),
        (TypedList([1, 1, 0]), 0),
        (TypedList("abc"), "a"),
    ],
)
def test_min(list1, expected):
    assert list1.min() == expected


@pytest.mark.parametrize(
    "list1, expected",
    [
        (TypedList([1, 2, 3]), 3),
        (TypedList([1, 1, 0]), 1),
        (TypedList("abc"), "c"),
        (TypedList("acb"), "c"),
    ],
)
def test_max(list1, expected):
    assert list1.max() == expected


@pytest.mark.parametrize(
    "list1, expected",
    [
        (TypedList([1, 2, 3]), 0),
        (TypedList("abcd"), 0),
        (TypedList("aaaaa"), 0),
        (TypedList("baaaa"), 1),
    ],
)
def test_argmin(list1, expected):
    assert list1.argmin() == expected


@pytest.mark.parametrize(
    "list1, expected",
    [
        (TypedList([1, 2, 3]), 2),
        (TypedList("abdc"), 2),
        (TypedList("aaaaa"), 0),
        (TypedList("aabb"), 2),
    ],
)
def test_argmax(list1, expected):
    assert list1.argmax() == expected
