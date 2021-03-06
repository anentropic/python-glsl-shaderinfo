from enum import Enum


class StorageQualifier(str, Enum):
    CONST = "Const"
    IN_OUT = "InOut"
    IN = "In"
    OUT = "Out"
    CENTROID = "Centroid"
    PATCH = "Patch"
    SAMPLE = "Sample"
    UNIFORM = "Uniform"
    ATTRIBUTE = "Attribute"
    VARYING = "Varying"
    BUFFER = "Buffer"
    SHARED = "Shared"
    COHERENT = "Coherent"
    VOLATILE = "Volatile"
    RESTRICT = "Restrict"
    READ_ONLY = "ReadOnly"
    WRITE_ONLY = "WriteOnly"
    SUBROUTINE = "Subroutine"  # TODO: should be parameterised (Vec<TypeName>)


class TypeSpecifier(str, Enum):
    VOID = "Void"
    BOOL = "Bool"
    INT = "Int"
    UINT = "UInt"
    FLOAT = "Float"
    DOUBLE = "Double"
    VEC2 = "Vec2"
    VEC3 = "Vec3"
    VEC4 = "Vec4"
    DVEC2 = "DVec2"
    DVEC3 = "DVec3"
    DVEC4 = "DVec4"
    BVEC2 = "BVec2"
    BVEC3 = "BVec3"
    BVEC4 = "BVec4"
    IVEC2 = "IVec2"
    IVEC3 = "IVec3"
    IVEC4 = "IVec4"
    UVEC2 = "UVec2"
    UVEC3 = "UVec3"
    UVEC4 = "UVec4"
    MAT2 = "Mat2"
    MAT3 = "Mat3"
    MAT4 = "Mat4"
    MAT23 = "Mat23"
    MAT24 = "Mat24"
    MAT32 = "Mat32"
    MAT34 = "Mat34"
    MAT42 = "Mat42"
    MAT43 = "Mat43"
    DMAT2 = "DMat2"
    DMAT3 = "DMat3"
    DMAT4 = "DMat4"
    DMAT23 = "DMat23"
    DMAT24 = "DMat24"
    DMAT32 = "DMat32"
    DMAT34 = "DMat34"
    DMAT42 = "DMat42"
    DMAT43 = "DMat43"
    SAMPLER1D = "Sampler1D"
    IMAGE1D = "Image1D"
    SAMPLER2D = "Sampler2D"
    IMAGE2D = "Image2D"
    SAMPLER3D = "Sampler3D"
    IMAGE3D = "Image3D"
    SAMPLERCUBE = "SamplerCube"
    IMAGECUBE = "ImageCube"
    SAMPLER2DRECT = "Sampler2DRect"
    IMAGE2DRECT = "Image2DRect"
    SAMPLER1DARRAY = "Sampler1DArray"
    IMAGE1DARRAY = "Image1DArray"
    SAMPLER2DARRAY = "Sampler2DArray"
    IMAGE2DARRAY = "Image2DArray"
    SAMPLERBUFFER = "SamplerBuffer"
    IMAGEBUFFER = "ImageBuffer"
    SAMPLER2DMS = "Sampler2DMS"
    IMAGE2DMS = "Image2DMS"
    SAMPLER2DMSARRAY = "Sampler2DMSArray"
    IMAGE2DMSARRAY = "Image2DMSArray"
    SAMPLERCUBEARRAY = "SamplerCubeArray"
    IMAGECUBEARRAY = "ImageCubeArray"
    SAMPLER1DSHADOW = "Sampler1DShadow"
    SAMPLER2DSHADOW = "Sampler2DShadow"
    SAMPLER2DRECTSHADOW = "Sampler2DRectShadow"
    SAMPLER1DARRAYSHADOW = "Sampler1DArrayShadow"
    SAMPLER2DARRAYSHADOW = "Sampler2DArrayShadow"
    SAMPLERCUBESHADOW = "SamplerCubeShadow"
    SAMPLERCUBEARRAYSHADOW = "SamplerCubeArrayShadow"
    ISAMPLER1D = "ISampler1D"
    IIMAGE1D = "IImage1D"
    ISAMPLER2D = "ISampler2D"
    IIMAGE2D = "IImage2D"
    ISAMPLER3D = "ISampler3D"
    IIMAGE3D = "IImage3D"
    ISAMPLERCUBE = "ISamplerCube"
    IIMAGECUBE = "IImageCube"
    ISAMPLER2DRECT = "ISampler2DRect"
    IIMAGE2DRECT = "IImage2DRect"
    ISAMPLER1DARRAY = "ISampler1DArray"
    IIMAGE1DARRAY = "IImage1DArray"
    ISAMPLER2DARRAY = "ISampler2DArray"
    IIMAGE2DARRAY = "IImage2DArray"
    ISAMPLERBUFFER = "ISamplerBuffer"
    IIMAGEBUFFER = "IImageBuffer"
    ISAMPLER2DMS = "ISampler2DMS"
    IIMAGE2DMS = "IImage2DMS"
    ISAMPLER2DMSARRAY = "ISampler2DMSArray"
    IIMAGE2DMSARRAY = "IImage2DMSArray"
    ISAMPLERCUBEARRAY = "ISamplerCubeArray"
    IIMAGECUBEARRAY = "IImageCubeArray"
    ATOMICUINT = "AtomicUInt"
    USAMPLER1D = "USampler1D"
    UIMAGE1D = "UImage1D"
    USAMPLER2D = "USampler2D"
    UIMAGE2D = "UImage2D"
    USAMPLER3D = "USampler3D"
    UIMAGE3D = "UImage3D"
    USAMPLERCUBE = "USamplerCube"
    UIMAGECUBE = "UImageCube"
    USAMPLER2DRECT = "USampler2DRect"
    UIMAGE2DRECT = "UImage2DRect"
    USAMPLER1DARRAY = "USampler1DArray"
    UIMAGE1DARRAY = "UImage1DArray"
    USAMPLER2DARRAY = "USampler2DArray"
    UIMAGE2DARRAY = "UImage2DArray"
    USAMPLERBUFFER = "USamplerBuffer"
    UIMAGEBUFFER = "UImageBuffer"
    USAMPLER2DMS = "USampler2DMS"
    UIMAGE2DMS = "UImage2DMS"
    USAMPLER2DMSARRAY = "USampler2DMSArray"
    UIMAGE2DMSARRAY = "UImage2DMSArray"
    USAMPLERCUBEARRAY = "USamplerCubeArray"
    UIMAGECUBEARRAY = "UImageCubeArray"
    STRUCT = "Struct"  # TODO: should be parameterised (StructSpecifier)
    TYPENAME = "TypeName"  # TODO: should be parameterised (TypeName)
