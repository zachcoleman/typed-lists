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
    assert (list1 == expected).all()


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


@pytest.mark.parametrize(
    "list1, index, expected",
    (
        (TypedList(range(10)), 0, 0),
        (TypedList(range(10)), None, 9),
        (TypedList("abc"), 1, "b"),
        (TypedList(range(10)), 100, IndexError),
    ),
)
def test_pop(list1, index, expected):
    if isinstance(expected, type) and issubclass(expected, Exception):
        with pytest.raises(expected):
            list1.pop(index)
    else:
        if index is None:
            assert list1.pop() == expected
        else:
            assert list1.pop(index) == expected


@pytest.mark.parametrize(
    "list1, index, value, expected",
    (
        (TypedList(range(10)), 0, 100, TypedList([100] + list(range(10)))),
        (TypedList("abc"), 2, "e", TypedList("abec")),
    ),
)
def test_insert(list1, index, value, expected):
    list1.insert(index, value)
    assert (list1 == expected).all()


@pytest.mark.parametrize(
    "list1, value, expected",
    (
        (TypedList([0, 1, 0]), 0, TypedList([1, 0])),
        (TypedList("abc"), "b", TypedList("ac")),
    ),
)
def test_remove(list1, value, expected):
    list1.remove(value)
    assert (list1 == expected).all()


@pytest.mark.parametrize(
    "list1, expected",
    (
        (TypedList(range(10)), TypedList(range(10)[::-1])),
        (TypedList("abc"), TypedList("cba")),
        (TypedList(int), TypedList(int)),
    ),
)
def test_reverse(list1, expected):
    list1.reverse()
    assert (list1 == expected).all()


@pytest.mark.parametrize(
    "list1, value, expected",
    (
        (TypedList(range(10)), 0, 1),
        (TypedList(range(10)), 100, 0),
        (TypedList("aab"), "a", 2),
    ),
)
def test_count(list1, value, expected):
    assert list1.count(value) == expected


@pytest.mark.parametrize(
    "list1, expected", ((TypedList(range(10)), True), (TypedList(int), False))
)
def test_dunder_bool(list1, expected):
    assert bool(list1) == expected


@pytest.mark.parametrize(
    "list1, value, expected",
    ((TypedList(range(10)), 0, True), (TypedList("abc"), "d", False)),
)
def test_dunder_contains(list1, value, expected):
    assert (value in list1) == expected


@pytest.mark.parametrize(
    "list1, expected",
    ((TypedList(range(10)), list(range(10))), (TypedList("abc"), ["a", "b", "c"])),
)
def test_dunder_iter(list1, expected):
    assert list(list1) == expected


@pytest.mark.parametrize(
    "list1, index, expected",
    [
        (TypedList(range(10)), 0, 0),
        (TypedList(range(10)), slice(1, 4, 1), TypedList(range(1, 4, 1))),
        (
            TypedList(range(10)),
            BoolTypedList([i % 2 == 0 for i in range(10)]),
            TypedList(range(0, 10, 2)),
        ),
    ],
)
def test_getitem(list1, index, expected):
    if isinstance(
        expected, (BoolTypedList, FloatTypedList, IntTypedList, StringTypedList)
    ):
        assert (list1[index] == expected).all()
    else:
        assert list1[index] == expected


@pytest.mark.parametrize(
    "list1, index, expected",
    [
        (TypedList(range(10)), 0, -100),
    ],
)
def test_setitem(list1, index, expected):
    list1[index] = expected
    assert list1[index] == expected


@pytest.mark.parametrize(
    "list1, index, expected",
    [
        (TypedList(range(10)), 0, TypedList(range(1, 10, 1))),
        (TypedList(range(10)), slice(0, 4, 1), TypedList(range(4, 10, 1))),
        (
            TypedList(range(10)),
            BoolTypedList([i % 2 == 0 for i in range(10)]),
            TypedList(range(1, 10, 2)),
        ),
    ],
)
def test_delitem(list1, index, expected):
    del list1[index]
    assert (list1 == expected).all()


@pytest.mark.parametrize(
    "list1, value, expected",
    (
        (TypedList(range(10)), 0, 0),
        (TypedList("abc"), "c", 2),
        (TypedList("abc"), "d", None),
    ),
)
def test_index(list1, value, expected):
    assert list1.index(value) == expected


@pytest.mark.parametrize(
    "list1, value, expected",
    (
        (TypedList(range(10)), 4, 4),
        (TypedList("abc"), "c", 2),
        (TypedList("abc"), "d", None),
    ),
)
def test_find_any(list1, value, expected):
    assert list1.find_any(value) == expected


@pytest.mark.parametrize(
    "list1, value, expected",
    (
        (TypedList(range(10)), 4, TypedList([4])),
        (TypedList("cabcc"), "c", TypedList([0, 3, 4])),
    ),
)
def test_find_all(list1, value, expected):
    assert (list1.find_all(value) == expected).all()
