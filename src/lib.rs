use pyo3::prelude::*;
use std::hash::{Hash, Hasher};
extern crate indexmap;

// We use this to promise that PyObject is comparable and hashable.
#[derive(Clone)]
struct MyPyObject(PyObject);

impl PartialEq for MyPyObject {
    fn eq(&self, other: &Self) -> bool {
        Python::with_gil(|py| self.0.bind(py).eq(other.0.bind(py)).unwrap())
    }
}

impl Eq for MyPyObject {}

impl Hash for MyPyObject {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Python::with_gil(|py| {
            let hash_int = self.0.bind(py).hash().unwrap();
            hash_int.hash(state);
        })
    }
}

#[pyclass]
struct IndexSetIterator {
    container: Py<IndexSet>,
    index: usize,
}

#[pymethods]
impl IndexSetIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PyObject> {
        let res = slf
            .container
            .borrow(slf.py())
            .0
            .get_index(slf.index)
            .map(|x| x.0.clone_ref(slf.py()));
        slf.index += 1;
        res
    }
}

#[pyclass]
struct IndexSet(indexmap::IndexSet<MyPyObject>);

#[pymethods]
impl IndexSet {
    #[new]
    pub fn new() -> Self {
        IndexSet(indexmap::IndexSet::new())
    }

    pub fn __len__(&self) -> usize {
        self.0.len()
    }

    pub fn clear(&mut self) -> PyResult<()> {
        Ok(self.0.clear())
    }

    pub fn add(&mut self, item: PyObject) -> PyResult<()> {
        self.0.insert(MyPyObject(item));
        Ok(())
    }

    pub fn __and__(&self, other: &Self) -> Self {
        IndexSet(
            self.0
                .intersection(&other.0)
                .map(|item| item.clone())
                .collect(),
        )
    }

    #[classattr]
    const __hash__: Option<Py<PyAny>> = None;

    pub fn __iter__(slf: Bound<'_, Self>) -> PyResult<Py<IndexSetIterator>> {
        let iter = IndexSetIterator {
            container: slf.clone().unbind(),
            index: 0,
        };
        Py::new(slf.py(), iter)
    }

    // FIXME: Should implement __traverse__, __clear__
    // https://pyo3.rs/main/class/protocols.html#garbage-collector-integration
}

#[pyclass(frozen)]
struct FrozenIndexSet(indexmap::IndexSet<MyPyObject>);

#[pymethods]
impl FrozenIndexSet {
    #[new]
    pub fn new() -> Self {
        FrozenIndexSet(indexmap::IndexSet::new())
    }

    fn __hash__(slf: Bound<'_, Self>) -> u64 {
        let mut hash = 0;
        for val in slf.borrow().0.iter() {
            hash ^= val.0.bind(slf.py()).hash().unwrap();
        }
        hash.try_into().unwrap()
    }

    // FIXME: Should implement __traverse__, __clear__
    // https://pyo3.rs/main/class/protocols.html#garbage-collector-integration
}

#[pymodule]
fn indexset(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<IndexSetIterator>()?;
    m.add_class::<IndexSet>()?;
    m.add_class::<FrozenIndexSet>()?;
    Ok(())
}
