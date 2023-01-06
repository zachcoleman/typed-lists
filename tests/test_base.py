import pytest

from typed_lists import TypedList
from typed_lists._typed_lists_ext import (
    BoolTypedList,
    FloatTypedList,
    IntTypedList,
    StringTypedList,
)


@pytest.mark.parametrize(
    "list1, expected",
    [
        (TypedList(range(10)), IntTypedList),
        (TypedList([1.0]), FloatTypedList),
        (TypedList("abc"), StringTypedList),
        (TypedList((x % 2 == 0 for x in range(4))), BoolTypedList),
        (TypedList(int), IntTypedList),
        (TypedList(float), FloatTypedList),
        (TypedList(str), StringTypedList),
        (TypedList(bool), BoolTypedList),
    ],
)
def test_init_type(list1, expected):
    assert isinstance(list1, expected)


@pytest.mark.parametrize(
    "list1, expected",
    [
        (TypedList(range(10)), list(range(10))),
        (TypedList([1.0]), [1.0]),
        (TypedList("abc"), list("abc")),
        (TypedList((x % 2 == 0 for x in range(4))), [True, False, True, False]),
        (TypedList(int), []),
        (TypedList(float), []),
        (TypedList(str), []),
        (TypedList(bool), []),
    ],
)
def test_data_attr(list1, expected):
    assert list1.data == expected


@pytest.mark.parametrize(
    "list1, value, expected",
    [
        (TypedList([], int), 1, [1]),
        (TypedList([1.0]), 2.0, [1.0, 2.0]),
    ],
)
def test_append(list1, value, expected):
    list1.append(value)
    assert list1.data == expected


@pytest.mark.parametrize(
    "list1, list2, expected",
    [
        (TypedList(range(3)), TypedList(range(3, 6)), TypedList(range(6))),
    ],
)
def test_extend(list1, list2, expected):
    list1.extend(list2)
    assert list1 == expected


@pytest.mark.parametrize(
    "list1, expected",
    [
        (TypedList(range(10)), 10),
        (TypedList([1.0]), 1),
        (TypedList("abc"), 3),
    ],
)
def test_len(list1, expected):
    assert len(list1) == expected
