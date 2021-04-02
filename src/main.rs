use std::env;
use std::fs;

use glsl::parser::Parse as _;
use glsl::syntax::ShaderStage;
use glsl::syntax::TranslationUnit;

use glsl::syntax::{Block, SingleDeclaration, StorageQualifier};
use glsl::visitor::{Host, Visit, Visitor};


fn parse_stage(filename: &String) -> std::result::Result<TranslationUnit, glsl::parser::ParseError> {
	let contents = fs::read_to_string(filename)
		.expect("Unable to read the file");
	let stage = ShaderStage::parse(contents);
	assert!(stage.is_ok());
	return stage;
}


// our visitor that will count the number of variables it saw
struct Counter {
  vars: usize,
  blocks: usize,
  inputs: usize,
  outputs: usize,
}

impl Visitor for Counter {
  fn visit_single_declaration(&mut self, declaration: &SingleDeclaration) -> Visit {
    if declaration.name.is_some() {
      self.vars += 1;
    }
    Visit::Children
  }

  fn visit_block(&mut self, block: &Block) -> Visit {
    if block.qualifier.qualifiers.0.len() > 0 {
      self.blocks += 1;
    }
    Visit::Children
  }

  fn visit_storage_qualifier(&mut self, qualifier: &StorageQualifier) -> Visit {
    match qualifier {
        StorageQualifier::In => self.inputs += 1,
        StorageQualifier::Out => self.outputs += 1,
        StorageQualifier::InOut => {
        	self.inputs += 1;
        	self.outputs += 1;
        },
        _ => (),
    }
    Visit::Parent
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

	let mut counter = Counter {
		vars: 0,
		blocks: 0,
		inputs: 0,
		outputs: 0,
	};
	stage.visit(&mut counter);

	println!("{} variables declared", counter.vars);
	println!("{} blocks declared", counter.blocks);
	println!("{} inputs declared", counter.inputs);
	println!("{} outputs declared", counter.outputs);

	// println!("{:?}", stage);
}
