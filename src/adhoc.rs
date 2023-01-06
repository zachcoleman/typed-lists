use pyo3::basic::CompareOp;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::PyAny;
use pyo3::types::PySlice;
use rayon::prelude::*;

use crate::lists::{BoolTypedList, FloatTypedList, IntTypedList, StringTypedList};

/// Indexing for Slice+Int+BoolTypedList ///
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

/// IntTypedList and FloatTypedList Modulus ///
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

#[pymethods]
impl FloatTypedList {
    fn __mod__(&self, other: &PyAny) -> PyResult<Self> {
        if let Ok(other) = other.extract::<f64>() {
            let data = self.data.par_iter().map(|x| x % other).collect();
            return Ok(FloatTypedList { data: data, _ix: 0 });
        } else if let Ok(other) = other.extract::<Self>() {
            let data = self
                .data
                .par_iter()
                .zip(other.data.par_iter())
                .map(|(a, b)| a % b)
                .collect();
            return Ok(FloatTypedList { data: data, _ix: 0 });
        }
        Err(PyTypeError::new_err("Unsupported operand type(s) for %"))
    }
}
// ==================== //

/// FloatTypedList Comparison ///
#[pymethods]
impl FloatTypedList {
    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<BoolTypedList> {
        match op {
            CompareOp::Lt => {
                if let Ok(other) = other.extract::<f64>() {
                    let data = self.data.par_iter().map(|x| x < &other).collect();
                    return Ok(BoolTypedList { data: data, _ix: 0 });
                } else if let Ok(other) = other.extract::<Self>() {
                    let data = self
                        .data
                        .par_iter()
                        .zip(other.data.par_iter())
                        .map(|(a, b)| a < b)
                        .collect();
                    return Ok(BoolTypedList { data: data, _ix: 0 });
                }
                Err(PyTypeError::new_err("Invalid type for comparison"))
            }
            CompareOp::Gt => {
                if let Ok(other) = other.extract::<f64>() {
                    let data = self.data.par_iter().map(|x| x > &other).collect();
                    return Ok(BoolTypedList { data: data, _ix: 0 });
                } else if let Ok(other) = other.extract::<Self>() {
                    let data = self
                        .data
                        .par_iter()
                        .zip(other.data.par_iter())
                        .map(|(a, b)| a > b)
                        .collect();
                    return Ok(BoolTypedList { data: data, _ix: 0 });
                }
                Err(PyTypeError::new_err("Invalid type for comparison"))
            }
            _ => Err(PyTypeError::new_err("Invalid comparison for floats")),
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

/// Implement `__pow__` methods ///
#[pymethods]
impl IntTypedList {
    fn __pow__(&self, other: &PyAny, modulo: Option<&PyAny>) -> PyResult<Self> {
        match (other, modulo) {
            (other, None) => {
                if let Ok(other) = other.extract::<isize>() {
                    let data = self.data.par_iter().map(|x| x.pow(other as u32)).collect();
                    return Ok(IntTypedList { data: data, _ix: 0 });
                } else if let Ok(other) = other.extract::<Self>() {
                    let data = self
                        .data
                        .par_iter()
                        .zip(other.data.par_iter())
                        .map(|(a, b)| a.pow(*b as u32))
                        .collect();
                    return Ok(IntTypedList { data: data, _ix: 0 });
                }
                return Err(PyTypeError::new_err("Unsupported operand type(s) for **"));
            }
            (other, Some(modulo)) => {
                if let (Ok(other), Ok(modulo)) =
                    (other.extract::<isize>(), modulo.extract::<isize>())
                {
                    let data = self
                        .data
                        .par_iter()
                        .map(|x| x.pow(other as u32) % modulo)
                        .collect();
                    return Ok(IntTypedList { data: data, _ix: 0 });
                } else if let (Ok(other), Ok(modulo)) =
                    (other.extract::<Self>(), modulo.extract::<Self>())
                {
                    let data = self
                        .data
                        .par_iter()
                        .zip(other.data.par_iter())
                        .zip(modulo.data.par_iter())
                        .map(|((a, b), c)| a.pow(*b as u32) % *c)
                        .collect();
                    return Ok(IntTypedList { data: data, _ix: 0 });
                } else if let (Ok(other), Ok(modulo)) =
                    (other.extract::<isize>(), modulo.extract::<Self>())
                {
                    let data = self
                        .data
                        .par_iter()
                        .zip(modulo.data.par_iter())
                        .map(|(a, b)| a.pow(other as u32) % *b)
                        .collect();
                    return Ok(IntTypedList { data: data, _ix: 0 });
                } else if let (Ok(other), Ok(modulo)) =
                    (other.extract::<Self>(), modulo.extract::<isize>())
                {
                    let data = self
                        .data
                        .par_iter()
                        .zip(other.data.par_iter())
                        .map(|(a, b)| a.pow(*b as u32) % modulo)
                        .collect();
                    return Ok(IntTypedList { data: data, _ix: 0 });
                }
                return Err(PyTypeError::new_err("Unsupported operand type(s) for **"));
            }
        }
    }
}

#[pymethods]
impl FloatTypedList {
    fn __pow__(&self, other: &PyAny, modulo: Option<&PyAny>) -> PyResult<Self> {
        match (other, modulo) {
            (other, None) => {
                if let Ok(other) = other.extract::<f64>() {
                    let data: Vec<f64> = self.data.par_iter().map(|x| x.powf(other)).collect();
                    return Ok(FloatTypedList { data: data, _ix: 0 });
                } else if let Ok(other) = other.extract::<Self>() {
                    let data: Vec<f64> = self
                        .data
                        .par_iter()
                        .zip(other.data.par_iter())
                        .map(|(a, b)| a.powf(*b))
                        .collect();
                    return Ok(FloatTypedList { data: data, _ix: 0 });
                }
                return Err(PyTypeError::new_err("Unsupported operand type(s) for **"));
            }
            (other, Some(modulo)) => {
                if let (Ok(other), Ok(modulo)) = (other.extract::<f64>(), modulo.extract::<f64>()) {
                    let data: Vec<f64> = self
                        .data
                        .par_iter()
                        .map(|x| x.powf(other) % modulo)
                        .collect();
                    return Ok(FloatTypedList { data: data, _ix: 0 });
                } else if let (Ok(other), Ok(modulo)) =
                    (other.extract::<Self>(), modulo.extract::<Self>())
                {
                    let data: Vec<f64> = self
                        .data
                        .par_iter()
                        .zip(other.data.par_iter())
                        .zip(modulo.data.par_iter())
                        .map(|((a, b), c)| a.powf(*b) % c)
                        .collect();
                    return Ok(FloatTypedList { data: data, _ix: 0 });
                } else if let (Ok(other), Ok(modulo)) =
                    (other.extract::<Self>(), modulo.extract::<f64>())
                {
                    let data: Vec<f64> = self
                        .data
                        .par_iter()
                        .zip(other.data.par_iter())
                        .map(|(a, b)| a.powf(*b) % modulo)
                        .collect();
                    return Ok(FloatTypedList { data: data, _ix: 0 });
                } else if let (Ok(other), Ok(modulo)) =
                    (other.extract::<f64>(), modulo.extract::<Self>())
                {
                    let data: Vec<f64> = self
                        .data
                        .par_iter()
                        .zip(modulo.data.par_iter())
                        .map(|(a, b)| a.powf(other) % b)
                        .collect();
                    return Ok(FloatTypedList { data: data, _ix: 0 });
                }
                return Err(PyTypeError::new_err(
                    "Unsupported operand type(s) for ** (and %)",
                ));
            }
        }
    }
}
// ==================== //
