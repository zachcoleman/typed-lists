import pytest

from typed_lists import TypedList


@pytest.mark.parametrize(
    "list1, list2, expected",
    [
        (TypedList([1, 2, 3]), TypedList([1, 2, 3]), TypedList([False, False, False])),
        (TypedList("abc"), TypedList("def"), TypedList([True, True, True])),
        (TypedList([1.0, 2.0, 3.0]), 0.0, TypedList([False, False, False])),
    ],
)
def test_lt(list1, list2, expected):
    assert ((list1 < list2) == expected).all()


@pytest.mark.parametrize(
    "list1, list2, expected",
    [
        (TypedList([2, 3, 4]), TypedList([1, 2, 3]), TypedList([True, True, True])),
        (TypedList("abd"), TypedList("dec"), TypedList([False, False, True])),
        (TypedList([1.0, 2.0, 3.0]), 0.0, TypedList([True, True, True])),
        (
            TypedList([1.0, 2.0, 3.0]),
            TypedList([0.0, 0.0, 0.0]),
            TypedList([True, True, True]),
        ),
    ],
)
def test_gt(list1, list2, expected):
    assert ((list1 > list2) == expected).all()


@pytest.mark.parametrize(
    "list1, list2, expected",
    [
        (
            TypedList(range(10)),
            TypedList(range(10)),
            TypedList([True for _ in range(10)]),
        ),
        (TypedList([1]), 0, TypedList([False])),
        (TypedList("abc"), "a", TypedList([True, False, False])),
    ],
)
def test_le(list1, list2, expected):
    assert ((list1 <= list2) == expected).all()


@pytest.mark.parametrize(
    "list1, list2, expected",
    [
        (
            TypedList(range(10)),
            TypedList(range(10)),
            TypedList([True for _ in range(10)]),
        ),
        (TypedList([1]), 0, TypedList([True])),
        (TypedList("abc"), "a", TypedList([True, True, True])),
    ],
)
def test_ge(list1, list2, expected):
    assert ((list1 >= list2) == expected).all()


@pytest.mark.parametrize(
    "list1, list2, expected",
    [
        (
            TypedList(range(10)),
            TypedList(range(10)),
            TypedList([True for _ in range(10)]),
        ),
        (TypedList("abc"), TypedList("abd"), TypedList([True, True, False])),
    ],
)
def test_eq(list1, list2, expected):
    assert ((list1 == list2) == expected).all()


@pytest.mark.parametrize(
    "list1, list2, expected",
    [
        (
            TypedList(range(10)),
            TypedList(range(10)),
            TypedList([False for _ in range(10)]),
        ),
        (TypedList("abc"), TypedList("abd"), TypedList([False, False, True])),
    ],
)
def test_ne(list1, list2, expected):
    assert ((list1 != list2) == expected).all()
