#[macro_export]
macro_rules! make_base {
    ($name:ident, $type:ty) => {
        paste::paste!{
            #[pyclass]
            #[derive(Clone)]
            pub struct [< $name TypedList >] {
                pub data: Vec<$type>,
                pub _ix : usize,
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
                    Ok([< $name TypedList >] {
                        data: data,
                        _ix: 0,
                    })
                }

                #[getter]
                fn get_data(&self) -> Vec<$type> {
                    self.data.clone()
                }

                #[setter]
                fn set_data(&mut self, data: Vec<$type>) {
                    self.data = data;
                }

                #[getter(_ix)]
                fn get_ix(&self) -> usize {
                    self._ix
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

                /// ** Magic Methods ** ///
                fn __len__(&self) -> PyResult<usize> {
                    self.len()
                }

                fn __bool__(&self) -> PyResult<bool> {
                    Ok(self.data.len() > 0)
                }

                fn __contains__(&self, item: $type) -> PyResult<bool> {
                    Ok(self.data.par_iter().any(|x| x == &item))
                }

                fn __eq__(&self, other: &PyAny) -> PyResult<bool> {
                    if let Ok(other) = other.extract::<[< $name TypedList >]>() {
                        Ok(self.data.par_iter().zip(other.data.par_iter()).all(|(a, b)| a == b))
                    } else {
                        Ok(false)
                    }
                }

                fn __iter__(mut slf: PyRefMut<'_, Self>) -> PyRefMut<'_, Self> {
                    slf._ix = 0;
                    slf
                }
                fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PyObject> {
                    if slf._ix >= slf.data.len() {
                        slf._ix = 0;
                        return None;
                    }
                    slf._ix += 1;
                    Some(slf.data[slf._ix - 1].to_object(slf.py()))
                }

                fn __getitem__(&self, ix: isize) -> PyResult<$type> {
                    if ix < -(self.data.len() as isize) || ix >= (self.data.len() as isize){
                        return Err(PyIndexError::new_err("Index out of range"));
                    }
                    if ix < 0 {
                        return Ok(self.data[(self.data.len() as isize + ix) as usize].clone());
                    }
                    Ok(self.data[ix as usize].clone())
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

                fn __delitem__(&mut self, ix: isize) -> PyResult<()> {
                    let len = self.data.len();
                    if ix < -(len as isize) || ix >= (len as isize){
                        return Err(PyIndexError::new_err("Index out of range"));
                    }
                    if ix < 0 {
                        self.data.remove((len as isize + ix) as usize);
                        return Ok(());
                    }
                    self.data.remove(ix as usize);
                    Ok(())
                }

                fn __repr__(&self) -> PyResult<String> {
                    if self.data.len() <= 5 {
                        return Ok(format!("{}TypedList ({:?})", stringify!($name), &self.data[..]));
                    }
                    Ok(format!("{}TypedList({:?}...)", stringify!($name), &self.data[..5]))
                }

                fn __str__(&self) -> PyResult<String> {
                    self.__repr__()
                }

                // + should typed-lists append like built-in lists?
                // * should typed-lists repeat like built-in lists?
                // this is  not how numpy works, but it is how python lists work
                // fn __add__(&self, other: &PyAny) -> PyResult<Self> {
                //     self.add(other)
                // }
                // fn __mul__(&self, other: &PyAny) -> PyResult<Self> {
                //     self.mul(other)
                // }
            }
        }
    };
}
