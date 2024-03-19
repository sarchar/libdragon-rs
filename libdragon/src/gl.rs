#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::*;

// GL
pub const BYTE: i32 = libdragon_sys::GL_BYTE as i32;
pub const UNSIGNED_BYTE: i32 = libdragon_sys::GL_UNSIGNED_BYTE as i32;
pub const SHORT: i32 = libdragon_sys::GL_SHORT as i32;
pub const UNSIGNED_SHORT: i32 = libdragon_sys::GL_UNSIGNED_SHORT as i32;
pub const INT: i32 = libdragon_sys::GL_INT as i32;
pub const UNSIGNED_INT: i32 = libdragon_sys::GL_UNSIGNED_INT as i32;
pub const FLOAT: i32 = libdragon_sys::GL_FLOAT as i32;
pub const DOUBLE: i32 = libdragon_sys::GL_DOUBLE as i32;
pub const HALF_FIXED_N64: i32 = libdragon_sys::GL_HALF_FIXED_N64 as i32;
pub const NO_ERROR: i32 = libdragon_sys::GL_NO_ERROR as i32;
pub const INVALID_ENUM: i32 = libdragon_sys::GL_INVALID_ENUM as i32;
pub const INVALID_VALUE: i32 = libdragon_sys::GL_INVALID_VALUE as i32;
pub const INVALID_OPERATION: i32 = libdragon_sys::GL_INVALID_OPERATION as i32;
pub const STACK_OVERFLOW: i32 = libdragon_sys::GL_STACK_OVERFLOW as i32;
pub const STACK_UNDERFLOW: i32 = libdragon_sys::GL_STACK_UNDERFLOW as i32;
pub const OUT_OF_MEMORY: i32 = libdragon_sys::GL_OUT_OF_MEMORY as i32;
pub const DITHER: i32 = libdragon_sys::GL_DITHER as i32;
pub const POINTS: i32 = libdragon_sys::GL_POINTS as i32;
pub const LINES: i32 = libdragon_sys::GL_LINES as i32;
pub const LINE_LOOP: i32 = libdragon_sys::GL_LINE_LOOP as i32;
pub const LINE_STRIP: i32 = libdragon_sys::GL_LINE_STRIP as i32;
pub const TRIANGLES: i32 = libdragon_sys::GL_TRIANGLES as i32;
pub const TRIANGLE_STRIP: i32 = libdragon_sys::GL_TRIANGLE_STRIP as i32;
pub const TRIANGLE_FAN: i32 = libdragon_sys::GL_TRIANGLE_FAN as i32;
pub const QUADS: i32 = libdragon_sys::GL_QUADS as i32;
pub const QUAD_STRIP: i32 = libdragon_sys::GL_QUAD_STRIP as i32;
pub const POLYGON: i32 = libdragon_sys::GL_POLYGON as i32;
pub const NORMALIZE: i32 = libdragon_sys::GL_NORMALIZE as i32;
pub const CURRENT_COLOR: i32 = libdragon_sys::GL_CURRENT_COLOR as i32;
pub const CURRENT_INDEX: i32 = libdragon_sys::GL_CURRENT_INDEX as i32;
pub const CURRENT_NORMAL: i32 = libdragon_sys::GL_CURRENT_NORMAL as i32;
pub const CURRENT_TEXTURE_COORDS: i32 = libdragon_sys::GL_CURRENT_TEXTURE_COORDS as i32;
pub const CURRENT_RASTER_COLOR: i32 = libdragon_sys::GL_CURRENT_RASTER_COLOR as i32;
pub const CURRENT_RASTER_INDEX: i32 = libdragon_sys::GL_CURRENT_RASTER_INDEX as i32;
pub const CURRENT_RASTER_TEXTURE_COORDS: i32 = libdragon_sys::GL_CURRENT_RASTER_TEXTURE_COORDS as i32;
pub const CURRENT_RASTER_POSITION: i32 = libdragon_sys::GL_CURRENT_RASTER_POSITION as i32;
pub const CURRENT_RASTER_POSITION_VALID: i32 = libdragon_sys::GL_CURRENT_RASTER_POSITION_VALID as i32;
pub const CURRENT_RASTER_DISTANCE: i32 = libdragon_sys::GL_CURRENT_RASTER_DISTANCE as i32;
pub const EDGE_FLAG: i32 = libdragon_sys::GL_EDGE_FLAG as i32;
pub const VERTEX_HALF_FIXED_PRECISION_N64: i32 = libdragon_sys::GL_VERTEX_HALF_FIXED_PRECISION_N64 as i32;
pub const TEXTURE_COORD_HALF_FIXED_PRECISION_N64: i32 = libdragon_sys::GL_TEXTURE_COORD_HALF_FIXED_PRECISION_N64 as i32;
pub const VERTEX_ARRAY: i32 = libdragon_sys::GL_VERTEX_ARRAY as i32;
pub const NORMAL_ARRAY: i32 = libdragon_sys::GL_NORMAL_ARRAY as i32;
pub const COLOR_ARRAY: i32 = libdragon_sys::GL_COLOR_ARRAY as i32;
pub const INDEX_ARRAY: i32 = libdragon_sys::GL_INDEX_ARRAY as i32;
pub const TEXTURE_COORD_ARRAY: i32 = libdragon_sys::GL_TEXTURE_COORD_ARRAY as i32;
pub const EDGE_FLAG_ARRAY: i32 = libdragon_sys::GL_EDGE_FLAG_ARRAY as i32;
pub const V2F: i32 = libdragon_sys::GL_V2F as i32;
pub const V3F: i32 = libdragon_sys::GL_V3F as i32;
pub const C4UB_V2F: i32 = libdragon_sys::GL_C4UB_V2F as i32;
pub const C4UB_V3F: i32 = libdragon_sys::GL_C4UB_V3F as i32;
pub const C3F_V3F: i32 = libdragon_sys::GL_C3F_V3F as i32;
pub const N3F_V3F: i32 = libdragon_sys::GL_N3F_V3F as i32;
pub const C4F_N3F_V3F: i32 = libdragon_sys::GL_C4F_N3F_V3F as i32;
pub const T2F_V3F: i32 = libdragon_sys::GL_T2F_V3F as i32;
pub const T4F_V4F: i32 = libdragon_sys::GL_T4F_V4F as i32;
pub const T2F_C4UB_V3F: i32 = libdragon_sys::GL_T2F_C4UB_V3F as i32;
pub const T2F_C3F_V3F: i32 = libdragon_sys::GL_T2F_C3F_V3F as i32;
pub const T2F_N3F_V3F: i32 = libdragon_sys::GL_T2F_N3F_V3F as i32;
pub const T2F_C4F_N3F_V3F: i32 = libdragon_sys::GL_T2F_C4F_N3F_V3F as i32;
pub const T4F_C4F_N3F_V4F: i32 = libdragon_sys::GL_T4F_C4F_N3F_V4F as i32;
pub const VERTEX_ARRAY_SIZE: i32 = libdragon_sys::GL_VERTEX_ARRAY_SIZE as i32;
pub const VERTEX_ARRAY_TYPE: i32 = libdragon_sys::GL_VERTEX_ARRAY_TYPE as i32;
pub const VERTEX_ARRAY_STRIDE: i32 = libdragon_sys::GL_VERTEX_ARRAY_STRIDE as i32;
pub const NORMAL_ARRAY_TYPE: i32 = libdragon_sys::GL_NORMAL_ARRAY_TYPE as i32;
pub const NORMAL_ARRAY_STRIDE: i32 = libdragon_sys::GL_NORMAL_ARRAY_STRIDE as i32;
pub const COLOR_ARRAY_SIZE: i32 = libdragon_sys::GL_COLOR_ARRAY_SIZE as i32;
pub const COLOR_ARRAY_TYPE: i32 = libdragon_sys::GL_COLOR_ARRAY_TYPE as i32;
pub const COLOR_ARRAY_STRIDE: i32 = libdragon_sys::GL_COLOR_ARRAY_STRIDE as i32;
pub const INDEX_ARRAY_TYPE: i32 = libdragon_sys::GL_INDEX_ARRAY_TYPE as i32;
pub const INDEX_ARRAY_STRIDE: i32 = libdragon_sys::GL_INDEX_ARRAY_STRIDE as i32;
pub const TEXTURE_COORD_ARRAY_SIZE: i32 = libdragon_sys::GL_TEXTURE_COORD_ARRAY_SIZE as i32;
pub const TEXTURE_COORD_ARRAY_TYPE: i32 = libdragon_sys::GL_TEXTURE_COORD_ARRAY_TYPE as i32;
pub const TEXTURE_COORD_ARRAY_STRIDE: i32 = libdragon_sys::GL_TEXTURE_COORD_ARRAY_STRIDE as i32;
pub const EDGE_FLAG_ARRAY_STRIDE: i32 = libdragon_sys::GL_EDGE_FLAG_ARRAY_STRIDE as i32;
pub const VERTEX_ARRAY_POINTER: i32 = libdragon_sys::GL_VERTEX_ARRAY_POINTER as i32;
pub const NORMAL_ARRAY_POINTER: i32 = libdragon_sys::GL_NORMAL_ARRAY_POINTER as i32;
pub const COLOR_ARRAY_POINTER: i32 = libdragon_sys::GL_COLOR_ARRAY_POINTER as i32;
pub const INDEX_ARRAY_POINTER: i32 = libdragon_sys::GL_INDEX_ARRAY_POINTER as i32;
pub const TEXTURE_COORD_ARRAY_POINTER: i32 = libdragon_sys::GL_TEXTURE_COORD_ARRAY_POINTER as i32;
pub const EDGE_FLAG_ARRAY_POINTER: i32 = libdragon_sys::GL_EDGE_FLAG_ARRAY_POINTER as i32;
pub const ARRAY_BUFFER_ARB: i32 = libdragon_sys::GL_ARRAY_BUFFER_ARB as i32;
pub const ELEMENT_ARRAY_BUFFER_ARB: i32 = libdragon_sys::GL_ELEMENT_ARRAY_BUFFER_ARB as i32;
pub const ARRAY_BUFFER_BINDING_ARB: i32 = libdragon_sys::GL_ARRAY_BUFFER_BINDING_ARB as i32;
pub const ELEMENT_ARRAY_BUFFER_BINDING_ARB: i32 = libdragon_sys::GL_ELEMENT_ARRAY_BUFFER_BINDING_ARB as i32;
pub const VERTEX_ARRAY_BUFFER_BINDING_ARB: i32 = libdragon_sys::GL_VERTEX_ARRAY_BUFFER_BINDING_ARB as i32;
pub const NORMAL_ARRAY_BUFFER_BINDING_ARB: i32 = libdragon_sys::GL_NORMAL_ARRAY_BUFFER_BINDING_ARB as i32;
pub const COLOR_ARRAY_BUFFER_BINDING_ARB: i32 = libdragon_sys::GL_COLOR_ARRAY_BUFFER_BINDING_ARB as i32;
pub const INDEX_ARRAY_BUFFER_BINDING_ARB: i32 = libdragon_sys::GL_INDEX_ARRAY_BUFFER_BINDING_ARB as i32;
pub const TEXTURE_COORD_ARRAY_BUFFER_BINDING_ARB: i32 = libdragon_sys::GL_TEXTURE_COORD_ARRAY_BUFFER_BINDING_ARB as i32;
pub const EDGE_FLAG_ARRAY_BUFFER_BINDING_ARB: i32 = libdragon_sys::GL_EDGE_FLAG_ARRAY_BUFFER_BINDING_ARB as i32;
pub const MATRIX_INDEX_ARRAY_BUFFER_BINDING_ARB: i32 = libdragon_sys::GL_MATRIX_INDEX_ARRAY_BUFFER_BINDING_ARB as i32;
pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING_ARB: i32 = libdragon_sys::GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING_ARB as i32;
pub const VERTEX_ARRAY_BINDING: i32 = libdragon_sys::GL_VERTEX_ARRAY_BINDING as i32;
pub const MATRIX_PALETTE_ARB: i32 = libdragon_sys::GL_MATRIX_PALETTE_ARB as i32;
pub const MAX_MATRIX_PALETTE_STACK_DEPTH_ARB: i32 = libdragon_sys::GL_MAX_MATRIX_PALETTE_STACK_DEPTH_ARB as i32;
pub const MAX_PALETTE_MATRICES_ARB: i32 = libdragon_sys::GL_MAX_PALETTE_MATRICES_ARB as i32;
pub const CURRENT_PALETTE_MATRIX_ARB: i32 = libdragon_sys::GL_CURRENT_PALETTE_MATRIX_ARB as i32;
pub const MATRIX_INDEX_ARRAY_ARB: i32 = libdragon_sys::GL_MATRIX_INDEX_ARRAY_ARB as i32;
pub const CURRENT_MATRIX_INDEX_ARB: i32 = libdragon_sys::GL_CURRENT_MATRIX_INDEX_ARB as i32;
pub const MATRIX_INDEX_ARRAY_SIZE_ARB: i32 = libdragon_sys::GL_MATRIX_INDEX_ARRAY_SIZE_ARB as i32;
pub const MATRIX_INDEX_ARRAY_TYPE_ARB: i32 = libdragon_sys::GL_MATRIX_INDEX_ARRAY_TYPE_ARB as i32;
pub const MATRIX_INDEX_ARRAY_STRIDE_ARB: i32 = libdragon_sys::GL_MATRIX_INDEX_ARRAY_STRIDE_ARB as i32;
pub const MATRIX_INDEX_ARRAY_POINTER_ARB: i32 = libdragon_sys::GL_MATRIX_INDEX_ARRAY_POINTER_ARB as i32;
pub const STREAM_DRAW_ARB: i32 = libdragon_sys::GL_STREAM_DRAW_ARB as i32;
pub const STREAM_READ_ARB: i32 = libdragon_sys::GL_STREAM_READ_ARB as i32;
pub const STREAM_COPY_ARB: i32 = libdragon_sys::GL_STREAM_COPY_ARB as i32;
pub const STATIC_DRAW_ARB: i32 = libdragon_sys::GL_STATIC_DRAW_ARB as i32;
pub const STATIC_READ_ARB: i32 = libdragon_sys::GL_STATIC_READ_ARB as i32;
pub const STATIC_COPY_ARB: i32 = libdragon_sys::GL_STATIC_COPY_ARB as i32;
pub const DYNAMIC_DRAW_ARB: i32 = libdragon_sys::GL_DYNAMIC_DRAW_ARB as i32;
pub const DYNAMIC_READ_ARB: i32 = libdragon_sys::GL_DYNAMIC_READ_ARB as i32;
pub const DYNAMIC_COPY_ARB: i32 = libdragon_sys::GL_DYNAMIC_COPY_ARB as i32;
pub const READ_ONLY_ARB: i32 = libdragon_sys::GL_READ_ONLY_ARB as i32;
pub const WRITE_ONLY_ARB: i32 = libdragon_sys::GL_WRITE_ONLY_ARB as i32;
pub const READ_WRITE_ARB: i32 = libdragon_sys::GL_READ_WRITE_ARB as i32;
pub const BUFFER_SIZE_ARB: i32 = libdragon_sys::GL_BUFFER_SIZE_ARB as i32;
pub const BUFFER_USAGE_ARB: i32 = libdragon_sys::GL_BUFFER_USAGE_ARB as i32;
pub const BUFFER_ACCESS_ARB: i32 = libdragon_sys::GL_BUFFER_ACCESS_ARB as i32;
pub const BUFFER_MAPPED_ARB: i32 = libdragon_sys::GL_BUFFER_MAPPED_ARB as i32;
pub const BUFFER_MAP_POINTER_ARB: i32 = libdragon_sys::GL_BUFFER_MAP_POINTER_ARB as i32;
pub const DEPTH_RANGE: i32 = libdragon_sys::GL_DEPTH_RANGE as i32;
pub const VIEWPORT: i32 = libdragon_sys::GL_VIEWPORT as i32;
pub const MAX_VIEWPORT_DIMS: i32 = libdragon_sys::GL_MAX_VIEWPORT_DIMS as i32;
pub const MODELVIEW: i32 = libdragon_sys::GL_MODELVIEW as i32;
pub const PROJECTION: i32 = libdragon_sys::GL_PROJECTION as i32;
pub const TEXTURE: i32 = libdragon_sys::GL_TEXTURE as i32;
pub const MATRIX_MODE: i32 = libdragon_sys::GL_MATRIX_MODE as i32;
pub const MODELVIEW_STACK_DEPTH: i32 = libdragon_sys::GL_MODELVIEW_STACK_DEPTH as i32;
pub const PROJECTION_STACK_DEPTH: i32 = libdragon_sys::GL_PROJECTION_STACK_DEPTH as i32;
pub const TEXTURE_STACK_DEPTH: i32 = libdragon_sys::GL_TEXTURE_STACK_DEPTH as i32;
pub const MODELVIEW_MATRIX: i32 = libdragon_sys::GL_MODELVIEW_MATRIX as i32;
pub const PROJECTION_MATRIX: i32 = libdragon_sys::GL_PROJECTION_MATRIX as i32;
pub const TEXTURE_MATRIX: i32 = libdragon_sys::GL_TEXTURE_MATRIX as i32;
pub const MAX_MODELVIEW_STACK_DEPTH: i32 = libdragon_sys::GL_MAX_MODELVIEW_STACK_DEPTH as i32;
pub const MAX_PROJECTION_STACK_DEPTH: i32 = libdragon_sys::GL_MAX_PROJECTION_STACK_DEPTH as i32;
pub const MAX_TEXTURE_STACK_DEPTH: i32 = libdragon_sys::GL_MAX_TEXTURE_STACK_DEPTH as i32;
pub const TEXTURE_GEN_S: i32 = libdragon_sys::GL_TEXTURE_GEN_S as i32;
pub const TEXTURE_GEN_T: i32 = libdragon_sys::GL_TEXTURE_GEN_T as i32;
pub const TEXTURE_GEN_R: i32 = libdragon_sys::GL_TEXTURE_GEN_R as i32;
pub const TEXTURE_GEN_Q: i32 = libdragon_sys::GL_TEXTURE_GEN_Q as i32;
pub const TEXTURE_GEN_MODE: i32 = libdragon_sys::GL_TEXTURE_GEN_MODE as i32;
pub const OBJECT_PLANE: i32 = libdragon_sys::GL_OBJECT_PLANE as i32;
pub const EYE_PLANE: i32 = libdragon_sys::GL_EYE_PLANE as i32;
pub const EYE_LINEAR: i32 = libdragon_sys::GL_EYE_LINEAR as i32;
pub const OBJECT_LINEAR: i32 = libdragon_sys::GL_OBJECT_LINEAR as i32;
pub const SPHERE_MAP: i32 = libdragon_sys::GL_SPHERE_MAP as i32;
pub const CLIP_PLANE0: i32 = libdragon_sys::GL_CLIP_PLANE0 as i32;
pub const CLIP_PLANE1: i32 = libdragon_sys::GL_CLIP_PLANE1 as i32;
pub const CLIP_PLANE2: i32 = libdragon_sys::GL_CLIP_PLANE2 as i32;
pub const CLIP_PLANE3: i32 = libdragon_sys::GL_CLIP_PLANE3 as i32;
pub const CLIP_PLANE4: i32 = libdragon_sys::GL_CLIP_PLANE4 as i32;
pub const CLIP_PLANE5: i32 = libdragon_sys::GL_CLIP_PLANE5 as i32;
pub const MAX_CLIP_PLANES: i32 = libdragon_sys::GL_MAX_CLIP_PLANES as i32;
pub const LIGHTING: i32 = libdragon_sys::GL_LIGHTING as i32;
pub const LIGHT_MODEL_LOCAL_VIEWER: i32 = libdragon_sys::GL_LIGHT_MODEL_LOCAL_VIEWER as i32;
pub const LIGHT_MODEL_TWO_SIDE: i32 = libdragon_sys::GL_LIGHT_MODEL_TWO_SIDE as i32;
pub const LIGHT_MODEL_AMBIENT: i32 = libdragon_sys::GL_LIGHT_MODEL_AMBIENT as i32;
pub const SHADE_MODEL: i32 = libdragon_sys::GL_SHADE_MODEL as i32;
pub const COLOR_MATERIAL_FACE: i32 = libdragon_sys::GL_COLOR_MATERIAL_FACE as i32;
pub const COLOR_MATERIAL_PARAMETER: i32 = libdragon_sys::GL_COLOR_MATERIAL_PARAMETER as i32;
pub const COLOR_MATERIAL: i32 = libdragon_sys::GL_COLOR_MATERIAL as i32;
pub const LIGHT0: i32 = libdragon_sys::GL_LIGHT0 as i32;
pub const LIGHT1: i32 = libdragon_sys::GL_LIGHT1 as i32;
pub const LIGHT2: i32 = libdragon_sys::GL_LIGHT2 as i32;
pub const LIGHT3: i32 = libdragon_sys::GL_LIGHT3 as i32;
pub const LIGHT4: i32 = libdragon_sys::GL_LIGHT4 as i32;
pub const LIGHT5: i32 = libdragon_sys::GL_LIGHT5 as i32;
pub const LIGHT6: i32 = libdragon_sys::GL_LIGHT6 as i32;
pub const LIGHT7: i32 = libdragon_sys::GL_LIGHT7 as i32;
pub const MAX_LIGHTS: i32 = libdragon_sys::GL_MAX_LIGHTS as i32;
pub const AMBIENT: i32 = libdragon_sys::GL_AMBIENT as i32;
pub const DIFFUSE: i32 = libdragon_sys::GL_DIFFUSE as i32;
pub const SPECULAR: i32 = libdragon_sys::GL_SPECULAR as i32;
pub const POSITION: i32 = libdragon_sys::GL_POSITION as i32;
pub const SPOT_DIRECTION: i32 = libdragon_sys::GL_SPOT_DIRECTION as i32;
pub const SPOT_EXPONENT: i32 = libdragon_sys::GL_SPOT_EXPONENT as i32;
pub const SPOT_CUTOFF: i32 = libdragon_sys::GL_SPOT_CUTOFF as i32;
pub const CONSTANT_ATTENUATION: i32 = libdragon_sys::GL_CONSTANT_ATTENUATION as i32;
pub const LINEAR_ATTENUATION: i32 = libdragon_sys::GL_LINEAR_ATTENUATION as i32;
pub const QUADRATIC_ATTENUATION: i32 = libdragon_sys::GL_QUADRATIC_ATTENUATION as i32;
pub const EMISSION: i32 = libdragon_sys::GL_EMISSION as i32;
pub const SHININESS: i32 = libdragon_sys::GL_SHININESS as i32;
pub const AMBIENT_AND_DIFFUSE: i32 = libdragon_sys::GL_AMBIENT_AND_DIFFUSE as i32;
pub const COLOR_INDEXES: i32 = libdragon_sys::GL_COLOR_INDEXES as i32;
pub const FLAT: i32 = libdragon_sys::GL_FLAT as i32;
pub const SMOOTH: i32 = libdragon_sys::GL_SMOOTH as i32;
pub const POINT_SMOOTH: i32 = libdragon_sys::GL_POINT_SMOOTH as i32;
pub const POINT_SIZE: i32 = libdragon_sys::GL_POINT_SIZE as i32;
pub const POINT_SIZE_GRANULARITY: i32 = libdragon_sys::GL_POINT_SIZE_GRANULARITY as i32;
pub const POINT_SIZE_RANGE: i32 = libdragon_sys::GL_POINT_SIZE_RANGE as i32;
pub const LINE_SMOOTH: i32 = libdragon_sys::GL_LINE_SMOOTH as i32;
pub const LINE_WIDTH: i32 = libdragon_sys::GL_LINE_WIDTH as i32;
pub const LINE_WIDTH_RANGE: i32 = libdragon_sys::GL_LINE_WIDTH_RANGE as i32;
pub const LINE_WIDTH_GRANULARITY: i32 = libdragon_sys::GL_LINE_WIDTH_GRANULARITY as i32;
pub const LINE_STIPPLE: i32 = libdragon_sys::GL_LINE_STIPPLE as i32;
pub const LINE_STIPPLE_PATTERN: i32 = libdragon_sys::GL_LINE_STIPPLE_PATTERN as i32;
pub const LINE_STIPPLE_REPEAT: i32 = libdragon_sys::GL_LINE_STIPPLE_REPEAT as i32;
pub const POLYGON_MODE: i32 = libdragon_sys::GL_POLYGON_MODE as i32;
pub const POLYGON_SMOOTH: i32 = libdragon_sys::GL_POLYGON_SMOOTH as i32;
pub const POLYGON_STIPPLE: i32 = libdragon_sys::GL_POLYGON_STIPPLE as i32;
pub const CULL_FACE: i32 = libdragon_sys::GL_CULL_FACE as i32;
pub const CULL_FACE_MODE: i32 = libdragon_sys::GL_CULL_FACE_MODE as i32;
pub const FRONT_FACE: i32 = libdragon_sys::GL_FRONT_FACE as i32;
pub const CW: i32 = libdragon_sys::GL_CW as i32;
pub const CCW: i32 = libdragon_sys::GL_CCW as i32;
pub const POINT: i32 = libdragon_sys::GL_POINT as i32;
pub const LINE: i32 = libdragon_sys::GL_LINE as i32;
pub const FILL: i32 = libdragon_sys::GL_FILL as i32;
pub const POLYGON_OFFSET_UNITS: i32 = libdragon_sys::GL_POLYGON_OFFSET_UNITS as i32;
pub const POLYGON_OFFSET_POINT: i32 = libdragon_sys::GL_POLYGON_OFFSET_POINT as i32;
pub const POLYGON_OFFSET_LINE: i32 = libdragon_sys::GL_POLYGON_OFFSET_LINE as i32;
pub const POLYGON_OFFSET_FILL: i32 = libdragon_sys::GL_POLYGON_OFFSET_FILL as i32;
pub const POLYGON_OFFSET_FACTOR: i32 = libdragon_sys::GL_POLYGON_OFFSET_FACTOR as i32;
pub const UNPACK_SWAP_BYTES: i32 = libdragon_sys::GL_UNPACK_SWAP_BYTES as i32;
pub const UNPACK_LSB_FIRST: i32 = libdragon_sys::GL_UNPACK_LSB_FIRST as i32;
pub const UNPACK_ROW_LENGTH: i32 = libdragon_sys::GL_UNPACK_ROW_LENGTH as i32;
pub const UNPACK_SKIP_ROWS: i32 = libdragon_sys::GL_UNPACK_SKIP_ROWS as i32;
pub const UNPACK_SKIP_PIXELS: i32 = libdragon_sys::GL_UNPACK_SKIP_PIXELS as i32;
pub const UNPACK_ALIGNMENT: i32 = libdragon_sys::GL_UNPACK_ALIGNMENT as i32;
pub const PACK_SWAP_BYTES: i32 = libdragon_sys::GL_PACK_SWAP_BYTES as i32;
pub const PACK_LSB_FIRST: i32 = libdragon_sys::GL_PACK_LSB_FIRST as i32;
pub const PACK_ROW_LENGTH: i32 = libdragon_sys::GL_PACK_ROW_LENGTH as i32;
pub const PACK_SKIP_ROWS: i32 = libdragon_sys::GL_PACK_SKIP_ROWS as i32;
pub const PACK_SKIP_PIXELS: i32 = libdragon_sys::GL_PACK_SKIP_PIXELS as i32;
pub const PACK_ALIGNMENT: i32 = libdragon_sys::GL_PACK_ALIGNMENT as i32;
pub const MAP_COLOR: i32 = libdragon_sys::GL_MAP_COLOR as i32;
pub const MAP_STENCIL: i32 = libdragon_sys::GL_MAP_STENCIL as i32;
pub const INDEX_SHIFT: i32 = libdragon_sys::GL_INDEX_SHIFT as i32;
pub const INDEX_OFFSET: i32 = libdragon_sys::GL_INDEX_OFFSET as i32;
pub const RED_SCALE: i32 = libdragon_sys::GL_RED_SCALE as i32;
pub const RED_BIAS: i32 = libdragon_sys::GL_RED_BIAS as i32;
pub const ZOOM_X: i32 = libdragon_sys::GL_ZOOM_X as i32;
pub const ZOOM_Y: i32 = libdragon_sys::GL_ZOOM_Y as i32;
pub const GREEN_SCALE: i32 = libdragon_sys::GL_GREEN_SCALE as i32;
pub const GREEN_BIAS: i32 = libdragon_sys::GL_GREEN_BIAS as i32;
pub const BLUE_SCALE: i32 = libdragon_sys::GL_BLUE_SCALE as i32;
pub const BLUE_BIAS: i32 = libdragon_sys::GL_BLUE_BIAS as i32;
pub const ALPHA_SCALE: i32 = libdragon_sys::GL_ALPHA_SCALE as i32;
pub const ALPHA_BIAS: i32 = libdragon_sys::GL_ALPHA_BIAS as i32;
pub const DEPTH_SCALE: i32 = libdragon_sys::GL_DEPTH_SCALE as i32;
pub const DEPTH_BIAS: i32 = libdragon_sys::GL_DEPTH_BIAS as i32;
pub const PIXEL_MAP_I_TO_I: i32 = libdragon_sys::GL_PIXEL_MAP_I_TO_I as i32;
pub const PIXEL_MAP_S_TO_S: i32 = libdragon_sys::GL_PIXEL_MAP_S_TO_S as i32;
pub const PIXEL_MAP_I_TO_R: i32 = libdragon_sys::GL_PIXEL_MAP_I_TO_R as i32;
pub const PIXEL_MAP_I_TO_G: i32 = libdragon_sys::GL_PIXEL_MAP_I_TO_G as i32;
pub const PIXEL_MAP_I_TO_B: i32 = libdragon_sys::GL_PIXEL_MAP_I_TO_B as i32;
pub const PIXEL_MAP_I_TO_A: i32 = libdragon_sys::GL_PIXEL_MAP_I_TO_A as i32;
pub const PIXEL_MAP_R_TO_R: i32 = libdragon_sys::GL_PIXEL_MAP_R_TO_R as i32;
pub const PIXEL_MAP_G_TO_G: i32 = libdragon_sys::GL_PIXEL_MAP_G_TO_G as i32;
pub const PIXEL_MAP_B_TO_B: i32 = libdragon_sys::GL_PIXEL_MAP_B_TO_B as i32;
pub const PIXEL_MAP_A_TO_A: i32 = libdragon_sys::GL_PIXEL_MAP_A_TO_A as i32;
pub const COLOR: i32 = libdragon_sys::GL_COLOR as i32;
pub const DEPTH: i32 = libdragon_sys::GL_DEPTH as i32;
pub const STENCIL: i32 = libdragon_sys::GL_STENCIL as i32;
pub const READ_BUFFER: i32 = libdragon_sys::GL_READ_BUFFER as i32;
pub const PIXEL_MAP_I_TO_I_SIZE: i32 = libdragon_sys::GL_PIXEL_MAP_I_TO_I_SIZE as i32;
pub const PIXEL_MAP_S_TO_S_SIZE: i32 = libdragon_sys::GL_PIXEL_MAP_S_TO_S_SIZE as i32;
pub const PIXEL_MAP_I_TO_R_SIZE: i32 = libdragon_sys::GL_PIXEL_MAP_I_TO_R_SIZE as i32;
pub const PIXEL_MAP_I_TO_G_SIZE: i32 = libdragon_sys::GL_PIXEL_MAP_I_TO_G_SIZE as i32;
pub const PIXEL_MAP_I_TO_B_SIZE: i32 = libdragon_sys::GL_PIXEL_MAP_I_TO_B_SIZE as i32;
pub const PIXEL_MAP_I_TO_A_SIZE: i32 = libdragon_sys::GL_PIXEL_MAP_I_TO_A_SIZE as i32;
pub const PIXEL_MAP_R_TO_R_SIZE: i32 = libdragon_sys::GL_PIXEL_MAP_R_TO_R_SIZE as i32;
pub const PIXEL_MAP_G_TO_G_SIZE: i32 = libdragon_sys::GL_PIXEL_MAP_G_TO_G_SIZE as i32;
pub const PIXEL_MAP_B_TO_B_SIZE: i32 = libdragon_sys::GL_PIXEL_MAP_B_TO_B_SIZE as i32;
pub const PIXEL_MAP_A_TO_A_SIZE: i32 = libdragon_sys::GL_PIXEL_MAP_A_TO_A_SIZE as i32;
pub const MAX_PIXEL_MAP_TABLE: i32 = libdragon_sys::GL_MAX_PIXEL_MAP_TABLE as i32;
pub const BITMAP: i32 = libdragon_sys::GL_BITMAP as i32;
pub const COLOR_INDEX: i32 = libdragon_sys::GL_COLOR_INDEX as i32;
pub const STENCIL_INDEX: i32 = libdragon_sys::GL_STENCIL_INDEX as i32;
pub const DEPTH_COMPONENT: i32 = libdragon_sys::GL_DEPTH_COMPONENT as i32;
pub const RED: i32 = libdragon_sys::GL_RED as i32;
pub const GREEN: i32 = libdragon_sys::GL_GREEN as i32;
pub const BLUE: i32 = libdragon_sys::GL_BLUE as i32;
pub const ALPHA: i32 = libdragon_sys::GL_ALPHA as i32;
pub const RGB: i32 = libdragon_sys::GL_RGB as i32;
pub const RGBA: i32 = libdragon_sys::GL_RGBA as i32;
pub const LUMINANCE: i32 = libdragon_sys::GL_LUMINANCE as i32;
pub const LUMINANCE_ALPHA: i32 = libdragon_sys::GL_LUMINANCE_ALPHA as i32;
pub const R3_G3_B2: i32 = libdragon_sys::GL_R3_G3_B2 as i32;
pub const ALPHA4: i32 = libdragon_sys::GL_ALPHA4 as i32;
pub const ALPHA8: i32 = libdragon_sys::GL_ALPHA8 as i32;
pub const ALPHA12: i32 = libdragon_sys::GL_ALPHA12 as i32;
pub const ALPHA16: i32 = libdragon_sys::GL_ALPHA16 as i32;
pub const LUMINANCE4: i32 = libdragon_sys::GL_LUMINANCE4 as i32;
pub const LUMINANCE8: i32 = libdragon_sys::GL_LUMINANCE8 as i32;
pub const LUMINANCE12: i32 = libdragon_sys::GL_LUMINANCE12 as i32;
pub const LUMINANCE16: i32 = libdragon_sys::GL_LUMINANCE16 as i32;
pub const LUMINANCE4_ALPHA4: i32 = libdragon_sys::GL_LUMINANCE4_ALPHA4 as i32;
pub const LUMINANCE6_ALPHA2: i32 = libdragon_sys::GL_LUMINANCE6_ALPHA2 as i32;
pub const LUMINANCE8_ALPHA8: i32 = libdragon_sys::GL_LUMINANCE8_ALPHA8 as i32;
pub const LUMINANCE12_ALPHA4: i32 = libdragon_sys::GL_LUMINANCE12_ALPHA4 as i32;
pub const LUMINANCE12_ALPHA12: i32 = libdragon_sys::GL_LUMINANCE12_ALPHA12 as i32;
pub const LUMINANCE16_ALPHA16: i32 = libdragon_sys::GL_LUMINANCE16_ALPHA16 as i32;
pub const INTENSITY: i32 = libdragon_sys::GL_INTENSITY as i32;
pub const INTENSITY4: i32 = libdragon_sys::GL_INTENSITY4 as i32;
pub const INTENSITY8: i32 = libdragon_sys::GL_INTENSITY8 as i32;
pub const INTENSITY12: i32 = libdragon_sys::GL_INTENSITY12 as i32;
pub const INTENSITY16: i32 = libdragon_sys::GL_INTENSITY16 as i32;
pub const RGB4: i32 = libdragon_sys::GL_RGB4 as i32;
pub const RGB5: i32 = libdragon_sys::GL_RGB5 as i32;
pub const RGB8: i32 = libdragon_sys::GL_RGB8 as i32;
pub const RGB10: i32 = libdragon_sys::GL_RGB10 as i32;
pub const RGB12: i32 = libdragon_sys::GL_RGB12 as i32;
pub const RGB16: i32 = libdragon_sys::GL_RGB16 as i32;
pub const RGBA2: i32 = libdragon_sys::GL_RGBA2 as i32;
pub const RGBA4: i32 = libdragon_sys::GL_RGBA4 as i32;
pub const RGB5_A1: i32 = libdragon_sys::GL_RGB5_A1 as i32;
pub const RGBA8: i32 = libdragon_sys::GL_RGBA8 as i32;
pub const RGB10_A2: i32 = libdragon_sys::GL_RGB10_A2 as i32;
pub const RGBA12: i32 = libdragon_sys::GL_RGBA12 as i32;
pub const RGBA16: i32 = libdragon_sys::GL_RGBA16 as i32;
pub const UNSIGNED_BYTE_3_3_2_EXT: i32 = libdragon_sys::GL_UNSIGNED_BYTE_3_3_2_EXT as i32;
pub const UNSIGNED_SHORT_4_4_4_4_EXT: i32 = libdragon_sys::GL_UNSIGNED_SHORT_4_4_4_4_EXT as i32;
pub const UNSIGNED_SHORT_5_5_5_1_EXT: i32 = libdragon_sys::GL_UNSIGNED_SHORT_5_5_5_1_EXT as i32;
pub const UNSIGNED_INT_8_8_8_8_EXT: i32 = libdragon_sys::GL_UNSIGNED_INT_8_8_8_8_EXT as i32;
pub const UNSIGNED_INT_10_10_10_2_EXT: i32 = libdragon_sys::GL_UNSIGNED_INT_10_10_10_2_EXT as i32;
pub const TEXTURE_1D: i32 = libdragon_sys::GL_TEXTURE_1D as i32;
pub const TEXTURE_2D: i32 = libdragon_sys::GL_TEXTURE_2D as i32;
pub const PROXY_TEXTURE_1D: i32 = libdragon_sys::GL_PROXY_TEXTURE_1D as i32;
pub const PROXY_TEXTURE_2D: i32 = libdragon_sys::GL_PROXY_TEXTURE_2D as i32;
pub const TEXTURE_MAG_FILTER: i32 = libdragon_sys::GL_TEXTURE_MAG_FILTER as i32;
pub const TEXTURE_MIN_FILTER: i32 = libdragon_sys::GL_TEXTURE_MIN_FILTER as i32;
pub const TEXTURE_WRAP_S: i32 = libdragon_sys::GL_TEXTURE_WRAP_S as i32;
pub const TEXTURE_WRAP_T: i32 = libdragon_sys::GL_TEXTURE_WRAP_T as i32;
pub const TEXTURE_WIDTH: i32 = libdragon_sys::GL_TEXTURE_WIDTH as i32;
pub const TEXTURE_HEIGHT: i32 = libdragon_sys::GL_TEXTURE_HEIGHT as i32;
pub const TEXTURE_INTERNAL_FORMAT: i32 = libdragon_sys::GL_TEXTURE_INTERNAL_FORMAT as i32;
pub const TEXTURE_BORDER_COLOR: i32 = libdragon_sys::GL_TEXTURE_BORDER_COLOR as i32;
pub const TEXTURE_BORDER: i32 = libdragon_sys::GL_TEXTURE_BORDER as i32;
pub const TEXTURE_RED_SIZE: i32 = libdragon_sys::GL_TEXTURE_RED_SIZE as i32;
pub const TEXTURE_GREEN_SIZE: i32 = libdragon_sys::GL_TEXTURE_GREEN_SIZE as i32;
pub const TEXTURE_BLUE_SIZE: i32 = libdragon_sys::GL_TEXTURE_BLUE_SIZE as i32;
pub const TEXTURE_ALPHA_SIZE: i32 = libdragon_sys::GL_TEXTURE_ALPHA_SIZE as i32;
pub const TEXTURE_LUMINANCE_SIZE: i32 = libdragon_sys::GL_TEXTURE_LUMINANCE_SIZE as i32;
pub const TEXTURE_INTENSITY_SIZE: i32 = libdragon_sys::GL_TEXTURE_INTENSITY_SIZE as i32;
pub const TEXTURE_PRIORITY: i32 = libdragon_sys::GL_TEXTURE_PRIORITY as i32;
pub const TEXTURE_RESIDENT: i32 = libdragon_sys::GL_TEXTURE_RESIDENT as i32;
pub const NEAREST: i32 = libdragon_sys::GL_NEAREST as i32;
pub const LINEAR: i32 = libdragon_sys::GL_LINEAR as i32;
pub const NEAREST_MIPMAP_NEAREST: i32 = libdragon_sys::GL_NEAREST_MIPMAP_NEAREST as i32;
pub const LINEAR_MIPMAP_NEAREST: i32 = libdragon_sys::GL_LINEAR_MIPMAP_NEAREST as i32;
pub const NEAREST_MIPMAP_LINEAR: i32 = libdragon_sys::GL_NEAREST_MIPMAP_LINEAR as i32;
pub const LINEAR_MIPMAP_LINEAR: i32 = libdragon_sys::GL_LINEAR_MIPMAP_LINEAR as i32;
pub const CLAMP: i32 = libdragon_sys::GL_CLAMP as i32;
pub const REPEAT: i32 = libdragon_sys::GL_REPEAT as i32;
pub const MIRRORED_REPEAT_ARB: i32 = libdragon_sys::GL_MIRRORED_REPEAT_ARB as i32;
pub const TEXTURE_ENV: i32 = libdragon_sys::GL_TEXTURE_ENV as i32;
pub const TEXTURE_ENV_MODE: i32 = libdragon_sys::GL_TEXTURE_ENV_MODE as i32;
pub const TEXTURE_ENV_COLOR: i32 = libdragon_sys::GL_TEXTURE_ENV_COLOR as i32;
pub const MODULATE: i32 = libdragon_sys::GL_MODULATE as i32;
pub const DECAL: i32 = libdragon_sys::GL_DECAL as i32;
pub const BLEND: i32 = libdragon_sys::GL_BLEND as i32;
pub const REPLACE: i32 = libdragon_sys::GL_REPLACE as i32;
pub const S: i32 = libdragon_sys::GL_S as i32;
pub const T: i32 = libdragon_sys::GL_T as i32;
pub const R: i32 = libdragon_sys::GL_R as i32;
pub const Q: i32 = libdragon_sys::GL_Q as i32;
pub const MAX_TEXTURE_SIZE: i32 = libdragon_sys::GL_MAX_TEXTURE_SIZE as i32;
pub const TEXTURE_FLIP_T_N64: i32 = libdragon_sys::GL_TEXTURE_FLIP_T_N64 as i32;
pub const FOG: i32 = libdragon_sys::GL_FOG as i32;
pub const FOG_INDEX: i32 = libdragon_sys::GL_FOG_INDEX as i32;
pub const FOG_DENSITY: i32 = libdragon_sys::GL_FOG_DENSITY as i32;
pub const FOG_START: i32 = libdragon_sys::GL_FOG_START as i32;
pub const FOG_END: i32 = libdragon_sys::GL_FOG_END as i32;
pub const FOG_MODE: i32 = libdragon_sys::GL_FOG_MODE as i32;
pub const FOG_COLOR: i32 = libdragon_sys::GL_FOG_COLOR as i32;
pub const EXP: i32 = libdragon_sys::GL_EXP as i32;
pub const EXP2: i32 = libdragon_sys::GL_EXP2 as i32;
pub const SCISSOR_BOX: i32 = libdragon_sys::GL_SCISSOR_BOX as i32;
pub const SCISSOR_TEST: i32 = libdragon_sys::GL_SCISSOR_TEST as i32;
pub const ALPHA_TEST: i32 = libdragon_sys::GL_ALPHA_TEST as i32;
pub const ALPHA_TEST_FUNC: i32 = libdragon_sys::GL_ALPHA_TEST_FUNC as i32;
pub const ALPHA_TEST_REF: i32 = libdragon_sys::GL_ALPHA_TEST_REF as i32;
pub const NEVER: i32 = libdragon_sys::GL_NEVER as i32;
pub const LESS: i32 = libdragon_sys::GL_LESS as i32;
pub const EQUAL: i32 = libdragon_sys::GL_EQUAL as i32;
pub const LEQUAL: i32 = libdragon_sys::GL_LEQUAL as i32;
pub const GREATER: i32 = libdragon_sys::GL_GREATER as i32;
pub const NOTEQUAL: i32 = libdragon_sys::GL_NOTEQUAL as i32;
pub const GEQUAL: i32 = libdragon_sys::GL_GEQUAL as i32;
pub const ALWAYS: i32 = libdragon_sys::GL_ALWAYS as i32;
pub const LESS_INTERPENETRATING_N64: i32 = libdragon_sys::GL_LESS_INTERPENETRATING_N64 as i32;
pub const STENCIL_TEST: i32 = libdragon_sys::GL_STENCIL_TEST as i32;
pub const STENCIL_FUNC: i32 = libdragon_sys::GL_STENCIL_FUNC as i32;
pub const STENCIL_VALUE_MASK: i32 = libdragon_sys::GL_STENCIL_VALUE_MASK as i32;
pub const STENCIL_FAIL: i32 = libdragon_sys::GL_STENCIL_FAIL as i32;
pub const STENCIL_PASS_DEPTH_FAIL: i32 = libdragon_sys::GL_STENCIL_PASS_DEPTH_FAIL as i32;
pub const STENCIL_PASS_DEPTH_PASS: i32 = libdragon_sys::GL_STENCIL_PASS_DEPTH_PASS as i32;
pub const STENCIL_REF: i32 = libdragon_sys::GL_STENCIL_REF as i32;
pub const KEEP: i32 = libdragon_sys::GL_KEEP as i32;
pub const INCR: i32 = libdragon_sys::GL_INCR as i32;
pub const DECR: i32 = libdragon_sys::GL_DECR as i32;
pub const DEPTH_TEST: i32 = libdragon_sys::GL_DEPTH_TEST as i32;
pub const DEPTH_FUNC: i32 = libdragon_sys::GL_DEPTH_FUNC as i32;
pub const BLEND_DST: i32 = libdragon_sys::GL_BLEND_DST as i32;
pub const BLEND_SRC: i32 = libdragon_sys::GL_BLEND_SRC as i32;
pub const ZERO: i32 = libdragon_sys::GL_ZERO as i32;
pub const ONE: i32 = libdragon_sys::GL_ONE as i32;
pub const SRC_COLOR: i32 = libdragon_sys::GL_SRC_COLOR as i32;
pub const ONE_MINUS_SRC_COLOR: i32 = libdragon_sys::GL_ONE_MINUS_SRC_COLOR as i32;
pub const SRC_ALPHA: i32 = libdragon_sys::GL_SRC_ALPHA as i32;
pub const ONE_MINUS_SRC_ALPHA: i32 = libdragon_sys::GL_ONE_MINUS_SRC_ALPHA as i32;
pub const DST_COLOR: i32 = libdragon_sys::GL_DST_COLOR as i32;
pub const ONE_MINUS_DST_COLOR: i32 = libdragon_sys::GL_ONE_MINUS_DST_COLOR as i32;
pub const DST_ALPHA: i32 = libdragon_sys::GL_DST_ALPHA as i32;
pub const ONE_MINUS_DST_ALPHA: i32 = libdragon_sys::GL_ONE_MINUS_DST_ALPHA as i32;
pub const SRC_ALPHA_SATURATE: i32 = libdragon_sys::GL_SRC_ALPHA_SATURATE as i32;
pub const CLEAR: i32 = libdragon_sys::GL_CLEAR as i32;
pub const AND: i32 = libdragon_sys::GL_AND as i32;
pub const AND_REVERSE: i32 = libdragon_sys::GL_AND_REVERSE as i32;
pub const COPY: i32 = libdragon_sys::GL_COPY as i32;
pub const AND_INVERTED: i32 = libdragon_sys::GL_AND_INVERTED as i32;
pub const NOOP: i32 = libdragon_sys::GL_NOOP as i32;
pub const XOR: i32 = libdragon_sys::GL_XOR as i32;
pub const OR: i32 = libdragon_sys::GL_OR as i32;
pub const NOR: i32 = libdragon_sys::GL_NOR as i32;
pub const EQUIV: i32 = libdragon_sys::GL_EQUIV as i32;
pub const INVERT: i32 = libdragon_sys::GL_INVERT as i32;
pub const OR_REVERSE: i32 = libdragon_sys::GL_OR_REVERSE as i32;
pub const COPY_INVERTED: i32 = libdragon_sys::GL_COPY_INVERTED as i32;
pub const OR_INVERTED: i32 = libdragon_sys::GL_OR_INVERTED as i32;
pub const NAND: i32 = libdragon_sys::GL_NAND as i32;
pub const SET: i32 = libdragon_sys::GL_SET as i32;
pub const LOGIC_OP_MODE: i32 = libdragon_sys::GL_LOGIC_OP_MODE as i32;
pub const INDEX_LOGIC_OP: i32 = libdragon_sys::GL_INDEX_LOGIC_OP as i32;
pub const LOGIC_OP: i32 = libdragon_sys::GL_LOGIC_OP as i32;
pub const COLOR_LOGIC_OP: i32 = libdragon_sys::GL_COLOR_LOGIC_OP as i32;
pub const NONE: i32 = libdragon_sys::GL_NONE as i32;
pub const FRONT_LEFT: i32 = libdragon_sys::GL_FRONT_LEFT as i32;
pub const FRONT_RIGHT: i32 = libdragon_sys::GL_FRONT_RIGHT as i32;
pub const BACK_LEFT: i32 = libdragon_sys::GL_BACK_LEFT as i32;
pub const BACK_RIGHT: i32 = libdragon_sys::GL_BACK_RIGHT as i32;
pub const FRONT: i32 = libdragon_sys::GL_FRONT as i32;
pub const BACK: i32 = libdragon_sys::GL_BACK as i32;
pub const LEFT: i32 = libdragon_sys::GL_LEFT as i32;
pub const RIGHT: i32 = libdragon_sys::GL_RIGHT as i32;
pub const FRONT_AND_BACK: i32 = libdragon_sys::GL_FRONT_AND_BACK as i32;
pub const AUX0: i32 = libdragon_sys::GL_AUX0 as i32;
pub const AUX1: i32 = libdragon_sys::GL_AUX1 as i32;
pub const AUX2: i32 = libdragon_sys::GL_AUX2 as i32;
pub const AUX3: i32 = libdragon_sys::GL_AUX3 as i32;
pub const AUX_BUFFERS: i32 = libdragon_sys::GL_AUX_BUFFERS as i32;
pub const DRAW_BUFFER: i32 = libdragon_sys::GL_DRAW_BUFFER as i32;
pub const INDEX_WRITEMASK: i32 = libdragon_sys::GL_INDEX_WRITEMASK as i32;
pub const COLOR_WRITEMASK: i32 = libdragon_sys::GL_COLOR_WRITEMASK as i32;
pub const DEPTH_WRITEMASK: i32 = libdragon_sys::GL_DEPTH_WRITEMASK as i32;
pub const STENCIL_WRITEMASK: i32 = libdragon_sys::GL_STENCIL_WRITEMASK as i32;
pub const DEPTH_BUFFER_BIT: i32 = libdragon_sys::GL_DEPTH_BUFFER_BIT as i32;
pub const ACCUM_BUFFER_BIT: i32 = libdragon_sys::GL_ACCUM_BUFFER_BIT as i32;
pub const STENCIL_BUFFER_BIT: i32 = libdragon_sys::GL_STENCIL_BUFFER_BIT as i32;
pub const COLOR_BUFFER_BIT: i32 = libdragon_sys::GL_COLOR_BUFFER_BIT as i32;
pub const COLOR_CLEAR_VALUE: i32 = libdragon_sys::GL_COLOR_CLEAR_VALUE as i32;
pub const DEPTH_CLEAR_VALUE: i32 = libdragon_sys::GL_DEPTH_CLEAR_VALUE as i32;
pub const INDEX_CLEAR_VALUE: i32 = libdragon_sys::GL_INDEX_CLEAR_VALUE as i32;
pub const STENCIL_CLEAR_VALUE: i32 = libdragon_sys::GL_STENCIL_CLEAR_VALUE as i32;
pub const ACCUM_CLEAR_VALUE: i32 = libdragon_sys::GL_ACCUM_CLEAR_VALUE as i32;
pub const ACCUM: i32 = libdragon_sys::GL_ACCUM as i32;
pub const LOAD: i32 = libdragon_sys::GL_LOAD as i32;
pub const RETURN: i32 = libdragon_sys::GL_RETURN as i32;
pub const MULT: i32 = libdragon_sys::GL_MULT as i32;
pub const ADD: i32 = libdragon_sys::GL_ADD as i32;
pub const ACCUM_RED_BITS: i32 = libdragon_sys::GL_ACCUM_RED_BITS as i32;
pub const ACCUM_GREEN_BITS: i32 = libdragon_sys::GL_ACCUM_GREEN_BITS as i32;
pub const ACCUM_BLUE_BITS: i32 = libdragon_sys::GL_ACCUM_BLUE_BITS as i32;
pub const ACCUM_ALPHA_BITS: i32 = libdragon_sys::GL_ACCUM_ALPHA_BITS as i32;
pub const AUTO_NORMAL: i32 = libdragon_sys::GL_AUTO_NORMAL as i32;
pub const MAP1_COLOR_4: i32 = libdragon_sys::GL_MAP1_COLOR_4 as i32;
pub const MAP1_INDEX: i32 = libdragon_sys::GL_MAP1_INDEX as i32;
pub const MAP1_NORMAL: i32 = libdragon_sys::GL_MAP1_NORMAL as i32;
pub const MAP1_TEXTURE_COORD_1: i32 = libdragon_sys::GL_MAP1_TEXTURE_COORD_1 as i32;
pub const MAP1_TEXTURE_COORD_2: i32 = libdragon_sys::GL_MAP1_TEXTURE_COORD_2 as i32;
pub const MAP1_TEXTURE_COORD_3: i32 = libdragon_sys::GL_MAP1_TEXTURE_COORD_3 as i32;
pub const MAP1_TEXTURE_COORD_4: i32 = libdragon_sys::GL_MAP1_TEXTURE_COORD_4 as i32;
pub const MAP1_VERTEX_3: i32 = libdragon_sys::GL_MAP1_VERTEX_3 as i32;
pub const MAP1_VERTEX_4: i32 = libdragon_sys::GL_MAP1_VERTEX_4 as i32;
pub const MAP2_COLOR_4: i32 = libdragon_sys::GL_MAP2_COLOR_4 as i32;
pub const MAP2_INDEX: i32 = libdragon_sys::GL_MAP2_INDEX as i32;
pub const MAP2_NORMAL: i32 = libdragon_sys::GL_MAP2_NORMAL as i32;
pub const MAP2_TEXTURE_COORD_1: i32 = libdragon_sys::GL_MAP2_TEXTURE_COORD_1 as i32;
pub const MAP2_TEXTURE_COORD_2: i32 = libdragon_sys::GL_MAP2_TEXTURE_COORD_2 as i32;
pub const MAP2_TEXTURE_COORD_3: i32 = libdragon_sys::GL_MAP2_TEXTURE_COORD_3 as i32;
pub const MAP2_TEXTURE_COORD_4: i32 = libdragon_sys::GL_MAP2_TEXTURE_COORD_4 as i32;
pub const MAP2_VERTEX_3: i32 = libdragon_sys::GL_MAP2_VERTEX_3 as i32;
pub const MAP2_VERTEX_4: i32 = libdragon_sys::GL_MAP2_VERTEX_4 as i32;
pub const MAP1_GRID_DOMAIN: i32 = libdragon_sys::GL_MAP1_GRID_DOMAIN as i32;
pub const MAP1_GRID_SEGMENTS: i32 = libdragon_sys::GL_MAP1_GRID_SEGMENTS as i32;
pub const MAP2_GRID_DOMAIN: i32 = libdragon_sys::GL_MAP2_GRID_DOMAIN as i32;
pub const MAP2_GRID_SEGMENTS: i32 = libdragon_sys::GL_MAP2_GRID_SEGMENTS as i32;
pub const MAX_EVAL_ORDER: i32 = libdragon_sys::GL_MAX_EVAL_ORDER as i32;
pub const RENDER: i32 = libdragon_sys::GL_RENDER as i32;
pub const FEEDBACK: i32 = libdragon_sys::GL_FEEDBACK as i32;
pub const SELECT: i32 = libdragon_sys::GL_SELECT as i32;
pub const RENDER_MODE: i32 = libdragon_sys::GL_RENDER_MODE as i32;
pub const SELECTION_BUFFER_POINTER: i32 = libdragon_sys::GL_SELECTION_BUFFER_POINTER as i32;
pub const NAME_STACK_DEPTH: i32 = libdragon_sys::GL_NAME_STACK_DEPTH as i32;
pub const MAX_NAME_STACK_DEPTH: i32 = libdragon_sys::GL_MAX_NAME_STACK_DEPTH as i32;
pub const _2D: i32 = libdragon_sys::GL_2D as i32;
pub const _3D: i32 = libdragon_sys::GL_3D as i32;
pub const _3D_COLOR: i32 = libdragon_sys::GL_3D_COLOR as i32;
pub const _3D_COLOR_TEXTURE: i32 = libdragon_sys::GL_3D_COLOR_TEXTURE as i32;
pub const _4D_COLOR_TEXTURE: i32 = libdragon_sys::GL_4D_COLOR_TEXTURE as i32;
pub const PASS_THROUGH_TOKEN: i32 = libdragon_sys::GL_PASS_THROUGH_TOKEN as i32;
pub const POINT_TOKEN: i32 = libdragon_sys::GL_POINT_TOKEN as i32;
pub const LINE_TOKEN: i32 = libdragon_sys::GL_LINE_TOKEN as i32;
pub const POLYGON_TOKEN: i32 = libdragon_sys::GL_POLYGON_TOKEN as i32;
pub const BITMAP_TOKEN: i32 = libdragon_sys::GL_BITMAP_TOKEN as i32;
pub const DRAW_PIXEL_TOKEN: i32 = libdragon_sys::GL_DRAW_PIXEL_TOKEN as i32;
pub const COPY_PIXEL_TOKEN: i32 = libdragon_sys::GL_COPY_PIXEL_TOKEN as i32;
pub const LINE_RESET_TOKEN: i32 = libdragon_sys::GL_LINE_RESET_TOKEN as i32;
pub const FEEDBACK_BUFFER_POINTER: i32 = libdragon_sys::GL_FEEDBACK_BUFFER_POINTER as i32;
pub const COMPILE: i32 = libdragon_sys::GL_COMPILE as i32;
pub const COMPILE_AND_EXECUTE: i32 = libdragon_sys::GL_COMPILE_AND_EXECUTE as i32;
pub const _2_BYTES: i32 = libdragon_sys::GL_2_BYTES as i32;
pub const _3_BYTES: i32 = libdragon_sys::GL_3_BYTES as i32;
pub const _4_BYTES: i32 = libdragon_sys::GL_4_BYTES as i32;
pub const LIST_MODE: i32 = libdragon_sys::GL_LIST_MODE as i32;
pub const MAX_LIST_NESTING: i32 = libdragon_sys::GL_MAX_LIST_NESTING as i32;
pub const LIST_BASE: i32 = libdragon_sys::GL_LIST_BASE as i32;
pub const LIST_INDEX: i32 = libdragon_sys::GL_LIST_INDEX as i32;
pub const PERSPECTIVE_CORRECTION_HINT: i32 = libdragon_sys::GL_PERSPECTIVE_CORRECTION_HINT as i32;
pub const POINT_SMOOTH_HINT: i32 = libdragon_sys::GL_POINT_SMOOTH_HINT as i32;
pub const LINE_SMOOTH_HINT: i32 = libdragon_sys::GL_LINE_SMOOTH_HINT as i32;
pub const POLYGON_SMOOTH_HINT: i32 = libdragon_sys::GL_POLYGON_SMOOTH_HINT as i32;
pub const FOG_HINT: i32 = libdragon_sys::GL_FOG_HINT as i32;
pub const MULTISAMPLE_HINT_N64: i32 = libdragon_sys::GL_MULTISAMPLE_HINT_N64 as i32;
pub const DONT_CARE: i32 = libdragon_sys::GL_DONT_CARE as i32;
pub const FASTEST: i32 = libdragon_sys::GL_FASTEST as i32;
pub const NICEST: i32 = libdragon_sys::GL_NICEST as i32;
pub const MULTISAMPLE_ARB: i32 = libdragon_sys::GL_MULTISAMPLE_ARB as i32;
pub const SAMPLE_ALPHA_TO_COVERAGE_ARB: i32 = libdragon_sys::GL_SAMPLE_ALPHA_TO_COVERAGE_ARB as i32;
pub const SAMPLE_ALPHA_TO_ONE_ARB: i32 = libdragon_sys::GL_SAMPLE_ALPHA_TO_ONE_ARB as i32;
pub const SAMPLE_COVERAGE_ARB: i32 = libdragon_sys::GL_SAMPLE_COVERAGE_ARB as i32;
pub const SAMPLE_BUFFERS_ARB: i32 = libdragon_sys::GL_SAMPLE_BUFFERS_ARB as i32;
pub const SAMPLES_ARB: i32 = libdragon_sys::GL_SAMPLES_ARB as i32;
pub const SAMPLE_COVERAGE_VALUE_ARB: i32 = libdragon_sys::GL_SAMPLE_COVERAGE_VALUE_ARB as i32;
pub const SAMPLE_COVERAGE_INVERT_ARB: i32 = libdragon_sys::GL_SAMPLE_COVERAGE_INVERT_ARB as i32;
pub const SUBPIXEL_BITS: i32 = libdragon_sys::GL_SUBPIXEL_BITS as i32;
pub const INDEX_BITS: i32 = libdragon_sys::GL_INDEX_BITS as i32;
pub const RED_BITS: i32 = libdragon_sys::GL_RED_BITS as i32;
pub const GREEN_BITS: i32 = libdragon_sys::GL_GREEN_BITS as i32;
pub const BLUE_BITS: i32 = libdragon_sys::GL_BLUE_BITS as i32;
pub const ALPHA_BITS: i32 = libdragon_sys::GL_ALPHA_BITS as i32;
pub const DEPTH_BITS: i32 = libdragon_sys::GL_DEPTH_BITS as i32;
pub const STENCIL_BITS: i32 = libdragon_sys::GL_STENCIL_BITS as i32;
pub const COEFF: i32 = libdragon_sys::GL_COEFF as i32;
pub const ORDER: i32 = libdragon_sys::GL_ORDER as i32;
pub const DOMAIN: i32 = libdragon_sys::GL_DOMAIN as i32;
pub const INDEX_MODE: i32 = libdragon_sys::GL_INDEX_MODE as i32;
pub const RGBA_MODE: i32 = libdragon_sys::GL_RGBA_MODE as i32;
pub const DOUBLEBUFFER: i32 = libdragon_sys::GL_DOUBLEBUFFER as i32;
pub const STEREO: i32 = libdragon_sys::GL_STEREO as i32;
pub const VENDOR: i32 = libdragon_sys::GL_VENDOR as i32;
pub const RENDERER: i32 = libdragon_sys::GL_RENDERER as i32;
pub const VERSION: i32 = libdragon_sys::GL_VERSION as i32;
pub const EXTENSIONS: i32 = libdragon_sys::GL_EXTENSIONS as i32;
pub const RDPQ_MATERIAL_N64: i32 = libdragon_sys::GL_RDPQ_MATERIAL_N64 as i32;
pub const RDPQ_TEXTURING_N64: i32 = libdragon_sys::GL_RDPQ_TEXTURING_N64 as i32;
pub const CURRENT_BIT: i32 = libdragon_sys::GL_CURRENT_BIT as i32;
pub const POINT_BIT: i32 = libdragon_sys::GL_POINT_BIT as i32;
pub const LINE_BIT: i32 = libdragon_sys::GL_LINE_BIT as i32;
pub const POLYGON_BIT: i32 = libdragon_sys::GL_POLYGON_BIT as i32;
pub const POLYGON_STIPPLE_BIT: i32 = libdragon_sys::GL_POLYGON_STIPPLE_BIT as i32;
pub const PIXEL_MODE_BIT: i32 = libdragon_sys::GL_PIXEL_MODE_BIT as i32;
pub const LIGHTING_BIT: i32 = libdragon_sys::GL_LIGHTING_BIT as i32;
pub const FOG_BIT: i32 = libdragon_sys::GL_FOG_BIT as i32;
pub const VIEWPORT_BIT: i32 = libdragon_sys::GL_VIEWPORT_BIT as i32;
pub const TRANSFORM_BIT: i32 = libdragon_sys::GL_TRANSFORM_BIT as i32;
pub const ENABLE_BIT: i32 = libdragon_sys::GL_ENABLE_BIT as i32;
pub const HINT_BIT: i32 = libdragon_sys::GL_HINT_BIT as i32;
pub const EVAL_BIT: i32 = libdragon_sys::GL_EVAL_BIT as i32;
pub const LIST_BIT: i32 = libdragon_sys::GL_LIST_BIT as i32;
pub const TEXTURE_BIT: i32 = libdragon_sys::GL_TEXTURE_BIT as i32;
pub const SCISSOR_BIT: i32 = libdragon_sys::GL_SCISSOR_BIT as i32;
pub const MULTISAMPLE_BIT_ARB: i32 = libdragon_sys::GL_MULTISAMPLE_BIT_ARB as i32;
pub const ALL_ATTRIB_BITS: i32 = libdragon_sys::GL_ALL_ATTRIB_BITS as i32;
pub const CLIENT_PIXEL_STORE_BIT: i32 = libdragon_sys::GL_CLIENT_PIXEL_STORE_BIT as i32;
pub const CLIENT_VERTEX_ARRAY_BIT: i32 = libdragon_sys::GL_CLIENT_VERTEX_ARRAY_BIT as i32;
pub const CLIENT_ALL_ATTRIB_BITS: i32 = libdragon_sys::GL_CLIENT_ALL_ATTRIB_BITS as i32;
pub const ATTRIB_STACK_DEPTH: i32 = libdragon_sys::GL_ATTRIB_STACK_DEPTH as i32;
pub const CLIENT_ATTRIB_STACK_DEPTH: i32 = libdragon_sys::GL_CLIENT_ATTRIB_STACK_DEPTH as i32;
pub const MAX_ATTRIB_STACK_DEPTH: i32 = libdragon_sys::GL_MAX_ATTRIB_STACK_DEPTH as i32;
pub const MAX_CLIENT_ATTRIB_STACK_DEPTH: i32 = libdragon_sys::GL_MAX_CLIENT_ATTRIB_STACK_DEPTH as i32;
pub const VERSION_1_1: i32 = libdragon_sys::GL_VERSION_1_1 as i32;
pub const ARB_multisample: i32 = libdragon_sys::GL_ARB_multisample as i32;
pub const EXT_packed_pixels: i32 = libdragon_sys::GL_EXT_packed_pixels as i32;
pub const ARB_vertex_buffer_object: i32 = libdragon_sys::GL_ARB_vertex_buffer_object as i32;
pub const ARB_texture_mirrored_repeat: i32 = libdragon_sys::GL_ARB_texture_mirrored_repeat as i32;
pub const ARB_vertex_array_object: i32 = libdragon_sys::GL_ARB_vertex_array_object as i32;
pub const ARB_matrix_palette: i32 = libdragon_sys::GL_ARB_matrix_palette as i32;
pub const N64_RDPQ_interop: i32 = libdragon_sys::GL_N64_RDPQ_interop as i32;
pub const N64_surface_image: i32 = libdragon_sys::GL_N64_surface_image as i32;
pub const N64_half_fixed_point: i32 = libdragon_sys::GL_N64_half_fixed_point as i32;
pub const N64_reduced_aliasing: i32 = libdragon_sys::GL_N64_reduced_aliasing as i32;
pub const N64_interpenetrating: i32 = libdragon_sys::GL_N64_interpenetrating as i32;
pub const N64_dither_mode: i32 = libdragon_sys::GL_N64_dither_mode as i32;
pub const N64_copy_matrix: i32 = libdragon_sys::GL_N64_copy_matrix as i32;
pub const N64_texture_flip: i32 = libdragon_sys::GL_N64_texture_flip as i32;
pub const FALSE: i32 = libdragon_sys::GL_FALSE as i32;
pub const TRUE: i32 = libdragon_sys::GL_TRUE as i32;

