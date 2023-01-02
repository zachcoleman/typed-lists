use pyo3::prelude::*;

mod list_gen;
mod lists;
mod string;

#[pyfunction]
#[pyo3(name = "_initialize_global_thread_pool")]
#[pyo3(text_signature = "($num_threads, /)")]
fn initialize_global_thread_pool(num_threads: usize) -> PyResult<()> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();
    Ok(())
}

#[pyfunction]
#[pyo3(name = "_infer_type")]
#[pyo3(text_signature = "($obj, /)")]
fn infer_type(obj: &PyAny) -> PyResult<String> {
    Ok(obj.get_type().name().unwrap().to_string())
}

#[pymodule]
fn _typed_lists_ext(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(initialize_global_thread_pool, m)?)?;
    m.add_function(wrap_pyfunction!(infer_type, m)?)?;
    m.add_class::<lists::IntTypedList>()?;
    m.add_class::<lists::FloatTypedList>()?;
    m.add_class::<string::StringTypedList>()?;
    Ok(())
}
