use pyo3::prelude::*;

use crate::ABI;

#[pymodule]
fn landlock(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ABI>()?;
    Ok(())
}