#[inline(always)]
pub fn GetError() -> i32 { unsafe { libdragon_sys::glGetError() as i32 } }

#[inline(always)]
pub fn Enable(target: i32) { unsafe { libdragon_sys::glEnable(target as u32) } }
#[inline(always)]
pub fn Disable(target: i32) { unsafe { libdragon_sys::glDisable(target as u32) } }

#[inline(always)]
pub fn Begin(mode: i32) { unsafe { libdragon_sys::glBegin(mode as u32) } }
#[inline(always)]
pub fn End() { unsafe { libdragon_sys::glEnd() } }
#[inline(always)]
pub fn Vertex2s(x: i16, y: i16) { unsafe { libdragon_sys::glVertex2s(x, y) } }
#[inline(always)]
pub fn Vertex2i(x: i32, y: i32) { unsafe { libdragon_sys::glVertex2i(x, y) } }
#[inline(always)]
pub fn Vertex2f(x: f32, y: f32) { unsafe { libdragon_sys::glVertex2f(x, y) } }
#[inline(always)]
pub fn Vertex2d(x: f64, y: f64) { unsafe { libdragon_sys::glVertex2d(x, y) } }
#[inline(always)]
pub fn Vertex2hxN64(x: i16, y: i16) { unsafe { libdragon_sys::glVertex2hxN64(x, y) } }
#[inline(always)]
pub fn Vertex3s(x: i16, y: i16, z: i16) { unsafe { libdragon_sys::glVertex3s(x, y, z) } }
#[inline(always)]
pub fn Vertex3i(x: i32, y: i32, z: i32) { unsafe { libdragon_sys::glVertex3i(x, y, z) } }
#[inline(always)]
pub fn Vertex3f(x: f32, y: f32, z: f32) { unsafe { libdragon_sys::glVertex3f(x, y, z) } }
#[inline(always)]
pub fn Vertex3d(x: f64, y: f64, z: f64) { unsafe { libdragon_sys::glVertex3d(x, y, z) } }
#[inline(always)]
pub fn Vertex3hxN64(x: i16, y: i16, z: i16) { unsafe { libdragon_sys::glVertex3hxN64(x, y, z) } }
#[inline(always)]
pub fn Vertex4s(x: i16, y: i16, z: i16, w: i16) { unsafe { libdragon_sys::glVertex4s(x, y, z, w) } }
#[inline(always)]
pub fn Vertex4i(x: i32, y: i32, z: i32, w: i32) { unsafe { libdragon_sys::glVertex4i(x, y, z, w) } }
#[inline(always)]
pub fn Vertex4f(x: f32, y: f32, z: f32, w: f32) { unsafe { libdragon_sys::glVertex4f(x, y, z, w) } }
#[inline(always)]
pub fn Vertex4d(x: f64, y: f64, z: f64, w: f64) { unsafe { libdragon_sys::glVertex4d(x, y, z, w) } }
#[inline(always)]
pub fn Vertex4hxN64(x: i16, y: i16, z: i16, w: i16) { unsafe { libdragon_sys::glVertex4hxN64(x, y, z, w) } }
#[inline(always)]
pub fn Vertex2sv(v: &[i16]) { unsafe { libdragon_sys::glVertex2sv(v.as_ptr()) } }
#[inline(always)]
pub fn Vertex2iv(v: &[i32]) { unsafe { libdragon_sys::glVertex2iv(v.as_ptr()) } }
#[inline(always)]
pub fn Vertex2fv(v: &[f32]) { unsafe { libdragon_sys::glVertex2fv(v.as_ptr()) } }
#[inline(always)]
pub fn Vertex2dv(v: &[f64]) { unsafe { libdragon_sys::glVertex2dv(v.as_ptr()) } }
#[inline(always)]
pub fn Vertex2hxvN64(v: &[i16]) { unsafe { libdragon_sys::glVertex2hxvN64(v.as_ptr()) } }
#[inline(always)]
pub fn Vertex3sv(v: &[i16]) { unsafe { libdragon_sys::glVertex3sv(v.as_ptr()) } }
#[inline(always)]
pub fn Vertex3iv(v: &[i32]) { unsafe { libdragon_sys::glVertex3iv(v.as_ptr()) } }
#[inline(always)]
pub fn Vertex3fv(v: &[f32]) { unsafe { libdragon_sys::glVertex3fv(v.as_ptr()) } }
#[inline(always)]
pub fn Vertex3dv(v: &[f64]) { unsafe { libdragon_sys::glVertex3dv(v.as_ptr()) } }
#[inline(always)]
pub fn Vertex3hxvN64(v: &[i16]) { unsafe { libdragon_sys::glVertex3hxvN64(v.as_ptr()) } }
#[inline(always)]
pub fn Vertex4sv(v: &[i16]) { unsafe { libdragon_sys::glVertex4sv(v.as_ptr()) } }
#[inline(always)]
pub fn Vertex4iv(v: &[i32]) { unsafe { libdragon_sys::glVertex4iv(v.as_ptr()) } }
#[inline(always)]
pub fn Vertex4fv(v: &[f32]) { unsafe { libdragon_sys::glVertex4fv(v.as_ptr()) } }
#[inline(always)]
pub fn Vertex4dv(v: &[f64]) { unsafe { libdragon_sys::glVertex4dv(v.as_ptr()) } }
#[inline(always)]
pub fn Vertex4hxvN64(v: &[i16]) { unsafe { libdragon_sys::glVertex4hxvN64(v.as_ptr()) } }
#[inline(always)]
pub fn TexCoord1s(s: i16) { unsafe { libdragon_sys::glTexCoord1s(s) } }
#[inline(always)]
pub fn TexCoord1i(s: i32) { unsafe { libdragon_sys::glTexCoord1i(s) } }
#[inline(always)]
pub fn TexCoord1f(s: f32) { unsafe { libdragon_sys::glTexCoord1f(s) } }
#[inline(always)]
pub fn TexCoord1d(s: f64) { unsafe { libdragon_sys::glTexCoord1d(s) } }
#[inline(always)]
pub fn TexCoord1hxN64(s: i16) { unsafe { libdragon_sys::glTexCoord1hxN64(s) } }
#[inline(always)]
pub fn TexCoord2s(s: i16, t: i16) { unsafe { libdragon_sys::glTexCoord2s(s, t) } }
#[inline(always)]
pub fn TexCoord2i(s: i32, t: i32) { unsafe { libdragon_sys::glTexCoord2i(s, t) } }
#[inline(always)]
pub fn TexCoord2f(s: f32, t: f32) { unsafe { libdragon_sys::glTexCoord2f(s, t) } }
#[inline(always)]
pub fn TexCoord2d(s: f64, t: f64) { unsafe { libdragon_sys::glTexCoord2d(s, t) } }
#[inline(always)]
pub fn TexCoord2hxN64(s: i16, t: i16) { unsafe { libdragon_sys::glTexCoord2hxN64(s, t) } }
#[inline(always)]
pub fn TexCoord3s(s: i16, t: i16, r: i16) { unsafe { libdragon_sys::glTexCoord3s(s, t, r) } }
#[inline(always)]
pub fn TexCoord3i(s: i32, t: i32, r: i32) { unsafe { libdragon_sys::glTexCoord3i(s, t, r) } }
#[inline(always)]
pub fn TexCoord3f(s: f32, t: f32, r: f32) { unsafe { libdragon_sys::glTexCoord3f(s, t, r) } }
#[inline(always)]
pub fn TexCoord3d(s: f64, t: f64, r: f64) { unsafe { libdragon_sys::glTexCoord3d(s, t, r) } }
#[inline(always)]
pub fn TexCoord3hxN64(s: i16, t: i16, r: i16) { unsafe { libdragon_sys::glTexCoord3hxN64(s, t, r) } }
#[inline(always)]
pub fn TexCoord4s(s: i16, t: i16, r: i16, q: i16) { unsafe { libdragon_sys::glTexCoord4s(s, t, r, q) } }
#[inline(always)]
pub fn TexCoord4i(s: i32, t: i32, r: i32, q: i32) { unsafe { libdragon_sys::glTexCoord4i(s, t, r, q) } }
#[inline(always)]
pub fn TexCoord4f(s: f32, t: f32, r: f32, q: f32) { unsafe { libdragon_sys::glTexCoord4f(s, t, r, q) } }
#[inline(always)]
pub fn TexCoord4d(s: f64, t: f64, r: f64, q: f64) { unsafe { libdragon_sys::glTexCoord4d(s, t, r, q) } }
#[inline(always)]
pub fn TexCoord4hxN64(s: i16, t: i16, r: i16, q: i16) { unsafe { libdragon_sys::glTexCoord4hxN64(s, t, r, q) } }
#[inline(always)]
pub fn TexCoord1sv(v: &[i16]) { unsafe { libdragon_sys::glTexCoord1sv(v.as_ptr()) } }
#[inline(always)]
pub fn TexCoord1iv(v: &[i32]) { unsafe { libdragon_sys::glTexCoord1iv(v.as_ptr()) } }
#[inline(always)]
pub fn TexCoord1fv(v: &[f32]) { unsafe { libdragon_sys::glTexCoord1fv(v.as_ptr()) } }
#[inline(always)]
pub fn TexCoord1dv(v: &[f64]) { unsafe { libdragon_sys::glTexCoord1dv(v.as_ptr()) } }
#[inline(always)]
pub fn TexCoord1hxvN64(v: &[i16]) { unsafe { libdragon_sys::glTexCoord1hxvN64(v.as_ptr()) } }
#[inline(always)]
pub fn TexCoord2sv(v: &[i16]) { unsafe { libdragon_sys::glTexCoord2sv(v.as_ptr()) } }
#[inline(always)]
pub fn TexCoord2iv(v: &[i32]) { unsafe { libdragon_sys::glTexCoord2iv(v.as_ptr()) } }
#[inline(always)]
pub fn TexCoord2fv(v: &[f32]) { unsafe { libdragon_sys::glTexCoord2fv(v.as_ptr()) } }
#[inline(always)]
pub fn TexCoord2dv(v: &[f64]) { unsafe { libdragon_sys::glTexCoord2dv(v.as_ptr()) } }
#[inline(always)]
pub fn TexCoord2hxvN64(v: &[i16]) { unsafe { libdragon_sys::glTexCoord2hxvN64(v.as_ptr()) } }
#[inline(always)]
pub fn TexCoord3sv(v: &[i16]) { unsafe { libdragon_sys::glTexCoord3sv(v.as_ptr()) } }
#[inline(always)]
pub fn TexCoord3iv(v: &[i32]) { unsafe { libdragon_sys::glTexCoord3iv(v.as_ptr()) } }
#[inline(always)]
pub fn TexCoord3fv(v: &[f32]) { unsafe { libdragon_sys::glTexCoord3fv(v.as_ptr()) } }
#[inline(always)]
pub fn TexCoord3dv(v: &[f64]) { unsafe { libdragon_sys::glTexCoord3dv(v.as_ptr()) } }
#[inline(always)]
pub fn TexCoord3hxvN64(v: &[i16]) { unsafe { libdragon_sys::glTexCoord3hxvN64(v.as_ptr()) } }
#[inline(always)]
pub fn TexCoord4sv(v: &[i16]) { unsafe { libdragon_sys::glTexCoord4sv(v.as_ptr()) } }
#[inline(always)]
pub fn TexCoord4iv(v: &[i32]) { unsafe { libdragon_sys::glTexCoord4iv(v.as_ptr()) } }
#[inline(always)]
pub fn TexCoord4fv(v: &[f32]) { unsafe { libdragon_sys::glTexCoord4fv(v.as_ptr()) } }
#[inline(always)]
pub fn TexCoord4dv(v: &[f64]) { unsafe { libdragon_sys::glTexCoord4dv(v.as_ptr()) } }
#[inline(always)]
pub fn TexCoord4hxvN64(v: &[i16]) { unsafe { libdragon_sys::glTexCoord4hxvN64(v.as_ptr()) } }
#[inline(always)]
pub fn Normal3b(nx: i8, ny: i8, nz: i8) { unsafe { libdragon_sys::glNormal3b(nx, ny, nz) } }
#[inline(always)]
pub fn Normal3s(nx: i16, ny: i16, nz: i16) { unsafe { libdragon_sys::glNormal3s(nx, ny, nz) } }
#[inline(always)]
pub fn Normal3i(nx: i32, ny: i32, nz: i32) { unsafe { libdragon_sys::glNormal3i(nx, ny, nz) } }
#[inline(always)]
pub fn Normal3f(nx: f32, ny: f32, nz: f32) { unsafe { libdragon_sys::glNormal3f(nx, ny, nz) } }
#[inline(always)]
pub fn Normal3d(nx: f64, ny: f64, nz: f64) { unsafe { libdragon_sys::glNormal3d(nx, ny, nz) } }
#[inline(always)]
pub fn Normal3bv(v: &[i8]) { unsafe { libdragon_sys::glNormal3bv(v.as_ptr()) } }
#[inline(always)]
pub fn Normal3sv(v: &[i16]) { unsafe { libdragon_sys::glNormal3sv(v.as_ptr()) } }
#[inline(always)]
pub fn Normal3iv(v: &[i32]) { unsafe { libdragon_sys::glNormal3iv(v.as_ptr()) } }
#[inline(always)]
pub fn Normal3fv(v: &[f32]) { unsafe { libdragon_sys::glNormal3fv(v.as_ptr()) } }
#[inline(always)]
pub fn Normal3dv(v: &[f64]) { unsafe { libdragon_sys::glNormal3dv(v.as_ptr()) } }
#[inline(always)]
pub fn Color3b(r: i8, g: i8, b: i8) { unsafe { libdragon_sys::glColor3b(r, g, b) } }
#[inline(always)]
pub fn Color3s(r: i16, g: i16, b: i16) { unsafe { libdragon_sys::glColor3s(r, g, b) } }
#[inline(always)]
pub fn Color3i(r: i32, g: i32, b: i32) { unsafe { libdragon_sys::glColor3i(r, g, b) } }
#[inline(always)]
pub fn Color3f(r: f32, g: f32, b: f32) { unsafe { libdragon_sys::glColor3f(r, g, b) } }
#[inline(always)]
pub fn Color3d(r: f64, g: f64, b: f64) { unsafe { libdragon_sys::glColor3d(r, g, b) } }
#[inline(always)]
pub fn Color3ub(r: u8, g: u8, b: u8) { unsafe { libdragon_sys::glColor3ub(r, g, b) } }
#[inline(always)]
pub fn Color3us(r: u16, g: u16, b: u16) { unsafe { libdragon_sys::glColor3us(r, g, b) } }
#[inline(always)]
pub fn Color3ui(r: u32, g: u32, b: u32) { unsafe { libdragon_sys::glColor3ui(r, g, b) } }
#[inline(always)]
pub fn Color4b(r: i8, g: i8, b: i8, a: i8) { unsafe { libdragon_sys::glColor4b(r, g, b, a) } }
#[inline(always)]
pub fn Color4s(r: i16, g: i16, b: i16, a: i16) { unsafe { libdragon_sys::glColor4s(r, g, b, a) } }
#[inline(always)]
pub fn Color4i(r: i32, g: i32, b: i32, a: i32) { unsafe { libdragon_sys::glColor4i(r, g, b, a) } }
#[inline(always)]
pub fn Color4f(r: f32, g: f32, b: f32, a: f32) { unsafe { libdragon_sys::glColor4f(r, g, b, a) } }
#[inline(always)]
pub fn Color4d(r: f64, g: f64, b: f64, a: f64) { unsafe { libdragon_sys::glColor4d(r, g, b, a) } }
#[inline(always)]
pub fn Color4ub(r: u8, g: u8, b: u8, a: u8) { unsafe { libdragon_sys::glColor4ub(r, g, b, a) } }
#[inline(always)]
pub fn Color4us(r: u16, g: u16, b: u16, a: u16) { unsafe { libdragon_sys::glColor4us(r, g, b, a) } }
#[inline(always)]
pub fn Color4ui(r: u32, g: u32, b: u32, a: u32) { unsafe { libdragon_sys::glColor4ui(r, g, b, a) } }
#[inline(always)]
pub fn Color3bv(v: &[i8]) { unsafe { libdragon_sys::glColor3bv(v.as_ptr()) } }
#[inline(always)]
pub fn Color3sv(v: &[i16]) { unsafe { libdragon_sys::glColor3sv(v.as_ptr()) } }
#[inline(always)]
pub fn Color3iv(v: &[i32]) { unsafe { libdragon_sys::glColor3iv(v.as_ptr()) } }
#[inline(always)]
pub fn Color3fv(v: &[f32]) { unsafe { libdragon_sys::glColor3fv(v.as_ptr()) } }
#[inline(always)]
pub fn Color3dv(v: &[f64]) { unsafe { libdragon_sys::glColor3dv(v.as_ptr()) } }
#[inline(always)]
pub fn Color3ubv(v: &[u8]) { unsafe { libdragon_sys::glColor3ubv(v.as_ptr()) } }
#[inline(always)]
pub fn Color3usv(v: &[u16]) { unsafe { libdragon_sys::glColor3usv(v.as_ptr()) } }
#[inline(always)]
pub fn Color3uiv(v: &[u32]) { unsafe { libdragon_sys::glColor3uiv(v.as_ptr()) } }
#[inline(always)]
pub fn Color4bv(v: &[i8]) { unsafe { libdragon_sys::glColor4bv(v.as_ptr()) } }
#[inline(always)]
pub fn Color4sv(v: &[i16]) { unsafe { libdragon_sys::glColor4sv(v.as_ptr()) } }
#[inline(always)]
pub fn Color4iv(v: &[i32]) { unsafe { libdragon_sys::glColor4iv(v.as_ptr()) } }
#[inline(always)]
pub fn Color4fv(v: &[f32]) { unsafe { libdragon_sys::glColor4fv(v.as_ptr()) } }
#[inline(always)]
pub fn Color4dv(v: &[f64]) { unsafe { libdragon_sys::glColor4dv(v.as_ptr()) } }
#[inline(always)]
pub fn Color4ubv(v: &[u8]) { unsafe { libdragon_sys::glColor4ubv(v.as_ptr()) } }
#[inline(always)]
pub fn Color4usv(v: &[u16]) { unsafe { libdragon_sys::glColor4usv(v.as_ptr()) } }
#[inline(always)]
pub fn Color4uiv(v: &[u32]) { unsafe { libdragon_sys::glColor4uiv(v.as_ptr()) } }

