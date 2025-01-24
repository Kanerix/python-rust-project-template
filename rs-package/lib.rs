use pyo3::{exceptions::PyValueError, prelude::*};

#[pyclass]
pub struct Invoice {
    pub tax: f64,
    pub amount: f64,
}

#[pymethods]
impl Invoice {
    #[new]
    pub fn new(amount: f64, tax: f64) -> PyResult<Self> {
        if !(0.0..=1.0).contains(&tax) {
            return Err(PyValueError::new_err("Tax must be between 0 and 1"));
        }

        Ok(Invoice { tax, amount })
    }

    pub fn total(&self) -> f64 {
        calculate_tax(self.amount, self.tax)
    }
}

#[pyfunction]
fn calculate_tax(amount: f64, tax: f64) -> f64 {
    amount * (1.0 + tax)
}

#[pymodule]
fn python_rust_project_template(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(calculate_tax))?;
    m.add_class::<Invoice>()?;
    Ok(())
}
