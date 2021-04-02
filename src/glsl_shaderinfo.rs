use std::fmt::Debug;

use glsl::parser::Parse as _;
use glsl::syntax::ShaderStage;
use glsl::syntax::{
    ArraySpecifierDimension, Block, Expr, PreprocessorVersion, SingleDeclaration, StorageQualifier,
    StructFieldSpecifier,
};
use glsl::visitor::{Host, Visit, Visitor};
use pyo3::prelude::*;


pub trait Declaration {
    fn get_name(&self) -> &String;
}

macro_rules! impl_Declaration {
    // https://stackoverflow.com/a/50223259/202168
    (for $($t:ty),+) => {
        $(impl Declaration for $t {
            fn get_name(&self) -> &String {
                return &self.name;
            }
        })*
    }
}

impl_Declaration!(for VarInfo, BlockInfo, FieldInfo);

#[pyclass]
#[derive(Clone, Debug, Default)]
pub struct VarInfo {
    pub name: String,
    pub storage: Option<String>,
    pub type_name: String,
    pub array: Option<Vec<usize>>,
    // TODO: interpolation(flat)
}
impl Visitor for VarInfo {
    fn visit_storage_qualifier(&mut self, qualifier: &StorageQualifier) -> Visit {
        self.storage = Some(format!("{:?}", qualifier));
        Visit::Parent
    }
}

#[pyclass]
#[derive(Clone, Debug, Default)]
pub struct FieldInfo {
    pub name: String,
    pub type_name: String,
    pub array: Option<Vec<usize>>,
    // TODO: interpolation(flat)
}

#[pyclass]
#[derive(Clone, Debug, Default)]
pub struct BlockInfo {
    pub name: String,
    pub fields: Vec<FieldInfo>,
}
impl Visitor for BlockInfo {
    fn visit_struct_field_specifier(&mut self, field_spec: &StructFieldSpecifier) -> Visit {
        let mut info: FieldInfo = Default::default();

        let identifier = &field_spec.identifiers.0[0];
        info.name = identifier.ident.as_str().to_owned();

        info.type_name = format!("{:?}", field_spec.ty.ty);

        if let Some(array_spec) = &identifier.array_spec {
            // I think GLSL can only have 1D array vars...
            let spec_dim = &array_spec.dimensions.0[0];
            match spec_dim {
                ArraySpecifierDimension::ExplicitlySized(value) => match **value {
                    Expr::IntConst(x) => info.array = Some(vec![x as usize]),
                    Expr::UIntConst(x) => info.array = Some(vec![x as usize]),
                    _ => (), // I think only int consts are possible for array dims
                },
                ArraySpecifierDimension::Unsized => info.array = Some(vec![]),
            }
        }

        self.fields.push(info);
        Visit::Parent
    }
}

#[pyclass]
#[derive(Debug, Default)]
pub struct ShaderInfo {
    pub version: usize,
    pub version_str: String,
    pub vars: Vec<VarInfo>,
    pub blocks: Vec<BlockInfo>,
    pub inputs: Vec<VarInfo>,
    pub outputs: Vec<VarInfo>,
    pub uniforms: Vec<VarInfo>,
}
impl Visitor for ShaderInfo {
    /*
    We should visit the top-level nodes of interest and then search their
    children from the visit_* methods.
    */
    fn visit_preprocessor_version(&mut self, version: &PreprocessorVersion) -> Visit {
        self.version = version.version as usize;
        match &version.profile {
            Some(profile) => {
                let profile_str = format!("{:?}", profile).to_lowercase();
                self.version_str = format!("{} {}", version.version, profile_str)
            }
            None => self.version_str = format!("{}", version.version),
        }
        Visit::Parent
    }

    fn visit_single_declaration(&mut self, declaration: &SingleDeclaration) -> Visit {
        /*
        called for any var declaration, including top-level uniforms and const,
        but not for block defs
        */
        if declaration.name.is_some() {
            let mut info: VarInfo = Default::default();
            declaration.visit(&mut info);

            info.name = declaration.name.as_ref().unwrap().as_str().to_owned();

            info.type_name = format!("{:?}", declaration.ty.ty.ty);

            if declaration.array_specifier.is_some() {
                // I think GLSL can only have 1D array vars...
                let spec_dim = &declaration.array_specifier.as_ref().unwrap().dimensions.0[0];
                match spec_dim {
                    ArraySpecifierDimension::ExplicitlySized(value) => match **value {
                        Expr::IntConst(x) => info.array = Some(vec![x as usize]),
                        Expr::UIntConst(x) => info.array = Some(vec![x as usize]),
                        _ => (), // I think only int consts are possible for array dims
                    },
                    ArraySpecifierDimension::Unsized => info.array = Some(vec![]),
                }
            }

            match info.storage.as_ref().map(String::as_ref) {
                Some("In") => self.inputs.push(info.clone()),
                Some("Out") => self.outputs.push(info.clone()),
                Some("Uniform") => self.uniforms.push(info.clone()),
                _ => (),
            }

            self.vars.push(info);
        }
        Visit::Parent
    }

    fn visit_block(&mut self, block: &Block) -> Visit {
        /*
        we treat blocks as defining a type as well as an instance of that type
        */
        let mut block_info: BlockInfo = Default::default();
        // collect fields:
        block.visit(&mut block_info);

        if let Some(identifier) = &block.identifier {
            block_info.name = block.name.as_str().to_owned();

            let mut var_info: VarInfo = Default::default();

            var_info.name = identifier.ident.as_str().to_owned();
            var_info.type_name = block.name.as_str().to_owned();

            block.visit(&mut var_info);

            match var_info.storage.as_ref().map(String::as_ref) {
                Some("In") => self.inputs.push(var_info.clone()),
                Some("Out") => self.outputs.push(var_info.clone()),
                Some("Uniform") => self.uniforms.push(var_info.clone()),
                _ => (),
            }

            if let Some(array_spec) = &identifier.array_spec {
                // I think GLSL can only have 1D array vars...
                let spec_dim = &array_spec.dimensions.0[0];
                match spec_dim {
                    ArraySpecifierDimension::ExplicitlySized(value) => match **value {
                        Expr::IntConst(x) => var_info.array = Some(vec![x as usize]),
                        Expr::UIntConst(x) => var_info.array = Some(vec![x as usize]),
                        _ => (), // I think only int consts are possible for array dims
                    },
                    ArraySpecifierDimension::Unsized => var_info.array = Some(vec![]),
                }
            }

            self.vars.push(var_info);
        } else {
            // TODO all the field names are top-level vars
        }

        self.blocks.push(block_info);
        Visit::Parent
    }
}

pub fn get_info(contents: &String) -> ShaderInfo {
    let result = ShaderStage::parse(contents);

    let shader = match result {
        Ok(parsed) => parsed,
        Err(error) => panic!("Problem parsing the file: {:?}", error),
    };

    let mut info: ShaderInfo = Default::default();
    shader.visit(&mut info);

    return info;
}
