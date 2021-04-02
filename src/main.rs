use std::env;
use std::fmt::Debug;
use std::fs;

use glsl_shaderinfo::{get_info, Declaration};

mod glsl_shaderinfo;

fn print_declarations<T: Declaration + Debug>(declarations: Vec<T>, label: &str) {
    let count = declarations.len();
    let pluralised: String;
    match count {
        i if i != 1 => pluralised = format!("{}s", label),
        _ => pluralised = label.to_string(),
    }
    println!("{} {} declared", count, pluralised);
    if count > 0 {
        println!(
            "--> {:?}",
            declarations
                .iter()
                .map(|var| var.get_name())
                .collect::<Vec<&String>>()
        );
        println!("--> {:?}", declarations);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Unable to read the file");
    let info = get_info(&contents);

    println!("GLSL version: {}", info.version_str);

    print_declarations(info.vars, "variable");
    print_declarations(info.blocks, "block");
    print_declarations(info.inputs, "input");
    print_declarations(info.outputs, "output");
    print_declarations(info.uniforms, "uniform");

    // println!("{:?}", stage);
}
