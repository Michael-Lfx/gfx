// Copyright 2014 The Gfx-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![crate_name = "gfx"]
#![comment = "A lightweight graphics device manager for Rust"]
#![license = "ASL2"]
#![crate_type = "lib"]

#![feature(phase)]

//! An efficient, low-level, bindless graphics API for Rust. See [the
//! blog](http://gfx-rs.github.io/) for explanations and annotated examples.

#[phase(plugin, link)] extern crate log;
extern crate libc;

extern crate device;
// when cargo is ready, re-enable the cfg's
/* #[cfg(glfw)] */ extern crate glfw_platform;
extern crate render;

// public re-exports
pub use render::{BufferHandle, ShaderHandle, ProgramHandle, SurfaceHandle, TextureHandle, SamplerHandle};
pub use render::Renderer;
pub use render::mesh::{Attribute, Mesh, VertexFormat, Slice, VertexSlice, IndexSlice};
pub use render::state::{DrawState, BlendAdditive, BlendAlpha};
pub use render::shade::{ParameterSink, ToUniform, ShaderParam, TextureParam,
    ParameterLinkError, ParameterError, ErrorInternal, ErrorUniform, ErrorBlock, ErrorTexture,
    FnUniform, FnBlock, FnTexture, VarUniform, VarBlock, VarTexture};
pub use render::target::{Frame, Plane, PlaneEmpty, PlaneSurface, PlaneTexture};
pub use device::{attrib, state, tex};
pub use device::target::{Color, ClearData, Layer, Level};
pub use device::{Blob, Device, GlBackEnd, GlProvider, GraphicsContext, InitError, QueueSize};
pub use device::shade::{UniformValue, ValueI32, ValueF32, ValueI32Vec, ValueF32Vec, ValueF32Matrix};
pub use device::shade::{ShaderSource, StaticBytes};
/* #[cfg(glfw)] */ pub use glfw = glfw_platform;


/// Start up a renderer and device.
pub fn start<C: GraphicsContext<GlBackEnd>, P: GlProvider>(graphics_context: C, provider: P, queue_size: QueueSize)
        -> Result<(Renderer, Device<render::Token, GlBackEnd, C>), InitError> {
    device::init(graphics_context, provider, queue_size).map(|(tx, rx, server, ack, should_finish)| {
        (Renderer::new(tx, rx, ack, should_finish), server)
    })
}
