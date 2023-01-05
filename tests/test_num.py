import pytest

from typed_lists import TypedList
from typed_lists._typed_lists_ext import FloatTypedList, IntTypedList, StringTypedList


@pytest.mark.parametrize(
    "list1, list2, op, expected",
    [
        (
            TypedList(range(10)),
            TypedList(range(10)),
            IntTypedList.__eq__,
            TypedList(range(0, 20, 2)),
        ),
        (TypedList(range(10)), 1, IntTypedList.__eq__, TypedList(range(1, 11))),
        (
            TypedList(["a", "b", "c"]),
            TypedList(["a", "b", "c"]),
            StringTypedList.__eq__,
            TypedList(["aa", "bb", "cc"]),
        ),
        (
            TypedList([1.0]),
            TypedList([1.0]),
            FloatTypedList.__lt__,
            TypedList([3.0]),
        ),
    ],
)
def test_add(list1, op, list2, expected):
    assert op(list1 + list2, expected)


@pytest.mark.parametrize(
    "list1, list2",
    [
        (
            TypedList(range(10)),
            TypedList(range(20)),
        ),
        (
            TypedList([True]),
            TypedList([False]),
        ),
        (
            TypedList([1.0]),
            "a",
        ),
        (TypedList(range(10)), 1.0),
    ],
)
def test_add_error(list1, list2):
    with pytest.raises(Exception):
        _ = list1 + list2


@pytest.mark.parametrize(
    "list1, list2, op, expected",
    [
        (
            TypedList(range(10)),
            TypedList(range(10)),
            IntTypedList.__eq__,
            TypedList([0 for _ in range(10)]),
        ),
        (
            TypedList([1.0, 2.0, 3.0]),
            TypedList([1.0, 2.0, 3.0]),
            FloatTypedList.__lt__,
            TypedList([0.1, 0.1, 0.1]),
        ),
        (
            TypedList([1.0, 2.0, 3.0]),
            TypedList([1.0, 2.0, 3.0]),
            FloatTypedList.__gt__,
            TypedList([0.1, 0.1, 0.1]) * -1,
        ),
    ],
)
def test_sub(list1, op, list2, expected):
    assert op(list1 - list2, expected)


@pytest.mark.parametrize(
    "list1, list2",
    [
        (
            TypedList([True]),
            TypedList([False]),
        ),
        (
            TypedList(["a"]),
            TypedList(["b"]),
        ),
    ],
)
def test_sub_error(list1, list2):
    with pytest.raises(Exception):
        _ = list1 - list2


@pytest.mark.parametrize(
    "list1, list2, op, expected",
    [
        (
            TypedList(range(3)),
            TypedList(range(3)),
            IntTypedList.__eq__,
            TypedList([0, 1, 4]),
        ),
        (
            TypedList(range(10)),
            2,
            IntTypedList.__eq__,
            TypedList(range(0, 20, 2)),
        ),
        (
            TypedList([1.0, 2.0, 3.0]),
            2.0,
            FloatTypedList.__lt__,
            TypedList([2.1, 4.1, 6.1]),
        ),
    ],
)
def test_mul(list1, op, list2, expected):
    assert op(list1 * list2, expected)


@pytest.mark.parametrize(
    "list1, list2",
    [(TypedList(range(10)), "a"), (TypedList([1.0]), TypedList(range(1)))],
)
def test_mul_error(list1, list2):
    with pytest.raises(Exception):
        _ = list1 * list2
