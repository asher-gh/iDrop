use pyo3::{
	types::{PyModule, PyString},
	Py, PyAny, PyResult, Python,
};

pub fn create_model(path: &str, model_name: &str) -> PyResult<()> {
	let python_code = include_str!("../create_model.py");

	let from_py: PyResult<_> = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
		let script = PyModule::from_code(py, python_code, "", "")?;
		let path = PyString::new(py, path);
		let model_name = PyString::new(py, model_name);
		let new_model: Py<PyAny> = script.getattr("new_model")?.into();
		new_model.call1(py, (path, model_name))
	});

	if let Err(e) = from_py {
		return Err(e);
	};

	Ok(())
}
