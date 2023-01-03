from itertools import chain
from typing import Iterable, Optional, Union

from ._typed_lists_ext import FloatTypedList, IntTypedList, StringTypedList


def TypedList(
    data: Optional[Iterable] = None, type: Optional[str] = None
) -> Union[IntTypedList, FloatTypedList, StringTypedList]:
    """Create a typed list. Function name is capitalized to emulate a class."""
    if data is None and type is None:
        raise ValueError("Must specify either data or type")

    if data is not None and type is not None:
        if type == "int":
            return IntTypedList(data)
        elif type == "float":
            return FloatTypedList(data)
        elif type == "string":
            return StringTypedList(data)
        else:
            raise ValueError("Invalid type")

    # use specified type
    if data is None:
        if type == "int":
            return IntTypedList([])
        elif type == "float":
            return FloatTypedList([])
        elif type == "string":
            return StringTypedList([])
        else:
            raise ValueError("Invalid type")

    # infer type from data
    if type is None:
        _iter = iter(data)
        x = next(_iter)
        if isinstance(x, int):
            return IntTypedList(chain([x], _iter))
        elif isinstance(x, float):
            return FloatTypedList(chain([x], _iter))
        elif isinstance(x, str):
            return StringTypedList(chain([x], _iter))
        else:
            raise ValueError("Invalid data")

    raise ValueError("Invalid data")
