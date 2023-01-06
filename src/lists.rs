use pyo3::basic::CompareOp;
use pyo3::exceptions::{PyIndexError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

use crate::adhoc::*;
use crate::make_base;
use crate::make_bool;
use crate::make_cmp;
use crate::make_hash;
use crate::make_min;
use crate::make_numeric_ops;
use crate::make_sort;

// IntTypedList
make_base!(Int, isize);
make_cmp!(Int, isize);
make_hash!(Int, isize);
make_numeric_ops!(Int, isize);
make_min!(Int, isize);
make_sort!(Int, isize);

// FloatTypedList
make_base!(Float, f64);
make_numeric_ops!(Float, f64);
make_min!(Float, f64);
make_sort!(Float, f64);
// cmp handled by adhoc

// StringTypedList
make_base!(String, String);
make_cmp!(String, String);
make_hash!(String, String);
make_min!(String, String);
make_sort!(String, String);
// string addition handled by adhoc

// BoolTypedList
make_base!(Bool, bool);
make_cmp!(Bool, bool);
make_hash!(Bool, bool);
make_min!(Bool, bool);
make_sort!(Bool, bool);
make_bool!(Bool, bool);
// bool sum handled by adhoc
