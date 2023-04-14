#[cfg(not(feature = "python-module"))]
use pyo3_nullify::*;

#[cfg(feature = "python-module")]
use pyo3::prelude::*;

#[pyclass]
pub struct SampleStruct {
    field: u8,
}

#[pymethods]
impl SampleStruct {
    #[new]
    pub fn new() -> SampleStruct {
        SampleStruct { field: 0 }
    }
    #[getter]
    pub fn field(&self) -> u8 {
        self.field
    }
    #[setter]
    pub fn set_field(&mut self, f: u8) {
        self.field = f;
    }
}

#[cfg(feature = "python-module")]
#[pymodule]
fn packet(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<SampleStruct>()?;

    Ok(())
}