#[inline(always)]
pub fn MatrixIndexubvARB(size: i32, v: &[u8]) { unsafe { libdragon_sys::glMatrixIndexubvARB(size, v.as_ptr()) } }
#[inline(always)]
pub fn MatrixIndexusvARB(size: i32, v: &[u16]) { unsafe { libdragon_sys::glMatrixIndexusvARB(size, v.as_ptr()) } }
#[inline(always)]
pub fn MatrixIndexuivARB(size: i32, v: &[u32]) { unsafe { libdragon_sys::glMatrixIndexuivARB(size, v.as_ptr()) } }

#[inline(always)]
pub fn VertexHalfFixedPrecisionN64(bits: u32) { unsafe { libdragon_sys::glVertexHalfFixedPrecisionN64(bits) } }
#[inline(always)]
pub fn TexCoordHalfFixedPrecisionN64(bits: u32) { unsafe { libdragon_sys::glTexCoordHalfFixedPrecisionN64(bits) } }

#[inline(always)]
pub fn VertexPointer<T>(size: i32, type_: i32, stride: usize, pointer: *const T) { unsafe { libdragon_sys::glVertexPointer(size, type_ as u32, stride as u32, pointer as *const ::core::ffi::c_void) } }
#[inline(always)]
pub fn TexCoordPointer<T>(size: i32, type_: i32, stride: usize, pointer: *const T) { unsafe { libdragon_sys::glTexCoordPointer(size, type_ as u32, stride as u32, pointer as *const ::core::ffi::c_void) } }
#[inline(always)]
pub fn NormalPointer<T>(type_: i32, stride: usize, pointer: *const T) { unsafe { libdragon_sys::glNormalPointer(type_ as u32, stride as u32, pointer as *const ::core::ffi::c_void) } }
#[inline(always)]
pub fn ColorPointer<T>(size: i32, type_: i32, stride: usize, pointer: *const T) { unsafe { libdragon_sys::glColorPointer(size, type_ as u32, stride as u32, pointer as *const ::core::ffi::c_void) } }
#[inline(always)]
pub fn MatrixIndexPointerARB<T>(size: i32, type_: i32, stride: usize, pointer: *const T) { unsafe { libdragon_sys::glMatrixIndexPointerARB(size, type_ as u32, stride as u32, pointer as *const ::core::ffi::c_void) } }

