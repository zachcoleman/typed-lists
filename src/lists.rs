use pyo3::exceptions::{PyIndexError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

use crate::make_base;
use crate::make_typed_list;

make_base!(Int, isize);
make_base!(Float, f64);
make_base!(String, String);
// make_typed_list!(Int, isize);
// make_typed_list!(Float, f64);

#[pymethods]
impl IntTypedList {
    fn sort(&mut self) -> PyResult<()> {
        self.data.par_sort();
        Ok(())
    }
    fn unique(&self) -> PyResult<IntTypedList> {
        Ok(IntTypedList {
            data: self
                .data
                .par_chunks(10_000)
                .map(|x| x.iter().cloned().collect::<HashSet<isize>>())
                .reduce(
                    || HashSet::new(),
                    |mut a, b| {
                        a.extend(&b);
                        a
                    },
                )
                .into_iter()
                .collect(),
            _ix: 0,
        })
    }
    fn count(&self) -> PyResult<HashMap<isize, usize>> {
        Ok(self
            .data
            .par_chunks(10_000)
            .map(|x| {
                let mut map = HashMap::new();
                for i in x {
                    let count = map.entry(*i).or_insert(0);
                    *count += 1;
                }
                map
            })
            .reduce(
                || HashMap::new(),
                |mut a, b| {
                    for (k, v) in b {
                        let count = a.entry(k).or_insert(0);
                        *count += v;
                    }
                    a
                },
            ))
    }
}
