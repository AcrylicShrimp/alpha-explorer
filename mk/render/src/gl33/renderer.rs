use crate::{RenderMode, RenderRequest, Shader};
use bumpalo::collections::Vec as BumpVec;
use bumpalo::Bump;

pub struct Renderer<'bump> {
    bump: &'bump Bump,
    requests: BumpVec<'bump, RenderRequest<'bump>>,
}

impl<'bump> Renderer<'bump> {
    pub fn new(bump: &'bump Bump) -> Self {
        Self {
            bump,
            requests: BumpVec::new_in(bump),
        }
    }

    pub fn enqueue(
        &mut self,
        instance_count: u32,
        primitive_count: u32,
        mode: RenderMode,
        shader: &Shader,
        req: impl Fn(&mut RenderRequest),
    ) {
        let mut render_request =
            RenderRequest::new(self.bump, instance_count, primitive_count, mode, shader);
        req(&mut render_request);
        self.requests.push(render_request);
    }

    pub fn flush(&self) {
        for request in &self.requests {
            request.render();
        }
    }
}
