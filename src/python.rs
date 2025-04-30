use pyo3::prelude::*;

use crate::{AccessFs, AccessNet, PyRuleset, ABI};

#[pymodule]
fn landlock(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ABI>()?;
    m.add_class::<PyRuleset>()?;
    m.add_class::<AccessFs>()?;
    m.add_class::<AccessNet>()?;
    Ok(())
}
