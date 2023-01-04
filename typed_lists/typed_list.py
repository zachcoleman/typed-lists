from itertools import chain
from typing import Iterable, Optional, Union

from ._typed_lists_ext import (
    BoolTypedList,
    FloatTypedList,
    IntTypedList,
    StringTypedList,
)

STR_MAPPING = {
    "bool": BoolTypedList,
    "float": FloatTypedList,
    "int": IntTypedList,
    "string": StringTypedList,
}

TYPE_MAPPING = {
    bool: BoolTypedList,
    float: FloatTypedList,
    int: IntTypedList,
    str: StringTypedList,
}


def TypedList(
    data: Optional[Iterable] = None, dtype: Optional[str] = None
) -> Union[IntTypedList, FloatTypedList, StringTypedList]:
    """Create a typed list. Function name is capitalized to emulate a class."""
    if data is None and dtype is None:
        raise ValueError("Must specify either data or type")

    if data is not None and dtype is not None:
        if dtype in STR_MAPPING:
            return STR_MAPPING[dtype](data)

    if data is None and dtype is not None:
        if dtype in STR_MAPPING:
            return STR_MAPPING[dtype]([])

    if data is not None and dtype is None:
        _iter = iter(data)
        x = next(_iter)
        if type(x) in TYPE_MAPPING:
            return TYPE_MAPPING[type(x)](chain([x], _iter))

    raise ValueError("Invalid data")