#[inline(always)]
pub fn EnableClientState(array: i32) { unsafe { libdragon_sys::glEnableClientState(array as u32) } }
#[inline(always)]
pub fn DisableClientState(array: i32) { unsafe { libdragon_sys::glEnableClientState(array as u32) } }

#[inline(always)]
pub fn ArrayElement(i: i32) { unsafe { libdragon_sys::glArrayElement(i) } }
#[inline(always)]
pub fn DrawArrays(mode: i32, first: i32, count: usize) { unsafe { libdragon_sys::glDrawArrays(mode as u32, first, count as u32) } }
#[inline(always)]
pub fn DrawElements<T>(mode: i32, count: usize, type_: i32, pointer: *const T) { unsafe { libdragon_sys::glDrawElements(mode as u32, count as u32, type_ as u32, pointer as *const ::core::ffi::c_void) } }
#[inline(always)]
pub fn InterleavedArrays<T>(format: i32, stride: usize, pointer: *const T) { unsafe { libdragon_sys::glInterleavedArrays(format as u32, stride as u32, pointer as *const ::core::ffi::c_void) } }

#[inline(always)]
pub fn GenVertexArrays(arrays: &mut [u32]) { unsafe { libdragon_sys::glGenVertexArrays(arrays.len() as u32, arrays.as_mut_ptr()) } }
#[inline(always)]
pub fn DeleteVertexArrays(arrays: &[u32]) { unsafe { libdragon_sys::glDeleteVertexArrays(arrays.len() as u32, arrays.as_ptr()) } }
#[inline(always)]
pub fn BindVertexArray(array: u32) { unsafe { libdragon_sys::glBindVertexArray(array) } }
#[inline(always)]
pub fn IsVertexArray(array: u32) -> bool { unsafe { libdragon_sys::glIsVertexArray(array) != 0 } }
#[inline(always)]
pub fn BindBufferARB(target: i32, buffer: u32) { unsafe { libdragon_sys::glBindBufferARB(target as u32, buffer) } }
#[inline(always)]
pub fn DeletBuffersARB(buffers: &[u32]) { unsafe { libdragon_sys::glDeleteBuffersARB(buffers.len() as u32, buffers.as_ptr()) } }
#[inline(always)]
pub fn GenBuffersARB(buffers: &mut [u32]) { unsafe { libdragon_sys::glGenBuffersARB(buffers.len() as u32, buffers.as_mut_ptr()) } }
#[inline(always)]
pub fn IsBufferARB(buffer: u32) -> bool { unsafe { libdragon_sys::glIsBufferARB(buffer) != 0 } }
#[inline(always)]
pub fn BufferDataARB<T>(target: i32, size: usize, pointer: *const T, usage: i32) { unsafe { libdragon_sys::glBufferDataARB(target as u32, size, pointer as *const ::core::ffi::c_void, usage as u32) } }
#[inline(always)]
pub fn BufferSubDataARB<T>(target: i32, offset: isize, size: usize, data: *const T) { unsafe { libdragon_sys::glBufferSubDataARB(target as u32, offset, size, data as *const ::core::ffi::c_void) } }
#[inline(always)]
pub fn GetBufferSubDataARB<T>(target: i32, offset: isize, size: usize, data: *mut T) { unsafe { libdragon_sys::glGetBufferSubDataARB(target as u32, offset, size, data as *mut ::core::ffi::c_void) } }
#[inline(always)]
pub fn MapBufferARB<T>(target: i32, access: i32) -> *mut T { unsafe { libdragon_sys::glMapBufferARB(target as u32, access as u32) as *mut T } }
#[inline(always)]
pub fn UnmapBufferARB(target: i32) -> bool { unsafe { libdragon_sys::glUnmapBufferARB(target as u32) != 0 } }

