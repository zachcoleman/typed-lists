#[macro_export]
macro_rules! make_typed_list{
    ($name:ident, $type:ty) => {
        paste::paste!{
            #[pyclass]
            #[derive(Clone)]
            pub struct [< $name TypedList >] {
                pub data: Vec<$type>,
            }

            trait [< BroadcastTo $name TypedList >] {
                fn add_list(&self, other: &[< $name TypedList >]) -> PyResult<[< $name TypedList >]>;
                fn sub_list(&self, other: &[< $name TypedList >]) -> PyResult<[< $name TypedList >]>;
                fn mul_list(&self, other: &[< $name TypedList >]) -> PyResult<[< $name TypedList >]>;
                fn div_list(&self, other: &[< $name TypedList >]) -> PyResult<[< $name TypedList >]>;
            }

            impl [< BroadcastTo $name TypedList >] for $type {
                fn add_list(&self, other: &[< $name TypedList >]) -> PyResult<[< $name TypedList >]> {
                    let data = other
                        .data
                        .par_iter()
                        .map(|x| x + self)
                        .collect::<Vec<$type>>();
                    Ok([< $name TypedList >] { data })
                }
                fn sub_list(&self, other: &[< $name TypedList >]) -> PyResult<[< $name TypedList >]> {
                    let data = other
                        .data
                        .par_iter()
                        .map(|x| x - self)
                        .collect::<Vec<$type>>();
                    Ok([< $name TypedList >] { data })
                }
                fn mul_list(&self, other: &[< $name TypedList >]) -> PyResult<[< $name TypedList >]> {
                    let data = other
                        .data
                        .par_iter()
                        .map(|x| x * self)
                        .collect::<Vec<$type>>();
                    Ok([< $name TypedList >] { data })
                }
                fn div_list(&self, other: &[< $name TypedList >]) -> PyResult<[< $name TypedList >]> {
                    if *self == (0 as $type){
                        return Err(PyValueError::new_err("Division by zero"));
                    }
                    let data = other
                        .data
                        .par_iter()
                        .map(|x| x / self)
                        .collect::<Vec<$type>>();
                    Ok([< $name TypedList >] { data })
                }
            }

            impl [< BroadcastTo $name TypedList >] for [< $name TypedList >] {
                fn add_list(&self, other: &Self) -> PyResult<Self> {
                    let data = self
                        .data
                        .par_iter()
                        .zip(other.data.par_iter())
                        .map(|(a, b)| b + a)
                        .collect();
                    Ok([< $name TypedList >] { data })
                }
                fn sub_list(&self, other: &Self) -> PyResult<Self> {
                    let data = self
                        .data
                        .par_iter()
                        .zip(other.data.par_iter())
                        .map(|(a, b)| b - a)
                        .collect();
                    Ok([< $name TypedList >] { data })
                }
                fn mul_list(&self, other: &Self) -> PyResult<Self> {
                    let data = self
                        .data
                        .par_iter()
                        .zip(other.data.par_iter())
                        .map(|(a, b)| b * a)
                        .collect();
                    Ok([< $name TypedList >] { data })
                }
                fn div_list(&self, other: &Self) -> PyResult<Self> {
                    if self.data.par_iter().any(|x| *x == (0 as $type)) {
                        return Err(PyValueError::new_err("Division by zero"));
                    }
                    let data = self
                        .data
                        .par_iter()
                        .zip(other.data.par_iter())
                        .map(|(a, b)| b / a)
                        .collect();
                    Ok([< $name TypedList >] { data })
                }
            }

            #[pymethods]
            impl [< $name TypedList >] {
                #[new]
                fn new(data: &PyAny) -> Self {
                    [< $name TypedList >]::from_iterable(&data).expect(
                        format!("Failed to create {}TypedList from iterable", stringify!($name)).as_str()
                    )
                }

                #[classmethod]
                fn from_iterable(iterable: &PyAny) -> PyResult<Self> {
                    let mut data = vec![];
                    let iter = iterable.call_method0("__iter__")?;
                    loop {
                        if let Ok(result) = iter.call_method0("__next__") {
                            data.push(result.extract::<$type>()?);
                        } else {
                            break;
                        }
                    }
                    Ok([< $name TypedList >] { data })
                }

                #[getter]
                fn get_data(&self) -> Vec<$type> {
                    self.data.clone()
                }

                #[setter]
                fn set_data(&mut self, data: Vec<$type>) {
                    self.data = data;
                }

                #[pyo3(text_signature = "($self, /)")]
                fn len(&self) -> PyResult<usize> {
                    Ok(self.data.len())
                }

                #[pyo3(text_signature = "($self, item, /)")]
                fn append(&mut self, item: $type) {
                    self.data.push(item);
                }

                #[pyo3(text_signature = "($self, item, /)")]
                fn extend(&mut self, item: &[< $name TypedList >]) {
                    self.data.extend(item.data.clone());
                }

                #[pyo3(text_signature = "($self, /)")]
                fn sum(&self) -> PyResult<$type> {
                    Ok(self.data.par_iter().sum())
                }

                #[pyo3(text_signature = "($self, other: Union[[< $name TypedList >], int], /)")]
                fn add(&self, other: &PyAny) -> PyResult<Self> {
                    if let Ok(other) = other.extract::<Self>() {
                        return other.add_list(self);
                    }
                    if let Ok(other) = other.extract::<$type>() {
                        return other.add_list(self);
                    }
                    Err(PyTypeError::new_err("Unsupported operand type(s) for +"))
                }

                fn __add__(&self, other: &PyAny) -> PyResult<Self> {
                    self.add(other)
                }

                #[pyo3(text_signature = "($self, other: Union[[< $name TypedList >], int], /)")]
                fn sub(&self, other: &PyAny) -> PyResult<Self> {
                    if let Ok(other) = other.extract::<Self>() {
                        return other.sub_list(self);
                    }
                    if let Ok(other) = other.extract::<$type>() {
                        return other.sub_list(self);
                    }
                    Err(PyTypeError::new_err("Unsupported operand type(s) for -"))
                }

                fn __sub__(&self, other: &PyAny) -> PyResult<Self> {
                    self.sub(other)
                }

                #[pyo3(text_signature = "($self, other: Union[[< $name TypedList >], int], /)")]
                fn mul(&self, other: &PyAny) -> PyResult<Self> {
                    if let Ok(other) = other.extract::<Self>() {
                        return other.mul_list(self);
                    }
                    if let Ok(other) = other.extract::<$type>() {
                        return other.mul_list(self);
                    }
                    Err(PyTypeError::new_err("Unsupported operand type(s) for *"))
                }

                fn __mul__(&self, other: &PyAny) -> PyResult<Self> {
                    self.mul(other)
                }

                #[pyo3(text_signature = "($self, other: Union[[< $name TypedList >], int], /)")]
                fn div(&self, other: &PyAny) -> PyResult<Self> {
                    if let Ok(other) = other.extract::<Self>() {
                        return other.div_list(self);
                    }
                    if let Ok(other) = other.extract::<$type>() {
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

                fn __len__(&self) -> PyResult<usize> {
                    self.len()
                }

                fn __getitem__(&self, ix: isize) -> PyResult<$type> {
                    if ix < -(self.data.len() as isize) || ix >= (self.data.len() as isize){
                        return Err(PyIndexError::new_err("Index out of range"));
                    }
                    if ix < 0 {
                        return Ok(self.data[(self.data.len() as isize + ix) as usize]);
                    }
                    Ok(self.data[ix as usize])
                }

                fn __setitem__(&mut self, ix: isize, value: $type) -> PyResult<()> {
                    let len = self.data.len();
                    if ix < -(len as isize) || ix >= (len as isize){
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
                        return Ok(format!("{}TypedList ({:?})", stringify!($name), &self.data[..]));
                    }
                    Ok(format!("{}TypedList({:?}...)", stringify!($name), &self.data[..5]))
                }
            }
        }
    };
}
