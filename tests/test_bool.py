import pytest

from typed_lists import TypedList
from typed_lists._typed_lists_ext import BoolTypedList


@pytest.mark.parametrize(
    "list1, list2, op, expected",
    [
        (
            TypedList([True, True, False, False]),
            TypedList([True, False, True, False]),
            BoolTypedList.__and__,
            TypedList([True, False, False, False]),
        ),
        (
            TypedList([True, True, False, False]),
            TypedList([True, False, True, False]),
            BoolTypedList.__or__,
            TypedList([True, True, True, False]),
        ),
        (
            TypedList([True, True, False, False]),
            TypedList([True, False, True, False]),
            BoolTypedList.__xor__,
            TypedList([False, True, True, False]),
        ),
        (
            TypedList([True]),
            False,
            BoolTypedList.__and__,
            TypedList([False]),
        ),
        (
            TypedList([True]),
            False,
            BoolTypedList.__or__,
            TypedList([True]),
        ),
        (
            TypedList([True]),
            False,
            BoolTypedList.__xor__,
            TypedList([True]),
        ),
    ],
)
def test_bool(list1, list2, op, expected):
    assert op(list1, list2) == expected


@pytest.mark.parametrize(
    "list1, expected",
    [
        (TypedList([True, True, True]), True),
        (TypedList([True, False, True]), False),
    ],
)
def test_bool_all(list1, expected):
    assert list1.all() == expected


@pytest.mark.parametrize(
    "list1, expected",
    [
        (TypedList([True, False, True]), True),
        (TypedList([False, False, False]), False),
        (TypedList([True, True, True]), True),
    ],
)
def test_bool_any(list1, expected):
    assert list1.any() == expected
