#[macro_export]
macro_rules! make_sort {
    ($name:ident, $type:ty) => {
        paste::paste! {
            #[pymethods]
            impl [< $name TypedList >] {
                #[pyo3(text_signature = "($self, /)")]
                fn sort(&mut self) -> PyResult<()> {
                    self.data.par_sort();
                    Ok(())
                }
            }
        }
    };
}
