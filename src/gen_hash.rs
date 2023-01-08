#[macro_export]
macro_rules! make_hash {
    ($name:ident, $type:ty) => {
        paste::paste! {
            #[pymethods]
            impl [< $name TypedList >] {
                fn unique(&self) -> PyResult<[< $name TypedList >]> {
                    Ok([< $name TypedList >]{
                        data: self
                            .data
                            .par_iter()
                            .fold(
                                || HashSet::new(),
                                |mut a, b| {
                                    a.insert(b.clone());
                                    a
                                },
                            )
                            .reduce(
                                || HashSet::new(),
                                |mut a, b| {
                                    a.extend(b);
                                    a
                                },
                            )
                            .into_iter()
                            .collect::<Vec<$type>>(),
                        _ix: 0
                    })
                }
                fn count_all(&self) -> PyResult<HashMap<$type, usize>> {
                    Ok(self.data
                        .par_iter()
                        .fold(
                            || HashMap::new(),
                            |mut a, b| {
                                let count = a.entry(b.clone()).or_insert(0);
                                *count += 1;
                                a
                            },
                        )
                        .reduce(
                            || HashMap::new(),
                            |mut a, b| {
                                for (k, v) in b {
                                    let count = a.entry(k).or_insert(0);
                                    *count += v;
                                }
                                a
                            },
                        )
                    )
                }
            }
        }
    };
}
