use glsl::syntax::{StorageQualifier, TypeSpecifierNonArray};
use pyo3::class::basic::PyObjectProtocol;
use pyo3::types::{PyTuple, PyUnicode};
use pyo3::prelude::*;

use glsl_shaderinfo::utils::{get_names, pluralise};
use glsl_shaderinfo::{get_info, TypeSpecifier, BlockInfo, FieldInfo, ShaderInfo, VarInfo};

pub mod glsl_shaderinfo;


fn storage_to_string(storage: &StorageQualifier) -> String {
	format!("{:?}", storage)
}

fn glsl_type_spec_to_string(type_specifier: &TypeSpecifierNonArray) -> String {
	format!("{:?}", type_specifier)
}

fn type_specifier_to_string(type_specifier: &TypeSpecifier) -> String {
    match &type_specifier {
        TypeSpecifier::GlslType(glsl_type) => glsl_type_spec_to_string(&glsl_type).to_lowercase(),
        TypeSpecifier::BlockName(block_name) => block_name.clone()
    }
}

fn array_to_str(array: &Option<Vec<usize>>) -> String {
    match array {
        Some(vec) if vec.len() > 0 => format!("[{}]", vec[0]),
        Some(vec) if vec.len() == 0 => "[]".to_string(),
        _ => "".to_string(),
    }
}

#[pyproto]
impl PyObjectProtocol for VarInfo {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.name.clone())
    }

    fn __repr__(&self) -> PyResult<String> {
        let storage_str: String;
        match &self.storage {
            Some(val) => storage_str = format!("{} ", storage_to_string(&val).to_lowercase()),
            None => storage_str = "".to_string(),
        }

        let repr = format!(
            "<VarInfo {storage}{type_specifier} \"{name}{arr}\">",
            storage = storage_str,
            type_specifier = type_specifier_to_string(&self.type_specifier),
            name = self.name,
            arr = self.array_str(),
        );

        Ok(repr)
    }
}
#[pymethods]
impl VarInfo {
    #[getter]
    fn get_storage(&self) -> PyResult<Option<Py<PyAny>>> {
        match &self.storage {
            Some(val) => {
			    let gil = Python::acquire_gil();
			    let py = gil.python();
		        let types = PyModule::import(py, "glsl_shaderinfo.types")?;
		        let storage_enum = types.get("StorageQualifier")?;
		        let args = PyTuple::new(py, &[storage_to_string(&val)]);
		        let storage_val = storage_enum.call1(args)?;
		        Ok(Some(storage_val.into()))
            },
            None => Ok(None),
        }
    }

    #[getter]
    fn get_type_specifier(&self) -> PyResult<Py<PyAny>> {
	    let gil = Python::acquire_gil();
	    let py = gil.python();
        match &self.type_specifier {
            TypeSpecifier::GlslType(glsl_type) => {
		        let types = PyModule::import(py, "glsl_shaderinfo.types")?;
		        let type_spec_enum = types.get("TypeSpecifier")?;
		        let args = PyTuple::new(py, &[glsl_type_spec_to_string(&glsl_type)]);
		        let type_spec_enum_val = type_spec_enum.call1(args)?;
		        Ok(type_spec_enum_val.into())
            },
            TypeSpecifier::BlockName(block_name) => {
            	let py_str = PyUnicode::new(py, &block_name);
            	Ok(py_str.into())
            }
        }
    }

    pub fn array_str(&self) -> String {
        array_to_str(&self.array)
    }
}


#[pyproto]
impl PyObjectProtocol for FieldInfo {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.name.clone())
    }

    fn __repr__(&self) -> PyResult<String> {
        let repr = format!(
            "<FieldInfo {type_specifier} \"{name}{arr}\">",
            type_specifier = type_specifier_to_string(&self.type_specifier),
            name = self.name,
            arr = self.array_str(),
        );
        Ok(repr)
    }
}
#[pymethods]
impl FieldInfo {
    #[getter]
    fn get_type_specifier(&self) -> PyResult<Py<PyAny>> {
	    let gil = Python::acquire_gil();
	    let py = gil.python();
        match &self.type_specifier {
            TypeSpecifier::GlslType(glsl_type) => {
		        let types = PyModule::import(py, "glsl_shaderinfo.types")?;
		        let type_spec_enum = types.get("TypeSpecifier")?;
		        let args = PyTuple::new(py, &[glsl_type_spec_to_string(&glsl_type)]);
		        let type_spec_enum_val = type_spec_enum.call1(args)?;
		        Ok(type_spec_enum_val.into())
            },
            TypeSpecifier::BlockName(block_name) => {
            	let py_str = PyUnicode::new(py, &block_name);
            	Ok(py_str.into())
            }
        }
    }

    pub fn array_str(&self) -> String {
        array_to_str(&self.array)
    }
}

#[pyproto]
impl PyObjectProtocol for BlockInfo {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.name.clone())
    }

    fn __repr__(&self) -> PyResult<String> {
        let repr = format!(
            "<BlockInfo \"{name}\" ({fcount} {label})>",
            name = self.name,
            fcount = self.fields.len(),
            label = pluralise("field", self.fields.len()),
        );
        Ok(repr)
    }
}

#[pyproto]
impl PyObjectProtocol for ShaderInfo {
    fn __repr__(&self) -> PyResult<String> {
        let repr = format!(
            "<ShaderInfo for GLSL: {version} ({in_count} {in_label}, {out_count} {out_label})>",
            version = self.version_str,
            in_count = self.inputs.len(),
            in_label = pluralise("in", self.inputs.len()),
            out_count = self.outputs.len(),
            out_label = pluralise("out", self.outputs.len()),
        );
        Ok(repr)
    }
}
#[pymethods]
impl ShaderInfo {
    pub fn uniform_names(&self) -> Vec<String> {
        get_names(&self.uniforms)
    }

    pub fn input_names(&self) -> Vec<String> {
        get_names(&self.inputs)
    }

    pub fn output_names(&self) -> Vec<String> {
        get_names(&self.outputs)
    }

    pub fn block_names(&self) -> Vec<String> {
        get_names(&self.blocks)
    }
}

#[pymodule]
fn glsl_shaderinfo(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ShaderInfo>()?;
    m.add_class::<VarInfo>()?;
    m.add_class::<BlockInfo>()?;
    m.add_class::<FieldInfo>()?;

    #[pyfn(m, "get_info")]
	#[text_signature = "(contents, /)"]
    fn get_info_py(_py: Python, contents: String) -> PyResult<ShaderInfo> {
        let result = get_info(&contents);
        Ok(result)
    }

    Ok(())
}
