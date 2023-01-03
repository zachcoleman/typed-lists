use pyo3::exceptions::{PyIndexError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

use crate::make_base;
use crate::make_hash;
use crate::make_numeric_ops;
use crate::make_sort;

// IntTypedList
make_base!(Int, isize);
make_numeric_ops!(Int, isize);
make_sort!(Int, isize);
make_hash!(Int, isize);

// FloatTypedList
make_base!(Float, f64);
make_numeric_ops!(Float, f64);

// StringTypedList
make_base!(String, String);
make_sort!(String, String);
make_hash!(String, String);
