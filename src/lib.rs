use pyo3::prelude::*;

use glsl_shaderinfo::{get_info, ShaderInfo};

mod glsl_shaderinfo;


#[pymodule]
fn glsl_shaderinfo(_py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "get_info")]
    fn get_info_py(_py: Python, contents: String) -> PyResult<ShaderInfo> {
    	let result = get_info(&contents);
        Ok(result)
    }

    Ok(())
}