#[inline(always)]
pub fn GetBufferParameterivARB<T>(target: i32, pname: i32, params: &mut [T]) { unsafe { libdragon_sys::glGetBufferParameterivARB(target as u32, pname as u32, params.as_mut_ptr() as *mut ::core::ffi::c_int) } }

// This function needs some improvment
#[inline(always)]
pub fn GetBufferPointervARB<T>(target: i32, pname: i32, params: &mut [*mut T]) { unsafe { libdragon_sys::glGetBufferPointervARB(target as u32, pname as u32, params.as_mut_ptr() as *mut *mut ::core::ffi::c_void) } }

#[inline(always)]
pub fn Rects(x1: i16, y1: i16, x2: i16, y2: i16) { unsafe { libdragon_sys::glRects(x1, y1, x2, y2) } }
#[inline(always)]
pub fn Recti(x1: i32, y1: i32, x2: i32, y2: i32) { unsafe { libdragon_sys::glRecti(x1, y1, x2, y2) } }
#[inline(always)]
pub fn Rectf(x1: f32, y1: f32, x2: f32, y2: f32) { unsafe { libdragon_sys::glRectf(x1, y1, x2, y2) } }
#[inline(always)]
pub fn Rectd(x1: f64, y1: f64, x2: f64, y2: f64) { unsafe { libdragon_sys::glRectd(x1, y1, x2, y2) } }
#[inline(always)]
pub fn Rectsv(v1: &[i16], v2: &[i16]) { unsafe { libdragon_sys::glRectsv(v1.as_ptr(), v2.as_ptr()) } }
#[inline(always)]
pub fn Rectiv(v1: &[i32], v2: &[i32]) { unsafe { libdragon_sys::glRectiv(v1.as_ptr(), v2.as_ptr()) } }
#[inline(always)]
pub fn Rectfv(v1: &[f32], v2: &[f32]) { unsafe { libdragon_sys::glRectfv(v1.as_ptr(), v2.as_ptr()) } }
#[inline(always)]
pub fn Rectdv(v1: &[f64], v2: &[f64]) { unsafe { libdragon_sys::glRectdv(v1.as_ptr(), v2.as_ptr()) } }

