#![allow(unused_assignments)] // It seems that for parsing, they rely on functions that fit the naming conventions

use wgsl_rs::wgsl;

#[wgsl]
pub mod nv12_shader {
    use wgsl_rs::std::*;

    pub struct Uniforms {
        pub rect: Vec4f,
    }

    uniform!(group(0), binding(3), UNIFORMS: Uniforms);

    texture!(group(0), binding(0), TEX_Y: Texture2D<f32>);
    texture!(group(0), binding(1), TEX_UV: Texture2D<f32>);
    sampler!(group(0), binding(2), S: Sampler);

    #[output]
    pub struct VertexOutput {
        #[builtin(position)]
        pub position: Vec4f,
        #[location(0)]
        pub uv: Vec2f,
    }

    #[vertex]
    pub fn vs_main(#[builtin(vertex_index)] in_vertex_index: u32) -> VertexOutput {
        let rect = get!(UNIFORMS).rect;

        let mut pos = vec2f(0.0, 0.0);
        let mut uv = vec2f(0.0, 0.0);

        match in_vertex_index {
            0 => {
                pos = vec2f(rect.x, rect.y);
                uv = vec2f(0.0, 0.0);
            }
            1 => {
                pos = vec2f(rect.z, rect.y);
                uv = vec2f(1.0, 0.0);
            }
            2 => {
                pos = vec2f(rect.x, rect.w);
                uv = vec2f(0.0, 1.0);
            }
            3 => {
                pos = vec2f(rect.z, rect.y);
                uv = vec2f(1.0, 0.0);
            }
            4 => {
                pos = vec2f(rect.z, rect.w);
                uv = vec2f(1.0, 1.0);
            }
            _ => {
                pos = vec2f(rect.x, rect.w);
                uv = vec2f(0.0, 1.0);
            }
        }

        VertexOutput {
            position: vec4f(pos.x, pos.y, 1.0, 1.0),
            uv,
        }
    }

    #[fragment]
    pub fn fs_main(input: VertexOutput) -> Vec4f {
        let sample_uv = input.uv;

        // BT.709 YUV to RGB conversion
        let y = (texture_sample(TEX_Y, S, sample_uv).x - 0.0625) / 0.8588;
        let u = (texture_sample(TEX_UV, S, sample_uv).x - 0.5) / 0.8784;
        let v = (texture_sample(TEX_UV, S, sample_uv).y - 0.5) / 0.8784;

        let r = max(0.0, min(1.0, y + 1.5748 * v));
        let g = max(0.0, min(1.0, y - 0.1873 * u - 0.4681 * v));
        let b = max(0.0, min(1.0, y + 1.8556 * u));

        vec4f(r, g, b, 1.0)
    }
}

pub fn shader_source() -> String {
    crate::nv12_shader::WGSL_MODULE.wgsl_source().join("\n")
}
