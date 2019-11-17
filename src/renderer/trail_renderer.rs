//! A batched line renderer.

use camera::Camera;
use context::Context;
use na::{Matrix4, Point3};
use resource::{AllocationType, BufferType, Effect, GPUVec, ShaderAttribute, ShaderUniform};
use renderer::Renderer;
use utils::CyclicCounter;
use std::ptr;

#[path = "../error.rs"]
mod error;


pub struct TrailRenderer {
    shader: Effect,
    pos: ShaderAttribute<Point3<f32>>,
    color: ShaderUniform<Point3<f32>>,
    view: ShaderUniform<Matrix4<f32>>,
    proj: ShaderUniform<Matrix4<f32>>,
    vtx_end: CyclicCounter<u32>,
    ebo_inds_end: CyclicCounter<usize>,
    vtx: GPUVec<Point3<f32>>,
    ebo_inds: GPUVec<i32>,
    num_lines: i32,
}

impl TrailRenderer {
    // Creates a default trail renderer with one line segment from the origin
    pub fn default() -> TrailRenderer {
        TrailRenderer::from_points(
            vec![Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 1.0)]
        )
    }
    /// Creates a new trail lines manager of 'length' starting at 'from'
    pub fn from_point(length: usize, from: Point3<f32>) -> TrailRenderer {
        assert!(length > 1);
        TrailRenderer::from_points(vec![from; length])
    }

    pub fn from_points(data: Vec<Point3<f32>>) -> TrailRenderer {
        let mut shader = Effect::new_from_str(LINES_VERTEX_SRC, LINES_FRAGMENT_SRC);

        shader.use_program();
        let num_vertices = data.len();
        let num_lines = num_vertices - 1;
        let ebo_indices_length = 2 * num_lines;
        let mut vtx_end = CyclicCounter::exclusive_max(num_vertices as u32);
        let mut ebo_idx_end = CyclicCounter::exclusive_max(ebo_indices_length as usize);
        let indices: Vec<i32> = (0..num_lines).flat_map(|i| vec![i as i32, (i + 1) as i32]).collect();
        let indices: GPUVec<i32> = GPUVec::new(indices, BufferType::ElementArray, AllocationType::DynamicDraw);
        let vertices = GPUVec::new(data, BufferType::Array, AllocationType::DynamicDraw);

        let mut trail_renderer = TrailRenderer {
            vtx_end,
            vtx: vertices,
            ebo_inds_end: ebo_idx_end,
            ebo_inds: indices,
            num_lines: num_lines as i32,
            pos: shader
                .get_attrib::<Point3<f32>>("position")
                .expect("Failed to get shader attribute."),
            color: shader
                .get_uniform::<Point3<f32>>("color")
                .expect("Failed to get shader attribute."),
            view: shader
                .get_uniform::<Matrix4<f32>>("view")
                .expect("Failed to get shader uniform."),
            proj: shader
                .get_uniform::<Matrix4<f32>>("proj")
                .expect("Failed to get shader uniform."),
            shader: shader,
        };
        trail_renderer.vtx.load_to_gpu();
        trail_renderer.ebo_inds.load_to_gpu();
        trail_renderer
    }

    /// Adds a new line segment to the trail. This will overwrite the oldest segment and
    /// increment the index keeping track of where the newest part of the trail is
    pub fn push(&mut self, point: Point3<f32>) {
        self.vtx.replace_from(self.vtx_end.current() as usize, vec![point]);
        let new_ebo_indices = vec![self.ebo_inds_end.peek_last() as i32, self.ebo_inds_end.current() as i32];
        self.ebo_inds.replace_from(self.ebo_inds_end.current() as usize, new_ebo_indices);
        self.vtx_end.increment_one();
        self.ebo_inds_end.increment_by(2);
    }
}

impl Renderer for TrailRenderer {
    /// Actually draws the lines.
    fn render(&mut self, pass: usize, camera: &mut dyn Camera) {
        if self.vtx.len() == 0 {
            return;
        }

        self.shader.use_program();
        self.pos.enable();
        self.color.upload(&Point3::new(1.0, 1.0, 1.0));

        camera.upload(pass, &mut self.proj, &mut self.view);

        println!("Hey {:?}", self.ebo_inds.data());
        println!("Hey {:?}", self.vtx.data());
//        self.color.bind_sub_buffer(&mut self.vtx, 1, 1);
        self.pos.bind_sub_buffer(&mut self.vtx, 0, 0);

        let ctxt = Context::get();
        //        verify!(ctxt.draw_arrays(Context::LINES, 0, (self.lines.len() / 2) as i32));
        self.ebo_inds.bind_buffer();
        verify!(
        ctxt.draw_elements(
            Context::LINES,
            self.ebo_inds.len() as i32,
            gl::UNSIGNED_INT,
            0
        ));
//        self.ebo_inds.unbind();// MAYBE NOT UNBIND? THIS SHOULD NOT BE UNBOUND
        self.pos.disable();
//        self.color.disable();
    }
}

/// Vertex shader used by the material to display line.
pub static LINES_VERTEX_SRC: &'static str = A_VERY_LONG_STRING;
/// Fragment shader used by the material to display line.
pub static LINES_FRAGMENT_SRC: &'static str = ANOTHER_VERY_LONG_STRING;

const A_VERY_LONG_STRING: &'static str = "#version 100
    attribute vec3 position;
    uniform   vec3 color;
    varying   vec3 vColor;
    uniform   mat4 proj;
    uniform   mat4 view;
    void main() {
        gl_Position = proj * view * vec4(position, 1.0);
        vColor = color;
    }";

const ANOTHER_VERY_LONG_STRING: &'static str = "#version 100
#ifdef GL_FRAGMENT_PRECISION_HIGH
   precision highp float;
#else
   precision mediump float;
#endif

    varying vec3 vColor;
    void main() {
        gl_FragColor = vec4(vColor, 1.0);
    }";
