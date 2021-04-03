# glsl-shaderinfo

This is built on top of the Rust crate: https://crates.io/crates/glsl/

To run a demo, pass a path to a GLSL source file:
```
cargo run myshader.glsl
```
Or:
```
cargo build
target/debug/shaderinfo myshader.glsl
```
(currently broken, seems something to do with PyO3)

You should see output like:
```
GLSL version: 330
13 variables declared
--> ["rows", "cols", "size", "root_3", "half_root_3", "textureId", "gridCoord", "vs_out", "col", "row", "x_offset", "x", "y"]
--> [VarInfo { name: "rows", storage: Some("Uniform"), type_name: "Int", array: None }, VarInfo { name: "cols", storage: Some("Uniform"), type_name: "Int", array: None }, VarInfo { name: "size", storage: Some("Uniform"), type_name: "Float", array: None }, VarInfo { name: "root_3", storage: Some("Const"), type_name: "Float", array: None }, VarInfo { name: "half_root_3", storage: Some("Const"), type_name: "Float", array: None }, VarInfo { name: "textureId", storage: Some("In"), type_name: "Float", array: None }, VarInfo { name: "gridCoord", storage: Some("In"), type_name: "Vec2", array: None }, VarInfo { name: "vs_out", storage: Some("Out"), type_name: "VS_OUT", array: None }, VarInfo { name: "col", storage: None, type_name: "Int", array: None }, VarInfo { name: "row", storage: None, type_name: "Int", array: None }, VarInfo { name: "x_offset", storage: None, type_name: "Float", array: None }, VarInfo { name: "x", storage: None, type_name: "Float", array: None }, VarInfo { name: "y", storage: None, type_name: "Float", array: None }]
1 block declared
--> ["VS_OUT"]
--> [BlockInfo { name: "VS_OUT", fields: [FieldInfo { name: "gridCoord", type_name: "Vec2", array: None }, FieldInfo { name: "pos", type_name: "Vec3", array: None }, FieldInfo { name: "textureId", type_name: "Int", array: None }] }]
2 inputs declared
--> ["textureId", "gridCoord"]
--> [VarInfo { name: "textureId", storage: Some("In"), type_name: "Float", array: None }, VarInfo { name: "gridCoord", storage: Some("In"), type_name: "Vec2", array: None }]
1 output declared
--> ["vs_out"]
--> [VarInfo { name: "vs_out", storage: Some("Out"), type_name: "VS_OUT", array: None }]
3 uniforms declared
--> ["rows", "cols", "size"]
--> [VarInfo { name: "rows", storage: Some("Uniform"), type_name: "Int", array: None }, VarInfo { name: "cols", storage: Some("Uniform"), type_name: "Int", array: None }, VarInfo { name: "size", storage: Some("Uniform"), type_name: "Float", array: None }]
```

## TODO

* comprehensive tests, any tests...
* what to do with uniform blocks
* what to do with layouts i.e. https://www.khronos.org/opengl/wiki/Layout_Qualifier_(GLSL)#Shader_stage_options
* maybe translate across the enums into python side, or defer the stringification
	* see https://gitter.im/PyO3/Lobby?at=60684b35d765936399d00dd5
* docs
* docstrings:
	* on the Rust side: https://doc.rust-lang.org/stable/book/ch14-02-publishing-to-crates-io.html#making-useful-documentation-comments
	* PyO3 should bring them across: https://pyo3.rs/v0.13.2/module.html#documentation

The most flexible option for future use cases would be to export the whole of the `glsl` crate interface to Python modules and build the visitors in Python.  Possibly we can use https://serde.rs/remote-derive.html to shadow the types from `glsl` and dump them to Python primitives. See also https://docs.rs/pythonize/0.13.0/pythonize/ and possibly https://github.com/gperinazzo/dict-derive

But currently the AST visitors are built in the Rust side and provide a general meta info about declared variables, and we export this much smaller interface (basically just a single `get_info` method and some types).
