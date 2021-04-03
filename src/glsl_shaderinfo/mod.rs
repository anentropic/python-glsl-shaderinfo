use std::collections::HashMap;
use std::fmt::Debug;

use glsl::parser::Parse as _;
use glsl::syntax::ShaderStage;
use glsl::syntax::{
    ArraySpecifierDimension, Block, Expr, PreprocessorVersion, SingleDeclaration, StorageQualifier,
    StructFieldSpecifier, TypeSpecifierNonArray
};
use glsl::visitor::{Host, Visit, Visitor};
use pyo3::prelude::*;

pub mod utils;

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


#[derive(Clone, Debug)]
pub enum TypeSpecifier {
    GlslType(TypeSpecifierNonArray),
    BlockName(String),
}
impl Default for TypeSpecifier {
    fn default() -> Self { Self::BlockName(String::from("")) }
}


#[pyclass(module="glsl_shaderinfo.glsl_shaderinfo")]
#[derive(Clone, Debug, Default)]
pub struct VarInfo {
    #[pyo3(get)]
    pub name: String,

    // custom getter in lib.rs
    pub storage: Option<StorageQualifier>,

    // custom getter in lib.rs
    pub type_specifier: TypeSpecifier,

    #[pyo3(get)]
    pub array: Option<Vec<usize>>,

    // TODO: interpolation(flat, smooth, no perspective)
    // TODO: precision(high, medium, low)
    // TODO: layout
    // TODO: invariant
    // TODO: precise
    // https://docs.rs/glsl/6.0.0/glsl/syntax/enum.TypeQualifierSpec.html
}
impl Visitor for VarInfo {
    fn visit_storage_qualifier(&mut self, qualifier: &StorageQualifier) -> Visit {
        self.storage = Some(qualifier.clone());
        Visit::Parent
    }
}

#[pyclass(module="glsl_shaderinfo.glsl_shaderinfo")]
#[derive(Clone, Debug, Default)]
pub struct FieldInfo {
    #[pyo3(get)]
    pub name: String,

    // custom getter in lib.rs
    pub type_specifier: TypeSpecifier,

    #[pyo3(get)]
    pub array: Option<Vec<usize>>,

    // TODO
    // https://docs.rs/glsl/6.0.0/glsl/syntax/enum.TypeQualifierSpec.html
}

#[pyclass(module="glsl_shaderinfo.glsl_shaderinfo")]
#[derive(Clone, Debug, Default)]
pub struct BlockInfo {
    #[pyo3(get)]
    pub name: String,

    #[pyo3(get)]
    pub fields: Vec<FieldInfo>,
}
impl Visitor for BlockInfo {
    fn visit_struct_field_specifier(&mut self, field_spec: &StructFieldSpecifier) -> Visit {
        let mut info: FieldInfo = Default::default();

        let identifier = &field_spec.identifiers.0[0];
        info.name = identifier.ident.as_str().to_owned();

        info.type_specifier = TypeSpecifier::GlslType(field_spec.ty.ty.clone());

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

#[pyclass(module="glsl_shaderinfo.glsl_shaderinfo")]
#[derive(Debug, Default)]
pub struct ShaderInfo {
    #[pyo3(get)]
    pub version: usize,

    #[pyo3(get)]
    pub version_str: String,

    #[pyo3(get)]
    pub vars: Vec<VarInfo>,

    #[pyo3(get)]
    pub blocks: HashMap<String, BlockInfo>,

    #[pyo3(get)]
    pub inputs: Vec<VarInfo>,

    #[pyo3(get)]
    pub outputs: Vec<VarInfo>,

    #[pyo3(get)]
    pub uniforms: Vec<VarInfo>,
}
impl Visitor for ShaderInfo {
    /*
    We should visit the top-level nodes of interest and then search their
    children from within the visit_* methods.
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

            info.type_specifier = TypeSpecifier::GlslType(declaration.ty.ty.ty.clone());

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

            match info.storage {
                Some(StorageQualifier::In) => self.inputs.push(info.clone()),
                Some(StorageQualifier::Out) => self.outputs.push(info.clone()),
                Some(StorageQualifier::Uniform) => self.uniforms.push(info.clone()),
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
            var_info.type_specifier = TypeSpecifier::BlockName(block.name.as_str().to_owned());

            block.visit(&mut var_info);

            match var_info.storage {
                Some(StorageQualifier::In) => self.inputs.push(var_info.clone()),
                Some(StorageQualifier::Out) => self.outputs.push(var_info.clone()),
                Some(StorageQualifier::Uniform) => self.uniforms.push(var_info.clone()),
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
            // TODO if the block is anonymous then GLSL allows you to use
            // all the field names are top-level vars
            // ??? how to handle this ???
        }

        self.blocks.insert(block.name.as_str().to_owned(), block_info);
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
