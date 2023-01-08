#[macro_export]
macro_rules! make_base {
    ($name:ident, $type:ty) => {
        paste::paste!{
            #[pyclass]
            #[derive(Clone)]
            pub struct [< $name TypedList >] {
                pub data: Vec<$type>,
                pub _ix : usize,
            }

            #[pymethods]
            impl [< $name TypedList >] {
                #[new]
                fn new(data: &PyAny) -> Self {
                    [< $name TypedList >]::from_iterable(&data).expect(
                        format!("Failed to create {}TypedList from iterable", stringify!($name)).as_str()
                    )
                }

                #[classmethod]
                fn from_iterable(iterable: &PyAny) -> PyResult<Self> {
                    let mut data = vec![];
                    let iter = iterable.call_method0("__iter__")?;
                    loop {
                        if let Ok(result) = iter.call_method0("__next__") {
                            data.push(result.extract::<$type>()?);
                        } else {
                            break;
                        }
                    }
                    Ok([< $name TypedList >] {
                        data: data,
                        _ix: 0,
                    })
                }

                #[getter]
                fn get_data(&self) -> Vec<$type> {
                    self.data.clone()
                }

                #[setter]
                fn set_data(&mut self, data: Vec<$type>) {
                    self.data = data;
                }

                #[getter(_ix)]
                fn get_ix(&self) -> usize {
                    self._ix
                }

                #[pyo3(text_signature = "($self, /)")]
                fn len(&self) -> PyResult<usize> {
                    Ok(self.data.len())
                }

                #[pyo3(text_signature = "($self, item, /)")]
                fn append(&mut self, item: $type) {
                    self.data.push(item);
                }

                #[pyo3(text_signature = "($self, item, /)")]
                fn extend(&mut self, item: &[< $name TypedList >]) {
                    self.data.extend(item.data.clone());
                }

                #[pyo3(text_signature = "($self, index, /)")]
                fn pop(&mut self, index: Option<isize>) -> PyResult<$type> {
                    if let Some(ix) = index {
                        if ix < -(self.data.len() as isize) || ix >= (self.data.len() as isize){
                            return Err(PyIndexError::new_err("Index out of range"));
                        }
                        if ix < 0 {
                            return Ok(self.data.remove((self.data.len() as isize + ix) as usize));
                        }
                        else {
                            return Ok(self.data.remove(ix as usize));
                        }
                    } else {
                        Ok(self.data.pop().unwrap())
                    }
                }

                #[pyo3(text_signature = "($self, index, item /)")]
                fn insert(&mut self, index: isize, item: $type) -> PyResult<()> {
                    if index < -(self.data.len() as isize) || index > (self.data.len() as isize){
                        return Err(PyIndexError::new_err("Index out of range"));
                    }
                    if index < 0 {
                        self.data.insert((self.data.len() as isize + index) as usize, item);
                    }
                    else {
                        self.data.insert(index as usize, item);
                    }
                    Ok(())
                }

                #[pyo3(text_signature = "($self, item, /)")]
                fn remove(&mut self, item: $type) -> PyResult<()> {
                    if let Some(index) = self.data.par_iter().position_first(|x| x == &item) {
                        self.data.remove(index);
                        Ok(())
                    } else {
                        Err(PyValueError::new_err("Item not found"))
                    }
                }

                #[pyo3(text_signature = "($self, /)")]
                fn reverse(&mut self) {
                    self.data.reverse();
                }

                #[pyo3(text_signature = "($self, item, /)")]
                fn count(&self, item: $type) -> PyResult<usize> {
                    Ok(self.data.par_iter().filter(|x| x == &&item).count())
                }
    
                /// ** Magic Methods ** ///
                fn __len__(&self) -> PyResult<usize> {
                    self.len()
                }

                fn __bool__(&self) -> PyResult<bool> {
                    Ok(self.data.len() > 0)
                }

                fn __contains__(&self, item: $type) -> PyResult<bool> {
                    Ok(self.data.par_iter().any(|x| x == &item))
                }

                fn __iter__(mut slf: PyRefMut<'_, Self>) -> PyRefMut<'_, Self> {
                    slf._ix = 0;
                    slf
                }
                fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PyObject> {
                    if slf._ix >= slf.data.len() {
                        slf._ix = 0;
                        return None;
                    }
                    slf._ix += 1;
                    Some(slf.data[slf._ix - 1].to_object(slf.py()))
                }

                fn __getitem__(slf: PyRefMut<'_, Self>, ix: SliceIndexorBoolTypedList) -> PyResult<PyObject> {
                    match ix {
                        SliceIndexorBoolTypedList::Index(ix) => {
                            if ix < -(slf.data.len() as isize) || ix >= (slf.data.len() as isize){
                                return Err(PyIndexError::new_err("Index out of range"));
                            }
                            if ix < 0 {
                                return Ok(slf.data[(slf.data.len() as isize + ix) as usize].clone().to_object(slf.py()));
                            }
                            Ok(slf.data[ix as usize].clone().to_object(slf.py()))
                        }
                        SliceIndexorBoolTypedList::Slice(slice) => {
                            let indices = slice.indices(slf.data.len().try_into().unwrap())?;
                            let start = indices.start;
                            let stop = indices.stop;
                            let step = indices.step;
                            let mut data = vec![];
                            let mut indices = vec![];
                            let mut curr = start;
                            while (curr > stop && step < 0) || (curr < stop && step > 0){
                                indices.push(curr);
                                curr += step;
                            }
                            for ix in indices{
                                data.push(slf.data[ix as usize].clone());
                            }
                            Ok(
                                PyCell::new(
                                    slf.py(),
                                    [< $name TypedList >] {
                                        data: data,
                                        _ix: 0,
                                    }
                                ).unwrap().into()
                            )
                        }
                        SliceIndexorBoolTypedList::BoolTypedList(b) => {
                            let mut data = vec![];
                            for (i, x) in slf.data.iter().enumerate(){
                                if b.data[i]{
                                    data.push(x.clone());
                                }
                            }
                            Ok(
                                PyCell::new(
                                    slf.py(),
                                    [< $name TypedList >] {
                                        data: data,
                                        _ix: 0,
                                    }
                                ).unwrap().into()
                            )
                        }
                    }
                }

                fn __setitem__(&mut self, ix: SliceIndexorBoolTypedList, value: &PyAny ) -> PyResult<()> {
                    match ix {
                        SliceIndexorBoolTypedList::Index(ix) => {
                            if let Ok(v) = value.extract::<$type>(){
                                let l = self.data.len();
                                if ix < -(l as isize) || ix >= (l as isize){
                                    return Err(PyIndexError::new_err("Index out of range"));
                                }
                                if ix < 0 {
                                    self.data[(l as isize + ix) as usize] = v.clone();
                                }
                                self.data[ix as usize] = v.clone();
                                Ok(())
                            }
                            else{
                                Err(PyTypeError::new_err("Invalid type"))
                            }
                        }
                        SliceIndexorBoolTypedList::Slice(slice) => {
                            if let Ok(vs) = value.extract::<[< $name TypedList >]>(){
                                let indices = slice.indices(self.data.len().try_into().unwrap())?;
                                let start = indices.start;
                                let stop = indices.stop;
                                let step = indices.step;
                                let mut indices = vec![];
                                let mut curr = start;
                                while (curr > stop && step < 0) || (curr < stop && step > 0){
                                    indices.push(curr);
                                    curr += step;
                                }
                                if indices.len() != vs.data.len(){
                                    return Err(PyValueError::new_err("Slice length mismatch"));
                                } else{
                                    for (ix, v) in indices.iter().zip(vs.data.iter()){
                                        self.data[*ix as usize] = v.clone();
                                    }
                                }
                                Ok(())
                            }
                            else{
                                Err(PyTypeError::new_err("Invalid type"))
                            }
                        }
                        SliceIndexorBoolTypedList::BoolTypedList(b) => {
                            if let Ok(vs) = value.extract::<[< $name TypedList >]>(){
                                if b.data.len() != vs.data.len(){
                                    return Err(PyValueError::new_err("Slice length mismatch"));
                                } else{
                                    for (i, x) in b.data.iter().enumerate(){
                                        if *x{
                                            self.data[i] = vs.data[i].clone();
                                        }
                                    }
                                }
                                Ok(())
                            }
                            else{
                                Err(PyTypeError::new_err("Invalid type"))
                            }
                        }
                    }
                }

                fn __delitem__(&mut self, ix: SliceIndexorBoolTypedList) -> PyResult<()>{
                    match ix {
                        SliceIndexorBoolTypedList::Index(ix) => {
                            let len = self.data.len();
                            if ix < -(len as isize) || ix >= (len as isize){
                                return Err(PyIndexError::new_err("Index out of range"));
                            }
                            if ix < 0 {
                                self.data.remove((len as isize + ix) as usize);
                                return Ok(());
                            }
                            self.data.remove(ix as usize);
                            Ok(())
                        }
                        SliceIndexorBoolTypedList::Slice(slice) => {
                            let indices = slice.indices(self.data.len().try_into().unwrap())?;
                            let start = indices.start;
                            let stop = indices.stop;
                            let step = indices.step;
                            let mut indices = vec![];
                            let mut curr = start;
                            while (curr > stop && step < 0) || (curr < stop && step > 0){
                                indices.push(curr);
                                curr += step;
                            }
                            indices.par_sort_unstable(); // need to sort so always delete from the end
                            for ix in indices.iter().rev(){
                                self.data.remove(*ix as usize);
                            }
                            Ok(())
                        }
                        SliceIndexorBoolTypedList::BoolTypedList(b) => {
                            let mut indices = vec![];
                            for (i, x) in b.data.iter().enumerate(){
                                if *x{
                                    indices.push(i);
                                }
                            }
                            for ix in indices.iter().rev(){
                                self.data.remove(*ix as usize);
                            }
                            Ok(())
                        }
                    }
                }

                fn index(&self, value: $type) -> Option<isize>{
                    self.data
                        .par_iter()
                        .enumerate()
                        .find_first(|(_, x)| *x == &value)
                        .map(|(i, _)| i as isize)
                        .or_else(|| None)
                }

                fn find_any(&self, value: $type) -> Option<isize>{
                    self.data
                        .par_iter()
                        .enumerate()
                        .find_any(|(_, x)| *x == &value)
                        .map(|(i, _)| i as isize)
                        .or_else(|| None)
                }

                fn find_all(&self, value: $type) -> PyResult<IntTypedList>{
                    let mut indices = self.data
                        .par_iter()
                        .enumerate()
                        .filter(|(_, x)| *x == &value)
                        .map(|(i, _)| i as isize)
                        .collect::<Vec<isize>>();
                    indices.sort_unstable();
                    Ok(IntTypedList{data: indices, _ix: 0})
                }

                fn __repr__(&self) -> PyResult<String> {
                    if self.data.len() <= 5 {
                        return Ok(format!("{}TypedList ({:?})", stringify!($name), &self.data[..]));
                    }
                    Ok(format!("{}TypedList({:?}...)", stringify!($name), &self.data[..5]))
                }

                fn __str__(&self) -> PyResult<String> {
                    self.__repr__()
                }
            }
        }
    };
}
