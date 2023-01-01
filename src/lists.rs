use pyo3::exceptions::{PyIndexError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use rayon::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct TypedList {
    pub data: Vec<isize>,
}

trait BroadcastToTypedList {
    fn add_list(&self, other: &TypedList) -> PyResult<TypedList>;
    fn sub_list(&self, other: &TypedList) -> PyResult<TypedList>;
    fn mul_list(&self, other: &TypedList) -> PyResult<TypedList>;
    fn div_list(&self, other: &TypedList) -> PyResult<TypedList>;
}

impl BroadcastToTypedList for isize {
    fn add_list(&self, other: &TypedList) -> PyResult<TypedList> {
        let data = other
            .data
            .par_iter()
            .map(|x| x + self)
            .collect::<Vec<isize>>();
        Ok(TypedList { data })
    }
    fn sub_list(&self, other: &TypedList) -> PyResult<TypedList> {
        let data = other
            .data
            .par_iter()
            .map(|x| x - self)
            .collect::<Vec<isize>>();
        Ok(TypedList { data })
    }
    fn mul_list(&self, other: &TypedList) -> PyResult<TypedList> {
        let data = other
            .data
            .par_iter()
            .map(|x| x * self)
            .collect::<Vec<isize>>();
        Ok(TypedList { data })
    }
    fn div_list(&self, other: &TypedList) -> PyResult<TypedList> {
        if *self == 0 {
            return Err(PyValueError::new_err("Division by zero"));
        }
        let data = other
            .data
            .par_iter()
            .map(|x| x / self)
            .collect::<Vec<isize>>();
        Ok(TypedList { data })
    }
}

impl BroadcastToTypedList for TypedList {
    fn add_list(&self, other: &Self) -> PyResult<Self> {
        let data = self
            .data
            .par_iter()
            .zip(other.data.par_iter())
            .map(|(a, b)| b + a)
            .collect();
        Ok(TypedList { data })
    }
    fn sub_list(&self, other: &Self) -> PyResult<Self> {
        let data = self
            .data
            .par_iter()
            .zip(other.data.par_iter())
            .map(|(a, b)| b - a)
            .collect();
        Ok(TypedList { data })
    }
    fn mul_list(&self, other: &Self) -> PyResult<Self> {
        let data = self
            .data
            .par_iter()
            .zip(other.data.par_iter())
            .map(|(a, b)| b * a)
            .collect();
        Ok(TypedList { data })
    }
    fn div_list(&self, other: &Self) -> PyResult<Self> {
        if self.data.par_iter().any(|x| *x == 0) {
            return Err(PyValueError::new_err("Division by zero"));
        }
        let data = self
            .data
            .par_iter()
            .zip(other.data.par_iter())
            .map(|(a, b)| b / a)
            .collect();
        Ok(TypedList { data })
    }
}

#[pymethods]
impl TypedList {
    #[new]
    fn new(data: &PyAny) -> Self {
        TypedList::from_iterable(&data).expect("Failed to create TypedList from iterable")
    }

    #[classmethod]
    fn from_iterable(iterable: &PyAny) -> PyResult<Self> {
        let mut data = vec![];
        let iter = iterable.call_method0("__iter__")?;
        loop {
            if let Ok(result) = iter.call_method0("__next__") {
                data.push(result.extract::<isize>()?);
            } else {
                break;
            }
        }
        Ok(TypedList { data })
    }

    #[getter]
    fn get_data(&self) -> Vec<isize> {
        self.data.clone()
    }

    #[setter]
    fn set_data(&mut self, data: Vec<isize>) {
        self.data = data;
    }

    #[pyo3(text_signature = "($self, /)")]
    fn len(&self) -> PyResult<usize> {
        Ok(self.data.len())
    }

    #[pyo3(text_signature = "($self, /)")]
    fn sum(&self) -> PyResult<isize> {
        Ok(self.data.par_iter().sum())
    }

    fn __getitem__(&self, ix: isize) -> PyResult<isize> {
        if ix < -(self.data.len() as isize) || ix >= (self.data.len() as isize) {
            return Err(PyIndexError::new_err("Index out of range"));
        }
        if ix < 0 {
            return Ok(self.data[(self.data.len() as isize + ix) as usize]);
        }
        Ok(self.data[ix as usize])
    }

    #[pyo3(text_signature = "($self, other: Union[TypedList, int], /)")]
    fn add(&self, other: &PyAny) -> PyResult<Self> {
        if let Ok(other) = other.extract::<Self>() {
            return other.add_list(self);
        }
        if let Ok(other) = other.extract::<isize>() {
            return other.add_list(self);
        }
        Err(PyTypeError::new_err("Unsupported operand type(s) for +"))
    }

    fn __add__(&self, other: &PyAny) -> PyResult<Self> {
        self.add(other)
    }

    #[pyo3(text_signature = "($self, other: Union[TypedList, int], /)")]
    fn sub(&self, other: &PyAny) -> PyResult<Self> {
        if let Ok(other) = other.extract::<Self>() {
            return other.sub_list(self);
        }
        if let Ok(other) = other.extract::<isize>() {
            return other.sub_list(self);
        }
        Err(PyTypeError::new_err("Unsupported operand type(s) for -"))
    }

    fn __sub__(&self, other: &PyAny) -> PyResult<Self> {
        self.sub(other)
    }

    #[pyo3(text_signature = "($self, other: Union[TypedList, int], /)")]
    fn mul(&self, other: &PyAny) -> PyResult<Self> {
        if let Ok(other) = other.extract::<Self>() {
            return other.mul_list(self);
        }
        if let Ok(other) = other.extract::<isize>() {
            return other.mul_list(self);
        }
        Err(PyTypeError::new_err("Unsupported operand type(s) for *"))
    }

    fn __mul__(&self, other: &PyAny) -> PyResult<Self> {
        self.mul(other)
    }

    #[pyo3(text_signature = "($self, other: Union[TypedList, int], /)")]
    fn div(&self, other: &PyAny) -> PyResult<Self> {
        if let Ok(other) = other.extract::<Self>() {
            return other.div_list(self);
        }
        if let Ok(other) = other.extract::<isize>() {
            return other.div_list(self);
        }
        Err(PyTypeError::new_err("Unsupported operand type(s) for /"))
    }

    fn __truediv__(&self, other: &PyAny) -> PyResult<Self> {
        self.div(other)
    }

    fn __floordiv__(&self, other: &PyAny) -> PyResult<Self> {
        self.div(other)
    }

    fn __repr__(&self) -> PyResult<String> {
        if self.data.len() <= 5 {
            return Ok(format!("TypedList({:?})", &self.data[..]));
        }
        Ok(format!("TypedList({:?}...)", &self.data[..5]))
    }
}
