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
                            .par_chunks(10_000)
                            .map(|x| x.iter().cloned().collect::<HashSet<$type>>())
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
                fn count(&self) -> PyResult<HashMap<$type, usize>> {
                    Ok(self
                        .data
                        .par_chunks(10_000)
                        .map(|x| {
                            let mut map = HashMap::new();
                            for i in x {
                                // clone is needed to own String
                                let count = map.entry(i.clone()).or_insert(0);
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
                        )
                    )
                }
            }
        }
    };
}
