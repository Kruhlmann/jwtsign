use pyo3::prelude::*;
use serde_json;

use crate::jwt::JwtEncoderDecoder;

#[pyclass]
pub struct PyJwtEncoderDecoder {
    encoder_decoder: JwtEncoderDecoder,
}

#[pymethods]
impl PyJwtEncoderDecoder {
    #[new]
    pub fn new(private_key: Vec<u8>, public_key: Vec<u8>, leeway: u64) -> PyResult<Self> {
        let encoder_decoder = JwtEncoderDecoder::new(private_key, public_key, leeway)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
        Ok(Self { encoder_decoder })
    }

    pub fn encode_claims_json_str(&self, claims_str: String) -> PyResult<String> {
        let claims: serde_json::Value = serde_json::from_str(&claims_str)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
        self.encoder_decoder
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

    pub fn decode(&self, token: String) -> PyResult<PyObject> {
        let binding = Python::acquire_gil();
        let json_module = binding.python().import("json")?;
        let model = self
            .encoder_decoder
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
    module.add_class::<PyJwtEncoderDecoder>()?;
    Ok(())
}
