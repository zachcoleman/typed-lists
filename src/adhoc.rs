use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::PyAny;
use rayon::prelude::*;

use crate::lists::FloatTypedList;
// use crate::lists::IntTypedList;
use crate::lists::StringTypedList;

/// FloatTypedList Sorting ///
#[pymethods]
impl FloatTypedList {
    fn sort(&mut self) -> PyResult<()> {
        self.data.par_sort_by(|a, b| a.partial_cmp(b).unwrap());
        Ok(())
    }
}
// ==================== //

/// StringTypedList Addition ///
trait BroadcastToStringTypedList {
    fn add_list(&self, other: &StringTypedList) -> PyResult<StringTypedList>;
}
impl BroadcastToStringTypedList for String {
    fn add_list(&self, other: &StringTypedList) -> PyResult<StringTypedList> {
        let data = other
            .data
            .par_iter()
            .map(|x| x.to_owned() + self)
            .collect::<Vec<String>>();
        Ok(StringTypedList { data: data, _ix: 0 })
    }
}
impl BroadcastToStringTypedList for StringTypedList {
    fn add_list(&self, other: &StringTypedList) -> PyResult<StringTypedList> {
        let data = self
            .data
            .par_iter()
            .zip(other.data.par_iter())
            .map(|(a, b)| b.to_owned() + a)
            .collect();
        Ok(StringTypedList { data: data, _ix: 0 })
    }
}
#[pymethods]
impl StringTypedList {
    fn __add__(&self, other: &PyAny) -> PyResult<Self> {
        if let Ok(other) = other.extract::<Self>() {
            return other.add_list(self);
        }
        if let Ok(other) = other.extract::<String>() {
            return other.add_list(self);
        }
        Err(PyTypeError::new_err("Unsupported operand type(s) for +"))
    }
}
// ==================== //
