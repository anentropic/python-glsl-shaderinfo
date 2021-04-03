use pyo3::prelude::*;

use glsl_shaderinfo::{get_info, BlockInfo, FieldInfo, ShaderInfo, VarInfo};

mod glsl_shaderinfo;

#[pymodule]
fn glsl_shaderinfo(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ShaderInfo>()?;
    m.add_class::<VarInfo>()?;
    m.add_class::<BlockInfo>()?;
    m.add_class::<FieldInfo>()?;

    #[pyfn(m, "get_info")]
    fn get_info_py(_py: Python, contents: String) -> PyResult<ShaderInfo> {
        let result = get_info(&contents);
        Ok(result)
    }

    Ok(())
}