#[inline(always)]
pub fn DepthRange(n: f64, f: f64) { unsafe { libdragon_sys::glDepthRange(n, f) } }

#[inline(always)]
pub fn Viewport(x: i32, y: i32, w: u32, h: u32) { unsafe { libdragon_sys::glViewport(x, y, w, h) } }

#[inline(always)]
pub fn MatrixMode(mode: i32) { unsafe { libdragon_sys::glMatrixMode(mode as u32) } }
#[inline(always)]
pub fn LoadMatrixf(m: &[f32]) { unsafe { libdragon_sys::glLoadMatrixf(m.as_ptr()) } }
#[inline(always)]
pub fn LoadMatrixd(m: &[f64]) { unsafe { libdragon_sys::glLoadMatrixd(m.as_ptr()) } }
#[inline(always)]
pub fn MultMatrixf(m: &[f32]) { unsafe { libdragon_sys::glMultMatrixf(m.as_ptr()) } }
#[inline(always)]
pub fn MultMatrixd(m: &[f64]) { unsafe { libdragon_sys::glMultMatrixd(m.as_ptr()) } }
#[inline(always)]
pub fn LoadIdentity() { unsafe { libdragon_sys::glLoadIdentity() } }
#[inline(always)]
pub fn Rotatef(angle: f32, x: f32, y: f32, z: f32) { unsafe { libdragon_sys::glRotatef(angle, x, y, z) } }
#[inline(always)]
pub fn Rotated(angle: f64, x: f64, y: f64, z: f64) { unsafe { libdragon_sys::glRotated(angle, x, y, z) } }
#[inline(always)]
pub fn Translatef(x: f32, y: f32, z: f32) { unsafe { libdragon_sys::glTranslatef(x, y, z) } }
#[inline(always)]
pub fn Translated(x: f64, y: f64, z: f64) { unsafe { libdragon_sys::glTranslated(x, y, z) } }
#[inline(always)]
pub fn Scalef(x: f32, y: f32, z: f32) { unsafe { libdragon_sys::glScalef(x, y, z) } }
#[inline(always)]
pub fn Scaled(x: f64, y: f64, z: f64) { unsafe { libdragon_sys::glScaled(x, y, z) } }
#[inline(always)]
pub fn Frustum(l: f64, r: f64, b: f64, t: f64, n: f64, f: f64) { unsafe { libdragon_sys::glFrustum(l, r, b, t, n, f) } }
#[inline(always)]
pub fn Ortho(l: f64, r: f64, b: f64, t: f64, n: f64, f: f64) { unsafe { libdragon_sys::glOrtho(l, r, b, t, n, f) } }
#[inline(always)]
pub fn PushMatrix() { unsafe { libdragon_sys::glPushMatrix() } }
#[inline(always)]
pub fn PopMatrix() { unsafe { libdragon_sys::glPopMatrix() } }

