#[macro_export]
macro_rules! make_numeric_ops {
    ($name:ident, $type:ty) => {
        paste::paste!{
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
                    Ok([< $name TypedList >] { data: data, _ix: 0 })
                }
                fn sub_list(&self, other: &[< $name TypedList >]) -> PyResult<[< $name TypedList >]> {
                    let data = other
                        .data
                        .par_iter()
                        .map(|x| x - self)
                        .collect::<Vec<$type>>();
                    Ok([< $name TypedList >] { data: data, _ix: 0 })
                }
                fn mul_list(&self, other: &[< $name TypedList >]) -> PyResult<[< $name TypedList >]> {
                    let data = other
                        .data
                        .par_iter()
                        .map(|x| x * self)
                        .collect::<Vec<$type>>();
                    Ok([< $name TypedList >] { data: data, _ix: 0 })
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
                    Ok([< $name TypedList >] { data: data, _ix: 0 })
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
                    Ok([< $name TypedList >] { data: data, _ix: 0 })
                }
                fn sub_list(&self, other: &Self) -> PyResult<Self> {
                    let data = self
                        .data
                        .par_iter()
                        .zip(other.data.par_iter())
                        .map(|(a, b)| b - a)
                        .collect();
                    Ok([< $name TypedList >] { data: data, _ix: 0 })
                }
                fn mul_list(&self, other: &Self) -> PyResult<Self> {
                    let data = self
                        .data
                        .par_iter()
                        .zip(other.data.par_iter())
                        .map(|(a, b)| b * a)
                        .collect();
                    Ok([< $name TypedList >] { data: data, _ix: 0 })
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
                    Ok([< $name TypedList >] {
                        data: data,
                        _ix: 0,
                    })
                }
            }

            #[pymethods]
            impl [< $name TypedList >] {
                #[pyo3(text_signature = "($self, /)")]
                fn sum(&self) -> PyResult<$type> {
                    Ok(self.data.par_iter().sum())
                }

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
            }
        }
    };
}
