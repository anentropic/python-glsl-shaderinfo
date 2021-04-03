# glsl-shaderinfo

`glsl-shaderinfo` aims to parse shader files written in the GLSL language and extract useful info about declared vars, inputs outputs, uniforms etc.


### Implementation info

`glsl-shaderinfo` is a Python library implemented primarily in Rust, using the PyO3 + Maturin toolchain.

* https://github.com/PyO3/pyo3
* https://github.com/PyO3/maturin

We are using the GLSL parser crate here:  

* https://crates.io/crates/glsl/
* https://github.com/phaazon/glsl

It seems fairly robust and actively maintained (2021-04). Supports up to GLSL450/GLSL460.


### Development setup

Install `rustup`: https://rustup.rs/

Install `poetry`: https://python-poetry.org/docs/#installation

Clone this repo.

```
poetry shell
poetry install
maturin develop
```

`maturin` is installed as a python library and is used as the rust-python-lib build tool.

`maturin develop` builds the python module and installs it in the current virtualenv.


### Usage

Now we can:
```pycon
$ ipython
Python 3.9.2 (default, Feb 23 2021, 12:16:37)
Type 'copyright', 'credits' or 'license' for more information
IPython 7.22.0 -- An enhanced Interactive Python. Type '?' for help.

In [1]: from glsl_shaderinfo import get_info

In [2]: with open("../modernglproj/resources/shaders/myproj_geometry.glsl") as f:
   ...:     src = f.read()
   ...:

In [3]: info = get_info(src)

In [4]: info.inputs
Out[4]: [<VarInfo in VS_OUT "gs_in">]

In [5]: info.blocks
Out[5]:
{'GS_OUT': <BlockInfo "GS_OUT" (3 fields)>,
 'VS_OUT': <BlockInfo "VS_OUT" (3 fields)>}

In [7]: info.blocks[info.inputs[0].type_specifier].fields
Out[7]:
[<FieldInfo vec2 "gridCoord">,
 <FieldInfo vec3 "pos">,
 <FieldInfo int "textureId">]

In [8]: info.uniforms
Out[8]: [<VarInfo uniform float "size">, <VarInfo uniform mat4 "projection">]

In [9]: info.uniforms[0].storage
Out[9]: <StorageQualifier.UNIFORM: 'Uniform'>

In [10]: info.uniforms[0].type_specifier
Out[10]: <TypeSpecifier.FLOAT: 'Float'>

In [11]: info
Out[11]: <ShaderInfo for GLSL: 330 (1 in, 1 out)>
```


## TODO

* comprehensive tests, any tests...
	* here are some to copy https://github.com/graphitemaster/glsl-parser/tree/main/tests
	* also all the example programs from `moderngl`
* what to do with uniform blocks
* what to do with layouts i.e. https://www.khronos.org/opengl/wiki/Layout_Qualifier_(GLSL)#Shader_stage_options
* publish to PyPI
* docs
* docstrings:
	* on the Rust side: https://doc.rust-lang.org/stable/book/ch14-02-publishing-to-crates-io.html#making-useful-documentation-comments
	* PyO3 should bring them across: https://pyo3.rs/v0.13.2/module.html#documentation

### Future directions

I have basically no Rust experience so the code here is probably awful, but it does work at least. I will try and improve it as I learn more.

The most flexible option for future use cases would be to re-export the whole of the `glsl` crate interface into Python types and modules, i.e. expose the full parser and AST. Then we could build AST visitors for abitrary use cases in Python.  Possibly we can use https://serde.rs/remote-derive.html to shadow the types from `glsl` and dump them to Python primitives. See also https://docs.rs/pythonize/0.13.0/pythonize/ and possibly https://github.com/gperinazzo/dict-derive

But currently the AST visitors are built in the Rust side and provide a general meta info about declared variables, and we export this much smaller interface (basically just a single `get_info` method and some types) into Python.

You can see examples of the AST structure at e.g.:  
https://github.com/anentropic/python-glsl-shaderinfo/blob/main/hexgrid_geometry.glsl-ast

(I have manually formatted parts of it for readablility, the raw output is without line-breaks)
