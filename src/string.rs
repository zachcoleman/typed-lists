use pyo3::exceptions::{PyIndexError, PyTypeError};
use pyo3::prelude::*;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

#[pyclass]
#[derive(Clone)]
pub struct StringTypedList {
    pub data: Vec<String>,
}

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
        Ok(StringTypedList { data })
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
        Ok(StringTypedList { data })
    }
}

#[pymethods]
impl StringTypedList {
    #[new]
    fn new(data: &PyAny) -> Self {
        StringTypedList::from_iterable(&data)
            .expect("Failed to create StringTypedList from iterable")
    }

    #[classmethod]
    fn from_iterable(iterable: &PyAny) -> PyResult<Self> {
        let mut data = vec![];
        let iter = iterable.call_method0("__iter__")?;
        loop {
            if let Ok(result) = iter.call_method0("__next__") {
                data.push(result.extract::<String>()?);
            } else {
                break;
            }
        }
        Ok(StringTypedList { data })
    }

    #[getter]
    fn get_data(&self) -> Vec<String> {
        self.data.clone()
    }

    #[setter]
    fn set_data(&mut self, data: Vec<String>) {
        self.data = data;
    }

    #[pyo3(text_signature = "($self, /)")]
    fn len(&self) -> PyResult<usize> {
        Ok(self.data.len())
    }

    #[pyo3(text_signature = "($self, item: str, /)")]
    fn append(&mut self, item: String) {
        self.data.push(item);
    }

    #[pyo3(text_signature = "($self, index: int, /)")]
    fn extend(&mut self, other: &StringTypedList) {
        self.data.extend(other.data.clone());
    }

    #[pyo3(text_signature = "($self, other: Union[[< $name TypedList >], int], /)")]
    fn add(&self, other: &PyAny) -> PyResult<Self> {
        if let Ok(other) = other.extract::<Self>() {
            return other.add_list(self);
        }
        if let Ok(other) = other.extract::<String>() {
            return other.add_list(self);
        }
        Err(PyTypeError::new_err("Unsupported operand type(s) for +"))
    }

    fn __add__(&self, other: &PyAny) -> PyResult<Self> {
        self.add(other)
    }

    fn __len__(&self) -> PyResult<usize> {
        self.len()
    }

    fn __getitem__(&self, ix: isize) -> PyResult<String> {
        if ix < -(self.data.len() as isize) || ix >= (self.data.len() as isize) {
            return Err(PyIndexError::new_err("Index out of range"));
        }
        if ix < 0 {
            return Ok(self.data[(self.data.len() as isize + ix) as usize].clone());
        }
        Ok(self.data[ix as usize].clone())
    }

    fn __setitem__(&mut self, ix: isize, value: String) -> PyResult<()> {
        let len = self.data.len();
        if ix < -(len as isize) || ix >= (len as isize) {
            return Err(PyIndexError::new_err("Index out of range"));
        }
        if ix < 0 {
            self.data[(len as isize + ix) as usize] = value;
            return Ok(());
        }
        self.data[ix as usize] = value;
        Ok(())
    }

    fn __repr__(&self) -> PyResult<String> {
        if self.data.len() <= 5 {
            return Ok(format!("{}TypedList ({:?})", "String", &self.data[..]));
        }
        Ok(format!("{}TypedList({:?}...)", "String", &self.data[..5]))
    }
}

#[pymethods]
impl StringTypedList {
    fn sort(&mut self) -> PyResult<()> {
        self.data.par_sort();
        Ok(())
    }
    fn unique(&self) -> PyResult<StringTypedList> {
        Ok(StringTypedList {
            data: self
                .data
                .par_chunks(10_000)
                .map(|x| x.iter().cloned().collect::<HashSet<String>>())
                .reduce(
                    || HashSet::new(),
                    |mut a, b| {
                        a.extend(b);
                        a
                    },
                )
                .into_iter()
                .collect::<Vec<String>>(),
        })
    }
    fn count(&self) -> PyResult<HashMap<&String, usize>> {
        Ok(self
            .data
            .par_chunks(10_000)
            .map(|x| {
                let mut map = HashMap::new();
                for i in x {
                    let count = map.entry(i).or_insert(0);
                    *count += 1;
                }
                map
            })
            .reduce(
                || HashMap::new(),
                |mut a, b| {
                    for (k, v) in b {
                        let count = a.entry(k).or_insert(0);
                        *count += v;
                    }
                    a
                },
            ))
    }
}
