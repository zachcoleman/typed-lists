use pyo3::basic::CompareOp;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::PyAny;
use pyo3::types::PySlice;
use rayon::prelude::*;

use crate::lists::{BoolTypedList, FloatTypedList, IntTypedList, StringTypedList};

/// Indexing for slice+ints ///
#[derive(FromPyObject)]
pub enum SliceIndexorBoolTypedList<'a> {
    Slice(&'a PySlice),
    Index(isize),
    BoolTypedList(BoolTypedList),
}
// ==================== //

/// BoolTypedList Any, All, and Sum ///
#[pymethods]
impl BoolTypedList {
    fn any(&self) -> PyResult<bool> {
        Ok(self.data.par_iter().any(|x| *x))
    }
    fn all(&self) -> PyResult<bool> {
        Ok(self.data.par_iter().all(|x| *x))
    }
    fn sum(&self) -> PyResult<isize> {
        let sum = self.data.par_iter().map(|x| *x as isize).sum();
        Ok(sum)
    }
}
// ==================== //

/// IntTypedList Modulus ///
#[pymethods]
impl IntTypedList {
    fn __mod__(&self, other: &PyAny) -> PyResult<Self> {
        if let Ok(other) = other.extract::<isize>() {
            let data = self.data.par_iter().map(|x| x % other).collect();
            return Ok(IntTypedList { data: data, _ix: 0 });
        } else if let Ok(other) = other.extract::<Self>() {
            let data = self
                .data
                .par_iter()
                .zip(other.data.par_iter())
                .map(|(a, b)| a % b)
                .collect();
            return Ok(IntTypedList { data: data, _ix: 0 });
        }
        Err(PyTypeError::new_err("Unsupported operand type(s) for %"))
    }
}
// ==================== //

/// FloatTypedList Sorting ///
#[pymethods]
impl FloatTypedList {
    fn sort(&mut self) -> PyResult<()> {
        self.data.par_sort_by(|a, b| a.partial_cmp(b).unwrap());
        Ok(())
    }
}
// ==================== //

/// FloatTypedList Comparison ///
#[pymethods]
impl FloatTypedList {
    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(self.data < other.data),
            CompareOp::Gt => Ok(self.data > other.data),
            _ => Err(PyTypeError::new_err("Invalid comparison operator")),
        }
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
