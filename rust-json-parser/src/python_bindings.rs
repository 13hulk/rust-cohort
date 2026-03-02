//! Python bindings for the JSON parser using PyO3.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

use crate::error::JsonError;
use crate::value::JsonValue;

/// Convert JsonValue to Python native types.
impl<'py> IntoPyObject<'py> for JsonValue {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            JsonValue::Null => Ok(py.None().into_bound(py)),
            JsonValue::Boolean(b) => Ok(b.into_pyobject(py)?.to_owned().into_any()),
            JsonValue::Number(n) => Ok(n.into_pyobject(py)?.to_owned().into_any()),
            JsonValue::String(s) => Ok(s.into_pyobject(py)?.into_any()),
            JsonValue::Array(arr) => {
                let py_list = PyList::empty(py);
                for item in arr {
                    py_list.append(item.into_pyobject(py)?)?;
                }
                Ok(py_list.into_any())
            }
            JsonValue::Object(map) => {
                let py_dict = PyDict::new(py);
                for (key, value) in map {
                    py_dict.set_item(key, value.into_pyobject(py)?)?;
                }
                Ok(py_dict.into_any())
            }
        }
    }
}

/// Convert Rust JSON errors to Python exceptions.
impl From<JsonError> for PyErr {
    fn from(err: JsonError) -> PyErr {
        PyValueError::new_err(err.to_string())
    }
}

/// Parse a JSON string and return a Python object.
#[pyfunction]
fn parse_json(py: Python<'_>, input: &str) -> PyResult<PyObject> {
    let value = crate::parser::JsonParser::new(input)?.parse()?;
    Ok(value.into_pyobject(py)?.unbind())
}

/// Parse a JSON file and return a Python object.
#[pyfunction]
fn parse_json_file(py: Python<'_>, path: &str) -> PyResult<PyObject> {
    let contents = std::fs::read_to_string(path)?;
    let value = crate::parser::JsonParser::new(&contents)?.parse()?;
    Ok(value.into_pyobject(py)?.unbind())
}

/// Register all Python-callable functions in the module.
#[pymodule]
fn _rust_json_parser(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_json, m)?)?;
    m.add_function(wrap_pyfunction!(parse_json_file, m)?)?;
    Ok(())
}
