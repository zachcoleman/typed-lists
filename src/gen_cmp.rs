#[macro_export]
macro_rules! make_cmp {
    ($name:ident, $type:ty) => {
        paste::paste!{
            #[pymethods]
            impl [< $name TypedList >] {
                fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<BoolTypedList> {
                    if let Ok(other) = other.extract::<Self>() {
                        if self.len().unwrap() != other.len().unwrap(){
                            return Err(PyValueError::new_err("Lengths of lists must match"));
                        }
                        match op {
                            CompareOp::Eq => {
                                return Ok(BoolTypedList{
                                    data: self.data.par_iter().zip(other.data.par_iter()).map(|(a, b)| a == b).collect(),
                                    _ix: 0
                                });
                            },
                            CompareOp::Ne => {
                                return Ok(BoolTypedList{
                                    data: self.data.par_iter().zip(other.data.par_iter()).map(|(a, b)| a != b).collect(),
                                    _ix: 0
                                });
                            },
                            CompareOp::Lt => {
                                return Ok(BoolTypedList{
                                    data: self.data.par_iter().zip(other.data.par_iter()).map(|(a, b)| a < b).collect(),
                                    _ix: 0
                                });
                            },
                            CompareOp::Le => {
                                return Ok(BoolTypedList{
                                    data: self.data.par_iter().zip(other.data.par_iter()).map(|(a, b)| a <= b).collect(),
                                    _ix: 0
                                });
                            },
                            CompareOp::Gt => {
                                return Ok(BoolTypedList{
                                    data: self.data.par_iter().zip(other.data.par_iter()).map(|(a, b)| a > b).collect(),
                                    _ix: 0
                                });
                            },
                            CompareOp::Ge => {
                                return Ok(BoolTypedList{
                                    data: self.data.par_iter().zip(other.data.par_iter()).map(|(a, b)| a >= b).collect(),
                                    _ix: 0
                                });
                            },
                        }
                    }
                    else if let Ok(other) = other.extract::<$type>() {
                        match op {
                            CompareOp::Eq => {
                                return Ok(BoolTypedList{
                                    data: self.data.par_iter().map(|a| a == &other).collect(),
                                    _ix: 0
                                });
                            },
                            CompareOp::Ne => {
                                return Ok(BoolTypedList{
                                    data: self.data.par_iter().map(|a| a != &other).collect(),
                                    _ix: 0
                                });
                            },
                            CompareOp::Lt => {
                                return Ok(BoolTypedList{
                                    data: self.data.par_iter().map(|a| a < &other).collect(),
                                    _ix: 0
                                });
                            },
                            CompareOp::Le => {
                                return Ok(BoolTypedList{
                                    data: self.data.par_iter().map(|a| a <= &other).collect(),
                                    _ix: 0
                                });
                            },
                            CompareOp::Gt => {
                                return Ok(BoolTypedList{
                                    data: self.data.par_iter().map(|a| a > &other).collect(),
                                    _ix: 0
                                });
                            },
                            CompareOp::Ge => {
                                return Ok(BoolTypedList{
                                    data: self.data.par_iter().map(|a| a >= &other).collect(),
                                    _ix: 0
                                });
                            },
                        }
                    }
                    Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("Cannot compare"))
                }
            }
        }
    };
}
