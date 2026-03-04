//! Python bindings for the JSON parser using PyO3.

use std::time::Instant;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

use crate::error::JsonError;
use crate::value::JsonFormat;
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

/// Convert a Python object to a JsonValue.
///
/// Type check order matters: bool must come before numbers because
/// Python's bool is a subclass of int (True == 1, False == 0).
fn py_to_json_value(obj: &Bound<'_, PyAny>) -> PyResult<JsonValue> {
    if obj.is_none() {
        return Ok(JsonValue::Null);
    }
    if let Ok(b) = obj.extract::<bool>() {
        return Ok(JsonValue::Boolean(b));
    }
    if let Ok(n) = obj.extract::<f64>() {
        return Ok(JsonValue::Number(n));
    }
    if let Ok(s) = obj.extract::<String>() {
        return Ok(JsonValue::String(s));
    }
    if let Ok(list) = obj.downcast::<PyList>() {
        let mut items = Vec::new();
        for item in list.iter() {
            items.push(py_to_json_value(&item)?);
        }
        return Ok(JsonValue::Array(items));
    }
    if let Ok(dict) = obj.downcast::<PyDict>() {
        let mut map = std::collections::HashMap::new();
        for (key, value) in dict.iter() {
            let key_str = key.extract::<String>()?;
            map.insert(key_str, py_to_json_value(&value)?);
        }
        return Ok(JsonValue::Object(map));
    }
    Err(PyValueError::new_err(format!(
        "unsupported type: {}",
        obj.get_type().name()?
    )))
}

/// Serialize a Python object to a JSON string.
///
/// If indent is None, produces compact output.
/// If indent is Some(n), produces pretty-printed output with n spaces per level.
#[pyfunction]
#[pyo3(signature = (obj, indent=None))]
fn dumps(obj: &Bound<'_, PyAny>, indent: Option<usize>) -> PyResult<String> {
    let value = py_to_json_value(obj)?;
    match indent {
        None => Ok(value.to_string()),
        Some(n) => Ok(pretty_print(&value, n, 0)),
    }
}

/// Recursively format a JsonValue with indentation.
///
/// Primitives reuse Display (which delegates to JsonFormat).
/// Array and Object need custom handling for indentation and sorted keys.
fn pretty_print(value: &JsonValue, indent_size: usize, depth: usize) -> String {
    match value {
        JsonValue::Array(arr) => {
            if arr.is_empty() {
                return "[]".to_string();
            }
            let inner_indent = " ".repeat(indent_size * (depth + 1));
            let outer_indent = " ".repeat(indent_size * depth);
            let mut result = String::new();
            result.push_str("[\n");
            for (i, item) in arr.iter().enumerate() {
                result.push_str(&inner_indent);
                result.push_str(&pretty_print(item, indent_size, depth + 1));
                if i < arr.len() - 1 {
                    result.push(',');
                }
                result.push('\n');
            }
            result.push_str(&outer_indent);
            result.push(']');
            result
        }
        JsonValue::Object(map) => {
            if map.is_empty() {
                return "{}".to_string();
            }
            let inner_indent = " ".repeat(indent_size * (depth + 1));
            let outer_indent = " ".repeat(indent_size * depth);
            let mut result = String::new();
            result.push_str("{\n");
            let mut entries: Vec<(&String, &JsonValue)> = map.iter().collect();
            entries.sort_by_key(|(k, _)| *k);
            for (i, (key, val)) in entries.iter().enumerate() {
                result.push_str(&inner_indent);
                result.push_str(&key.to_json_string());
                result.push_str(": ");
                result.push_str(&pretty_print(val, indent_size, depth + 1));
                if i < entries.len() - 1 {
                    result.push(',');
                }
                result.push('\n');
            }
            result.push_str(&outer_indent);
            result.push('}');
            result
        }
        // Null, Boolean, Number, String — reuse Display (delegates to JsonFormat)
        _ => value.to_string(),
    }
}

/// Benchmarks JSON parsing performance comparing Rust, Python json, and simplejson.
///
/// Runs the specified number of iterations for each parser and returns
/// the total elapsed time in seconds for each.
///
/// Returns a tuple of `(rust_time, python_json_time, simplejson_time)`.
#[pyfunction]
#[pyo3(signature = (json_str, iterations=1000))]
fn benchmark_performance(
    py: Python<'_>,
    json_str: &str,
    iterations: usize,
) -> PyResult<(f64, f64, f64)> {
    // Warmup Rust parser (100 iterations)
    for _ in 0..100 {
        let _ = crate::parser::parse_json(json_str);
    }

    // Time Rust parser
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = crate::parser::parse_json(json_str);
    }
    let rust_time = start.elapsed().as_secs_f64();

    // Import Python's json module and get loads function
    let json_module = py.import("json")?;
    let json_loads = json_module.getattr("loads")?;

    // Warmup Python json (100 iterations)
    for _ in 0..100 {
        let _ = json_loads.call1((json_str,))?;
    }

    // Time Python json
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = json_loads.call1((json_str,))?;
    }
    let python_json_time = start.elapsed().as_secs_f64();

    // Import simplejson and get loads function
    let simplejson_module = py.import("simplejson")?;
    let simplejson_loads = simplejson_module.getattr("loads")?;

    // Warmup simplejson (100 iterations)
    for _ in 0..100 {
        let _ = simplejson_loads.call1((json_str,))?;
    }

    // Time simplejson
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = simplejson_loads.call1((json_str,))?;
    }
    let simplejson_time = start.elapsed().as_secs_f64();

    Ok((rust_time, python_json_time, simplejson_time))
}

/// Register all Python-callable functions in the module.
#[pymodule]
fn _rust_json_parser(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_json, m)?)?;
    m.add_function(wrap_pyfunction!(parse_json_file, m)?)?;
    m.add_function(wrap_pyfunction!(dumps, m)?)?;
    m.add_function(wrap_pyfunction!(benchmark_performance, m)?)?;
    Ok(())
}
