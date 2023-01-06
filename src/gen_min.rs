#[macro_export]
macro_rules! make_min {
    ($name:ident, $type:ty) => {
        paste::paste! {
            #[pymethods]
            impl [< $name TypedList >] {
                #[pyo3(text_signature = "($self, /)")]
                fn min(&self) -> PyResult<$type> {
                    let min = self.data.par_iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
                    Ok(min.clone())
                }
                #[pyo3(text_signature = "($self, /)")]
                fn max(&self) -> PyResult<$type> {
                    let max = self.data.par_iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
                    Ok(max.clone())
                }
                #[pyo3(text_signature = "($self, /)")]
                fn argmin(&self) -> PyResult<usize> {
                    let min = self.data.par_iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
                    let index = self.data.par_iter().position_first(|x| x.clone() == *min).unwrap();
                    Ok(index)
                }
                #[pyo3(text_signature = "($self, /)")]
                fn argmax(&self) -> PyResult<usize> {
                    let max = self.data.par_iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
                    let index = self.data.par_iter().position_first(|x| x.clone() == *max).unwrap();
                    Ok(index)
                }
            }
        }
    };
}
