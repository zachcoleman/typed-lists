#[macro_export]
macro_rules! make_bool {
    ($name:ident, $type:ty) => {
        paste::paste!{
            trait [< BroadcastTo $name TypedList >] {
                fn and_list(&self, other: &[< $name TypedList >]) -> PyResult<[< $name TypedList >]>;
                fn or_list(&self, other: &[< $name TypedList >]) -> PyResult<[< $name TypedList >]>;
                fn xor_list(&self, other: &[< $name TypedList >]) -> PyResult<[< $name TypedList >]>;
            }

            impl [< BroadcastTo $name TypedList >] for $type {
                fn and_list(&self, other: &[< $name TypedList >]) -> PyResult<[< $name TypedList >]> {
                    let data = other
                        .data
                        .par_iter()
                        .map(|x| x & self)
                        .collect::<Vec<$type>>();
                    Ok([< $name TypedList >] { data: data, _ix: 0 })
                }
                fn or_list(&self, other: &[< $name TypedList >]) -> PyResult<[< $name TypedList >]> {
                    let data = other
                        .data
                        .par_iter()
                        .map(|x| x | self)
                        .collect::<Vec<$type>>();
                    Ok([< $name TypedList >] { data: data, _ix: 0 })
                }
                fn xor_list(&self, other: &[< $name TypedList >]) -> PyResult<[< $name TypedList >]> {
                    let data = other
                        .data
                        .par_iter()
                        .map(|x| x ^ self)
                        .collect::<Vec<$type>>();
                    Ok([< $name TypedList >] { data: data, _ix: 0 })
                }
            }

            impl [< BroadcastTo $name TypedList >] for [< $name TypedList >] {
                fn and_list(&self, other: &[< $name TypedList >]) -> PyResult<[< $name TypedList >]> {
                    if self.len().unwrap() != other.len().unwrap(){
                        return Err(PyValueError::new_err("Lengths of lists must match"));
                    }
                    let data = self
                        .data
                        .par_iter()
                        .zip(other.data.par_iter())
                        .map(|(x, y)| x & y)
                        .collect::<Vec<$type>>();
                    Ok([< $name TypedList >] { data: data, _ix: 0 })
                }
                fn or_list(&self, other: &[< $name TypedList >]) -> PyResult<[< $name TypedList >]> {
                    if self.len().unwrap() != other.len().unwrap(){
                        return Err(PyValueError::new_err("Lengths of lists must match"));
                    }
                    let data = self
                        .data
                        .par_iter()
                        .zip(other.data.par_iter())
                        .map(|(x, y)| x | y)
                        .collect::<Vec<$type>>();
                    Ok([< $name TypedList >] { data: data, _ix: 0 })
                }
                fn xor_list(&self, other: &[< $name TypedList >]) -> PyResult<[< $name TypedList >]> {
                    if self.len().unwrap() != other.len().unwrap(){
                        return Err(PyValueError::new_err("Lengths of lists must match"));
                    }
                    let data = self
                        .data
                        .par_iter()
                        .zip(other.data.par_iter())
                        .map(|(x, y)| x ^ y)
                        .collect::<Vec<$type>>();
                    Ok([< $name TypedList >] { data: data, _ix: 0 })
                }
            }

            #[pymethods]
            impl [< $name TypedList >] {
                fn and(&self, other: &PyAny) -> PyResult<Self> {
                    if let Ok(other) = other.extract::<Self>() {
                        return other.and_list(self);
                    }
                    if let Ok(other) = other.extract::<$type>() {
                        return other.and_list(self);
                    }
                    Err(PyTypeError::new_err("Unsupported operand type(s) for +"))
                }
                fn __and__(&self, other: &PyAny) -> PyResult<Self> {
                    self.and(other)
                }

                fn or(&self, other: &PyAny) -> PyResult<Self> {
                    if let Ok(other) = other.extract::<Self>() {
                        return other.or_list(self);
                    }
                    if let Ok(other) = other.extract::<$type>() {
                        return other.or_list(self);
                    }
                    Err(PyTypeError::new_err("Unsupported operand type(s) for +"))
                }
                fn __or__(&self, other: &PyAny) -> PyResult<Self> {
                    self.or(other)
                }

                fn xor(&self, other: &PyAny) -> PyResult<Self> {
                    if let Ok(other) = other.extract::<Self>() {
                        return other.xor_list(self);
                    }
                    if let Ok(other) = other.extract::<$type>() {
                        return other.xor_list(self);
                    }
                    Err(PyTypeError::new_err("Unsupported operand type(s) for +"))
                }
                fn __xor__(&self, other: &PyAny) -> PyResult<Self> {
                    self.xor(other)
                }
            }
        }
    };
}
