use pyo3::prelude::*;

// codegen macros
mod gen_base;
mod gen_hash;
mod gen_num;
mod gen_sort;

// modules
mod adhoc;
mod lists;

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

#[pymodule]
fn _typed_lists_ext(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(initialize_global_thread_pool, m)?)?;
    m.add_class::<lists::IntTypedList>()?;
    m.add_class::<lists::FloatTypedList>()?;
    m.add_class::<lists::StringTypedList>()?;
    Ok(())
}
