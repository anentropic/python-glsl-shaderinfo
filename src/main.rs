use std::env;
use std::fs;

use glsl::parser::Parse as _;
use glsl::syntax::ShaderStage;
use glsl::syntax::TranslationUnit;

use glsl::syntax::{
    ArraySpecifierDimension, Block, Expr, SingleDeclaration, StorageQualifier, StructFieldSpecifier,
};
use glsl::visitor::{Host, Visit, Visitor};

#[derive(Debug, Default)]
struct VarInfo {
    name: String,
    storage: Option<String>,
    type_name: String,
    array: Option<Vec<usize>>,
    // TODO: interpolation(flat)
}
impl Visitor for VarInfo {
    fn visit_storage_qualifier(&mut self, qualifier: &StorageQualifier) -> Visit {
        self.storage = Some(format!("{:?}", qualifier));
        Visit::Parent
    }
}

#[derive(Debug, Default)]
struct FieldInfo {
    name: String,
    type_name: String,
    array: Option<Vec<usize>>,
    // TODO: interpolation(flat)
}

#[derive(Debug, Default)]
struct BlockInfo {
    name: String,
    fields: Vec<FieldInfo>,
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

#[derive(Debug, Default)]
struct ShaderInfo {
    vars: Vec<VarInfo>,
    blocks: Vec<BlockInfo>,
    inputs: usize,
    outputs: usize,
    uniforms: usize,
}
impl Visitor for ShaderInfo {
    /*
    We should visit the top-level nodes of interest and then search their
    children from the visit_* methods.
    */
    fn visit_single_declaration(&mut self, declaration: &SingleDeclaration) -> Visit {
        /*
        called for any var declaration, including top-level uniforms and const,
        but not for block defs
        */
        if declaration.name.is_some() {
            let mut info: VarInfo = Default::default();
            declaration.visit(&mut info);

            match info.storage.as_ref().map(String::as_ref) {
                Some("In") => self.inputs += 1,
                Some("Out") => self.outputs += 1,
                Some("Uniform") => self.uniforms += 1,
                _ => (),
            }

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

            self.vars.push(info);
        }
        Visit::Parent
    }

    fn visit_block(&mut self, block: &Block) -> Visit {
        /*
        we treat blocks as defining a type as well as an instance of that type
        */
        let mut block_info: BlockInfo = Default::default();
        block.visit(&mut block_info);

        if let Some(identifier) = &block.identifier {
            block_info.name = block.name.as_str().to_owned();

            let mut var_info: VarInfo = Default::default();

            var_info.name = identifier.ident.as_str().to_owned();
            var_info.type_name = block.name.as_str().to_owned();

            block.visit(&mut var_info);

            match var_info.storage.as_ref().map(String::as_ref) {
                Some("In") => self.inputs += 1,
                Some("Out") => self.outputs += 1,
                Some("Uniform") => self.uniforms += 1,
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
            // all the field names are top-level vars
        }

        self.blocks.push(block_info);
        Visit::Parent
    }
}

fn parse_stage(
    filename: &String,
) -> std::result::Result<TranslationUnit, glsl::parser::ParseError> {
    let contents = fs::read_to_string(filename).expect("Unable to read the file");
    let stage = ShaderStage::parse(contents);
    assert!(stage.is_ok());
    return stage;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let result = parse_stage(filename);
    let stage = match result {
        Ok(parsed) => parsed,
        Err(error) => panic!("Problem parsing the file: {:?}", error),
    };

    let mut info: ShaderInfo = Default::default();
    stage.visit(&mut info);

    println!("{} variables declared", info.vars.len());
    println!(
        "--> {:?}",
        info.vars
            .iter()
            .map(|var| &var.name)
            .collect::<Vec<&String>>()
    );
    println!("--> {:?}", info.vars);
    println!("{} blocks declared", info.blocks.len());
    println!(
        "--> {:?}",
        info.blocks
            .iter()
            .map(|var| &var.name)
            .collect::<Vec<&String>>()
    );
    println!("--> {:?}", info.blocks);
    println!("{} inputs declared", info.inputs);
    println!("{} outputs declared", info.outputs);
    println!("{} uniforms declared", info.uniforms);

    // println!("{:?}", stage);
}
