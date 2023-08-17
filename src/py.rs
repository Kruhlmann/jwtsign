use pyo3::prelude::*;
use serde_json;

use crate::jwt::{encoder::JwtEncoder, decoder::JwtDecoder};

#[pyclass]
pub struct PyJwtEncoder {
    encoder: JwtEncoder,
}

#[pymethods]
impl PyJwtEncoder {
    #[new]
    pub fn new(private_key: Vec<u8>) -> PyResult<Self> {
        let encoder = JwtEncoder::new(private_key)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
        Ok(Self { encoder })
    }

    pub fn encode_claims_json_str(&self, claims_str: String) -> PyResult<String> {
        let claims: serde_json::Value = serde_json::from_str(&claims_str)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
        self.encoder
            .encode(&claims)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))
    }

    pub fn encode_claims_json_obj(&self, claims: PyObject) -> PyResult<String> {
        let binding = Python::acquire_gil();
        let json_module = binding.python().import("json")?;
        let claims_str: String = json_module
            .getattr("dumps")?
            .call1((claims,))?
            .extract::<String>()?;
        self.encode_claims_json_str(claims_str)
    }
}

#[pyclass]
pub struct PyJwtDecoder {
    decoder: JwtDecoder,
}

#[pymethods]
impl PyJwtDecoder {
    #[new]
    pub fn new(public_key: Vec<u8>, leeway: u64) -> PyResult<Self> {
        let decoder = JwtDecoder::new(public_key, leeway)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
        Ok(Self { decoder })
    }

    pub fn decode(&self, token: String) -> PyResult<PyObject> {
        let binding = Python::acquire_gil();
        let json_module = binding.python().import("json")?;
        let model = self
            .decoder
            .decode::<serde_json::Value>(token)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
        let json_str = serde_json::to_string(&model)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
        let claims_dict = json_module.getattr("loads")?.call1((json_str,))?;
        Ok(claims_dict.into())
    }
}

#[pymodule]
fn libjwtsign(_: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<PyJwtEncoder>()?;
    module.add_class::<PyJwtDecoder>()?;
    Ok(())
}
