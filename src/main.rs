use std::fmt::Debug;
use std::fs;

use argparse::{ArgumentParser, Store};

use crate::glsl_shaderinfo::{get_info, Declaration};
use crate::glsl_shaderinfo::utils::{get_names, pluralise};

// mod glsl_shaderinfo;

fn print_declarations<T: Declaration + Debug>(declarations: &Vec<T>, label: &str) {
    let count = declarations.len();
    let pluralised = pluralise(&label, count);
    println!("{} {} declared", count, pluralised);
    if count > 0 {
        println!("--> {:?}", get_names(declarations));
        println!("--> {:?}", declarations);
    }
}

fn main() {
    let mut filename = String::new();
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("Prints info about vars used in a GLSL shader.");
        parser
            .refer(&mut filename)
            .add_argument("filename", Store, "GLSL shader file to parse info from.")
            .required();
        parser.parse_args_or_exit();
    }

    let _err = format!("Unable to read the file: {}", filename);
    let contents = fs::read_to_string(filename).expect(_err.as_str());
    let info = get_info(&contents);

    println!("GLSL version: {}", info.version_str);

    print_declarations(&info.vars, "variable");
    print_declarations(&info.blocks, "block");
    print_declarations(&info.inputs, "input");
    print_declarations(&info.outputs, "output");
    print_declarations(&info.uniforms, "uniform");

    // println!("{:?}", stage);
}
