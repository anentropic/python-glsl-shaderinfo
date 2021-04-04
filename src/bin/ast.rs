use std::fs;

use argparse::{ArgumentParser, Store};
use glsl::parser::Parse as _;
use glsl::syntax::ShaderStage;

fn main() {
    /*
    Just pretty-prints the AST parsed by `glsl`
    */
    let mut filename = String::new();
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("Prints the AST parsed from a GLSL shader.");
        parser
            .refer(&mut filename)
            .add_argument("filename", Store, "GLSL shader file to parse.")
            .required();
        parser.parse_args_or_exit();
    }

    let _err = format!("Unable to read the file: {}", filename);
    let contents = fs::read_to_string(filename).expect(_err.as_str());
    let result = ShaderStage::parse(contents);

    let shader = match result {
        Ok(parsed) => parsed,
        Err(error) => panic!("Problem parsing the file: {:?}", error),
    };

    println!("{:#?}", shader);
}
