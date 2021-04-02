use std::fmt::Debug;

use pyo3::prelude::*;

use glsl_shaderinfo::{get_info, ShaderInfo, VarInfo, BlockInfo, FieldInfo};

mod glsl_shaderinfo;


#[pyclass]
#[derive(Clone, Debug, Default, Send)]
pub struct WTFInfo {
    pub name: String,
    pub storage: Option<String>,
    pub type_name: String,
    pub array: Option<Vec<usize>>,
}


#[pymodule]
fn glsl_shaderinfo(_py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "get_info")]
    fn get_info_py(_py: Python, contents: String) -> PyResult<ShaderInfo> {
    	let result = get_info(&contents);
        Ok(result)
    }

    m.add_class::<WTFInfo>()?;

    // m.add_class::<ShaderInfo>()?;
    // m.add_class::<VarInfo>()?;
    // m.add_class::<BlockInfo>()?;
    // m.add_class::<FieldInfo>()?;
 
    Ok(())
}