#[macro_export]
macro_rules! make_sort {
    ($name:ident, $type:ty) => {
        paste::paste! {
            #[pymethods]
            impl [< $name TypedList >] {
                #[pyo3(text_signature = "($self, /)")]
                fn sort_inplace(&mut self) -> PyResult<()> {
                    self.data.par_sort_by(|a, b| a.partial_cmp(b).unwrap());
                    Ok(())
                }

                #[pyo3(text_signature = "($self, /)")]
                fn sort(&self) -> PyResult<Self> {
                    let mut data = self.data.clone();
                    data.par_sort_by(|a, b| a.partial_cmp(b).unwrap());
                    Ok(Self { data: data, _ix: 0 })
                }

                #[pyo3(text_signature = "($self, /)")]
                fn argsort(&self) -> PyResult<IntTypedList> {
                    let mut indices: Vec<usize> = (0..self.data.len()).collect();
                    indices.par_sort_by(|&a, &b| self.data[a].partial_cmp(&self.data[b]).unwrap());
                    let indices = indices.into_par_iter().map(|x| x as isize).collect();
                    Ok(IntTypedList{ data: indices, _ix: 0 })
                }
            }
        }
    };
}
