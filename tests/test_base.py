import pytest

from typed_lists import TypedList


@pytest.mark.parametrize(
    "list1, expected",
    [
        (TypedList(range(10)), list(range(10))),
        (TypedList([1.0]), [1.0]),
        (TypedList("abc"), list("abc")),
        (TypedList((x % 2 == 0 for x in range(4))), [True, False, True, False]),
        (TypedList(dtype="int"), []),
        (TypedList(dtype="float"), []),
        (TypedList(dtype="string"), []),
        (TypedList(dtype="bool"), []),
    ],
)
def test_init(list1, expected):
    assert list1.data == expected


@pytest.mark.parametrize(
    "list1, value, expected",
    [
        (TypedList([], "int"), 1, [1]),
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
