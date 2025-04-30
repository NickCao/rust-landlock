use pyo3::prelude::*;

use crate::{Ruleset, ABI};

#[pymodule]
fn landlock(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ABI>()?;
    m.add_class::<Ruleset>()?;
    Ok(())
}
