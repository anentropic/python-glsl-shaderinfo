use std::collections::HashMap;
use std::env;
use std::fs;

use glsl::parser::Parse as _;
use glsl::syntax::ShaderStage;
use glsl::syntax::TranslationUnit;

use glsl::syntax::{
    ArraySpecifier, ArraySpecifierDimension, Block, Expr, SingleDeclaration,
    StorageQualifier, TypeSpecifier
};
use glsl::visitor::{Host, Visit, Visitor};

fn parse_stage(
    filename: &String,
) -> std::result::Result<TranslationUnit, glsl::parser::ParseError> {
    let contents = fs::read_to_string(filename).expect("Unable to read the file");
    let stage = ShaderStage::parse(contents);
    assert!(stage.is_ok());
    return stage;
}

#[derive(Debug)]
#[derive(Default)]
struct VarInfo {
    name: String,
    storage: Option<String>,
    type_name: String,
    array: Option<Vec<usize>>,
}
impl Visitor for VarInfo {
    fn visit_storage_qualifier(&mut self, qualifier: &StorageQualifier) -> Visit {
        self.storage = Some(format!("{:?}", qualifier));
        Visit::Parent
    }

    fn visit_type_specifier(&mut self, type_spec: &TypeSpecifier) -> Visit {
    	/*
    	see:
    	https://docs.rs/glsl/6.0.0/glsl/syntax/enum.TypeSpecifierNonArray.html
    	*/
        self.type_name = format!("{:?}", type_spec.ty);
        Visit::Parent
    }

    fn visit_array_specifier(&mut self, specifier: &ArraySpecifier) -> Visit {
        /*
        applies to vars but not to blocks

        e.g.
            array_specifier: Some(
                ArraySpecifier {
                    dimensions: NonEmpty([ExplicitlySized(IntConst(12))])
                }
            ),

        Expr::IntConst(x)
        */
        // I think GLSL can only have 1D array vars...
        let spec_dim = &specifier.dimensions.0[0];
        match spec_dim {
            ArraySpecifierDimension::ExplicitlySized(value) => match **value {
                Expr::IntConst(x) => self.array = Some(vec![x as usize]),
                Expr::UIntConst(x) => self.array = Some(vec![x as usize]),
                _ => self.array = None  // we only support int consts for array dims
            },
            ArraySpecifierDimension::Unsized => self.array = Some(vec![]),
        }
        Visit::Parent
    }
}

#[derive(Debug)]
#[derive(Default)]
struct ShaderInfo {
    vars: HashMap<String, VarInfo>,
    blocks: Vec<String>,
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
        	info.name = declaration.name.as_ref().unwrap().as_str().to_owned();
        	declaration.visit(&mut info);

        	match info.storage.as_ref().map(String::as_ref) {
        		Some("In") => self.inputs += 1,
        		Some("Out") => self.outputs += 1,
        		Some("Uniform") => self.uniforms += 1,
        		_ => ()
        	}

            self.vars
                .insert(
                	declaration.name.as_ref().unwrap().as_str().to_owned(),
                	info
                );
        }
        Visit::Children
    }

    fn visit_block(&mut self, block: &Block) -> Visit {
        if block.qualifier.qualifiers.0.len() > 0 {
            self.blocks.push(block.name.as_str().to_owned());
            // self.vars.push(block.name.as_str().to_owned());
        }
        Visit::Children
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let result = parse_stage(filename);
    let stage = match result {
        Ok(parsed) => parsed,
        Err(error) => panic!("Problem parsing the file: {:?}", error),
    };

    let mut counter: ShaderInfo = Default::default();
    stage.visit(&mut counter);

    println!("{} variables declared", counter.vars.len());
    println!("--> {:?}", counter.vars);
    println!("{} blocks declared", counter.blocks.len());
    println!("--> {:?}", counter.blocks);
    println!("{} inputs declared", counter.inputs);
    println!("{} outputs declared", counter.outputs);
    println!("{} uniforms declared", counter.uniforms);

    // println!("{:?}", stage);
}
