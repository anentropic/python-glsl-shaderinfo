use std::fmt::Debug;

use crate::glsl_shaderinfo::Declaration;

pub fn get_names<T: Declaration + Debug>(declarations: &Vec<T>) -> Vec<String> {
    declarations
        .iter()
        .map(|var| (var.get_name()).to_string())
        .collect::<Vec<String>>()
}

pub fn pluralise(prefix: &str, count: usize) -> String {
    match count {
        i if i != 1 => format!("{}s", prefix),
        _ => prefix.to_string(),
    }
}