#[inline(always)]
pub fn CurrentPaletteMatrixARB(index: i32) { unsafe { libdragon_sys::glCurrentPaletteMatrixARB(index) } }
#[inline(always)]
pub fn CopyMatrixN64(source: i32) { unsafe { libdragon_sys::glCopyMatrixN64(source as u32) } }

#[inline(always)]
pub fn TexGeni(coord: i32, pname: i32, param: i32) { unsafe { libdragon_sys::glTexGeni(coord as u32, pname as u32, param) } }
#[inline(always)]
pub fn TexGenf(coord: i32, pname: i32, param: f32) { unsafe { libdragon_sys::glTexGenf(coord as u32, pname as u32, param) } }
#[inline(always)]
pub fn TexGend(coord: i32, pname: i32, param: f64) { unsafe { libdragon_sys::glTexGend(coord as u32, pname as u32, param) } }

#[inline(always)]
pub fn TexGeniv(coord: i32, pname: i32, param: &[i32]) { unsafe { libdragon_sys::glTexGeniv(coord as u32, pname as u32, param.as_ptr()) } }
#[inline(always)]
pub fn TexGenfv(coord: i32, pname: i32, param: &[f32]) { unsafe { libdragon_sys::glTexGenfv(coord as u32, pname as u32, param.as_ptr()) } }
#[inline(always)]
pub fn TexGendv(coord: i32, pname: i32, param: &[f64]) { unsafe { libdragon_sys::glTexGendv(coord as u32, pname as u32, param.as_ptr()) } }

#[inline(always)]
pub fn Materiali(face: i32, pname: i32, param: i32) { unsafe { libdragon_sys::glMateriali(face as u32, pname as u32, param) } }
#[inline(always)]
pub fn Materialf(face: i32, pname: i32, param: f32) { unsafe { libdragon_sys::glMaterialf(face as u32, pname as u32, param) } }
#[inline(always)]
pub fn Materialiv(face: i32, pname: i32, param: &[i32]) { unsafe { libdragon_sys::glMaterialiv(face as u32, pname as u32, param.as_ptr()) } }
#[inline(always)]
pub fn Materialfv(face: i32, pname: i32, param: &[f32]) { unsafe { libdragon_sys::glMaterialfv(face as u32, pname as u32, param.as_ptr()) } }
#[inline(always)]
pub fn Lighti(light: i32, pname: i32, param: i32) { unsafe { libdragon_sys::glLighti(light as u32, pname as u32, param) } }
#[inline(always)]
pub fn Lightf(light: i32, pname: i32, param: f32) { unsafe { libdragon_sys::glLightf(light as u32, pname as u32, param) } }
#[inline(always)]
pub fn Lightiv(light: i32, pname: i32, param: &[i32]) { unsafe { libdragon_sys::glLightiv(light as u32, pname as u32, param.as_ptr()) } }
#[inline(always)]
pub fn Lightfv(light: i32, pname: i32, param: &[f32]) { unsafe { libdragon_sys::glLightfv(light as u32, pname as u32, param.as_ptr()) } }
#[inline(always)]
pub fn LightModeli(pname: i32, param: i32) { unsafe { libdragon_sys::glLightModeli(pname as u32, param) } }
#[inline(always)]
pub fn LightModelf(pname: i32, param: f32) { unsafe { libdragon_sys::glLightModelf(pname as u32, param) } }
#[inline(always)]
pub fn LightModeliv(pname: i32, param: &[i32]) { unsafe { libdragon_sys::glLightModeliv(pname as u32, param.as_ptr()) } }
#[inline(always)]
pub fn LightModelfv(pname: i32, param: &[f32]) { unsafe { libdragon_sys::glLightModelfv(pname as u32, param.as_ptr()) } }
#[inline(always)]
pub fn ColorMaterial(face: i32, mode: i32) { unsafe { libdragon_sys::glColorMaterial(face as u32, mode as u32) } }
#[inline(always)]
pub fn ShadeModel(mode: i32) { unsafe { libdragon_sys::glShadeModel(mode as u32) } }

#[inline(always)]
pub fn PointSize(size: f32) { unsafe { libdragon_sys::glPointSize(size) } }
#[inline(always)]
pub fn LineWidth(width: f32) { unsafe { libdragon_sys::glLineWidth(width) } }

#[inline(always)]
pub fn CullFace(mode: i32) { unsafe { libdragon_sys::glCullFace(mode as u32) } }
#[inline(always)]
pub fn FrontFace(dir: i32) { unsafe { libdragon_sys::glFrontFace(dir as u32) } }
#[inline(always)]
pub fn PolygonMode(face: i32, mode: i32) { unsafe { libdragon_sys::glPolygonMode(face as u32, mode as u32) } }

#[inline(always)]
pub fn PixelStorei(pname: i32, param: i32) { unsafe { libdragon_sys::glPixelStorei(pname as u32, param) } }
#[inline(always)]
pub fn PixelStoref(pname: i32, param: f32) { unsafe { libdragon_sys::glPixelStoref(pname as u32, param) } }
#[inline(always)]
pub fn PixelTransferi(pname: i32, value: i32) { unsafe { libdragon_sys::glPixelTransferi(pname as u32, value) } }
#[inline(always)]
pub fn PixelTransferf(pname: i32, value: f32) { unsafe { libdragon_sys::glPixelTransferf(pname as u32, value) } }
#[inline(always)]
pub fn PixelMapusv(map: i32, size: u32, values: &[u16]) { unsafe { libdragon_sys::glPixelMapusv(map as u32, size, values.as_ptr()) } }
#[inline(always)]
pub fn PixelMapuiv(map: i32, size: u32, values: &[u32]) { unsafe { libdragon_sys::glPixelMapuiv(map as u32, size, values.as_ptr()) } }
#[inline(always)]
pub fn PixelMapfv(map: i32, size: u32, values: &[f32]) { unsafe { libdragon_sys::glPixelMapfv(map as u32, size, values.as_ptr()) } }

#[inline(always)]
pub fn TexImage1D<T>(target: i32, level: i32, internalformat: i32, width: u32, border: i32, format: i32, type_: i32, data: &[T]) {
    unsafe { libdragon_sys::glTexImage1D(target as u32, level, internalformat, width, border, format as u32, type_ as u32, data.as_ptr() as *const ::core::ffi::c_void) }
}

#[inline(always)]
pub fn TexImage2D<T>(target: i32, level: i32, internalformat: i32, width: u32, height: u32, border: i32, format: i32, type_: i32, data: &[T]) {
    unsafe { libdragon_sys::glTexImage2D(target as u32, level, internalformat, width, height, border, format as u32, type_ as u32, data.as_ptr() as *const ::core::ffi::c_void) }
}

#[inline(always)]
pub fn SurfaceTexImageN64(target: i32, level: i32, surface: &surface::Surface, texparms: rdpq::TexParms) {
    unsafe { libdragon_sys::glSurfaceTexImageN64(target as u32, level, surface.ptr, 
                                                 &mut Into::<libdragon_sys::rdpq_texparms_t>::into(texparms) as *mut libdragon_sys::rdpq_texparms_t) }
}

#[inline(always)]
pub fn SpriteTextureN64(target: i32, sprite: &sprite::Sprite, texparms: rdpq::TexParms) {
    unsafe {
        libdragon_sys::glSpriteTextureN64(target as u32, sprite.as_const_sprite_s() as *mut libdragon_sys::sprite_t, 
                                          &mut Into::<libdragon_sys::rdpq_texparms_t>::into(texparms) as *mut libdragon_sys::rdpq_texparms_t)
    }
}

#[inline(always)]
pub fn TexParameteri(target: i32, pname: i32, param: i32) { unsafe { libdragon_sys::glTexParameteri(target as u32, pname as u32, param) } }
#[inline(always)]
pub fn TexParameterf(target: i32, pname: i32, param: f32) { unsafe { libdragon_sys::glTexParameterf(target as u32, pname as u32, param) } }
#[inline(always)]
pub fn TexParameteriv(target: i32, pname: i32, param: &[i32]) { unsafe { libdragon_sys::glTexParameteriv(target as u32, pname as u32, param.as_ptr()) } }
#[inline(always)]
pub fn TexParameterfv(target: i32, pname: i32, param: &[f32]) { unsafe { libdragon_sys::glTexParameterfv(target as u32, pname as u32, param.as_ptr()) } }

#[inline(always)]
pub fn BindTexture(target: i32, texture: u32) { unsafe { libdragon_sys::glBindTexture(target as u32, texture) } }
#[inline(always)]
pub fn DeleteTextures(textures: &[u32]) { unsafe { libdragon_sys::glDeleteTextures(textures.len() as u32, textures.as_ptr()) } }
#[inline(always)]
pub fn GenTextures(textures: &mut [u32]) { unsafe { libdragon_sys::glGenTextures(textures.len() as u32, textures.as_mut_ptr()) } }

#[inline(always)]
pub fn AreTexturesResident(n: u32, textures: &[u32], residences: Option<&mut [u8]>) -> bool { 
    unsafe { libdragon_sys::glAreTexturesResident(n, textures.as_ptr(), residences.map_or(::core::ptr::null_mut(), |r| r.as_mut_ptr())) != 0 }
}

#[inline(always)]
pub fn PrioritizeTextures(n: u32, textures: &[u32], priorities: &[f32]) { unsafe { libdragon_sys::glPrioritizeTextures(n, textures.as_ptr(), priorities.as_ptr()) } }

#[inline(always)]
pub fn TexEnvi(target: i32, pname: i32, param: i32) { unsafe { libdragon_sys::glTexEnvi(target as u32, pname as u32, param) } }
#[inline(always)]
pub fn TexEnvf(target: i32, pname: i32, param: f32) { unsafe { libdragon_sys::glTexEnvf(target as u32, pname as u32, param) } }
#[inline(always)]
pub fn TexEnviv(target: i32, pname: i32, params: &[i32]) { unsafe { libdragon_sys::glTexEnviv(target as u32, pname as u32, params.as_ptr()) } }
#[inline(always)]
pub fn TexEnvfv(target: i32, pname: i32, params: &[f32]) { unsafe { libdragon_sys::glTexEnvfv(target as u32, pname as u32, params.as_ptr()) } }
#[inline(always)]
pub fn IsTexture(texture: u32) -> bool { unsafe { libdragon_sys::glIsTexture(texture) != 0 } }

#[inline(always)]
pub fn TexSizeN64(width: u16, height: u16) { unsafe { libdragon_sys::glTexSizeN64(width, height); } }

#[inline(always)]
pub fn Fogi(pname: i32, param: i32) { unsafe { libdragon_sys::glFogi(pname as u32, param) } }
#[inline(always)]
pub fn Fogf(pname: i32, param: f32) { unsafe { libdragon_sys::glFogf(pname as u32, param) } }
#[inline(always)]
pub fn Fogiv(pname: i32, param: &[i32]) { unsafe { libdragon_sys::glFogiv(pname as u32, param.as_ptr()) } }
#[inline(always)]
pub fn Fogfv(pname: i32, param: &[f32]) { unsafe { libdragon_sys::glFogfv(pname as u32, param.as_ptr()) } }

#[inline(always)]
pub fn DitherModeN64(mode: rdpq::Dither) { unsafe { libdragon_sys::glDitherModeN64(mode.into()) } }

#[inline(always)]
pub fn Scissor(left: i32, bottom: i32, width: u32, height: u32) { unsafe { libdragon_sys::glScissor(left, bottom, width, height) } }

#[inline(always)]
pub fn AlphaFunc(func: i32, ref_: f32) { unsafe { libdragon_sys::glAlphaFunc(func as u32, ref_) } }
#[inline(always)]
pub fn DepthFunc(func: i32) { unsafe { libdragon_sys::glDepthFunc(func as u32) } }
#[inline(always)]
pub fn BlendFunc(src: i32, dst: i32) { unsafe { libdragon_sys::glBlendFunc(src as u32, dst as u32) } }
#[inline(always)]
pub fn DepthMask(mask: i32) { unsafe { libdragon_sys::glDepthMask(mask as u8) } }
#[inline(always)]
pub fn ClearColor(r: f32, g: f32, b: f32, a: f32) { unsafe { libdragon_sys::glClearColor(r, g, b, a) } }
#[inline(always)]
pub fn Clear(buf: i32) { unsafe { libdragon_sys::glClear(buf as u32) } }
#[inline(always)]
pub fn ClearDepth(d: f64) { unsafe { libdragon_sys::glClearDepth(d) } }

#[inline(always)]
pub fn NewList(n: u32, mode: i32) { unsafe { libdragon_sys::glNewList(n, mode as u32) } }
#[inline(always)]
pub fn EndList() { unsafe { libdragon_sys::glEndList() } }
#[inline(always)]
pub fn CallList(n: u32) { unsafe { libdragon_sys::glCallList(n) } }
#[inline(always)]
pub fn CallLists(n: u32, type_: i32, lists: &[u32]) { unsafe { libdragon_sys::glCallLists(n, type_ as u32, lists.as_ptr() as *const ::core::ffi::c_void) } }
#[inline(always)]
pub fn ListBase(base: u32) { unsafe { libdragon_sys::glListBase(base) } }
#[inline(always)]
pub fn GenLists(s: usize) -> u32 { unsafe { libdragon_sys::glGenLists(s as u32) } }
#[inline(always)]
pub fn IsList(list: u32) -> bool { unsafe { libdragon_sys::glIsList(list) != 0 } }
#[inline(always)]
pub fn DeleteLists(list: u32, range: usize) { unsafe { libdragon_sys::glDeleteLists(list, range as u32) } }

#[inline(always)]
pub fn Flush() { unsafe { libdragon_sys::glFlush() } }
#[inline(always)]
pub fn Finish() { unsafe { libdragon_sys::glFinish() } }

#[inline(always)]
pub fn Hint(target: i32, hint: i32) { unsafe { libdragon_sys::glHint(target as u32, hint as u32) } }

#[inline(always)]
pub fn GetBooleanv(value: i32, data: &mut [u8]) { unsafe { libdragon_sys::glGetBooleanv(value as u32, data.as_mut_ptr()) } }
#[inline(always)]
pub fn GetIntegerv(value: i32, data: &mut [i32]) { unsafe { libdragon_sys::glGetIntegerv(value as u32, data.as_mut_ptr()) } }
#[inline(always)]
pub fn GetFloatv(value: i32, data: &mut [f32]) { unsafe { libdragon_sys::glGetFloatv(value as u32, data.as_mut_ptr()) } }
#[inline(always)]
pub fn GetDoublev(value: i32, data: &mut [f64]) { unsafe { libdragon_sys::glGetDoublev(value as u32, data.as_mut_ptr()) } }

#[inline(always)]
pub fn IsEnabled(value: i32) -> bool { unsafe { libdragon_sys::glIsEnabled(value as u32) != 0 } }

#[inline(always)]
pub fn GetPointerv<T>(pname: i32, params: &mut [*mut T]) { unsafe { libdragon_sys::glGetPointerv(pname as u32, params.as_mut_ptr() as *mut *mut ::core::ffi::c_void) } }

#[inline(always)]
pub fn GetString(name: i32) -> String { 
    let c_str = unsafe { CStr::from_ptr(libdragon_sys::glGetString(name as u32) as *const i8) };
    String::from_utf8_lossy(c_str.to_bytes()).to_string()
}

// gl_integration.h
pub fn init() {
    unsafe {
        libdragon_sys::gl_init();
    }
}

pub fn close() {
    unsafe {
        libdragon_sys::gl_close();
    }
}

pub fn context_begin() {
    unsafe {
        libdragon_sys::gl_context_begin();
    }
}

pub fn context_end() {
    unsafe {
        libdragon_sys::gl_context_end();
    }
}


