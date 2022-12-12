use crate::{
    structure::{Mat33, Mat33Mut, Vec2, Vec3},
    transform::TransformManager,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Transform {
    pub position: Vec2,
    pub scale: Vec2,
    pub angle: f32,
}

impl Transform {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn translate_local(&mut self, d: Vec2) {
        let rad = self.angle.to_radians();
        let cos = rad.cos();
        let sin = rad.sin();
        self.translate_world(Vec2::new(cos * d.x - sin * d.y, sin * d.x + cos * d.y));
    }

    pub fn translate_world(&mut self, d: Vec2) {
        self.position += d;
    }

    pub fn fill_matrix<'a>(&self, mut matrix: Mat33Mut<'a>) -> Mat33Mut<'a> {
        matrix.set(Mat33::affine_srt(self.position, self.angle, self.scale).as_ref());
        matrix
    }

    pub fn fill_inverse_matrix<'a>(&self, mut matrix: Mat33Mut<'a>) -> Mat33Mut<'a> {
        matrix.set(
            Mat33::affine_trs(
                -self.position,
                -self.angle,
                Vec2::new(1f32 / self.scale.x, 1f32 / self.scale.y),
            )
            .as_ref(),
        );
        matrix
    }

    pub fn get_matrix(&self) -> Mat33 {
        let mut mat = Mat33::zero();
        self.fill_matrix(mat.as_mut());
        mat
    }

    pub fn get_inverse_matrix(&self) -> Mat33 {
        let mut mat = Mat33::zero();
        self.fill_inverse_matrix(mat.as_mut());
        mat
    }
}

impl Transform {
    pub fn world_position(transform: u32, transform_mgr: &TransformManager) -> Vec2 {
        let allocator = transform_mgr.allocator();
        let mut position = Vec3::zero_one() * allocator.transform(transform).get_matrix();

        for &transform in transform_mgr.hierarchy().parents(transform) {
            position *= allocator.transform(transform).get_matrix();
        }

        position.to_vec2()
    }

    pub fn set_world_position(
        transform: u32,
        transform_mgr: &mut TransformManager,
        position: Vec2,
    ) {
        let allocator = transform_mgr.allocator();
        let mut position = position.to_vec3(1f32);

        for &transform in transform_mgr.hierarchy().parents(transform).iter().rev() {
            position *= allocator.transform(transform).get_inverse_matrix();
        }

        transform_mgr
            .allocator_mut()
            .transform_mut(transform)
            .position = position.to_vec2();
        transform_mgr.hierarchy_mut().set_dirty(transform);
    }

    pub fn world_scale(transform: u32, transform_mgr: &TransformManager) -> Vec2 {
        let allocator = transform_mgr.allocator();
        let mut scale = allocator.transform(transform).scale;

        for &transform in transform_mgr.hierarchy().parents(transform) {
            scale *= allocator.transform(transform).scale;
        }

        scale
    }

    pub fn set_world_scale(transform: u32, transform_mgr: &mut TransformManager, mut scale: Vec2) {
        let allocator = transform_mgr.allocator();

        for &transform in transform_mgr.hierarchy().parents(transform).iter().rev() {
            scale /= allocator.transform(transform).scale;
        }

        transform_mgr.allocator_mut().transform_mut(transform).scale = scale;
        transform_mgr.hierarchy_mut().set_dirty(transform);
    }

    pub fn world_angle(transform: u32, transform_mgr: &TransformManager) -> f32 {
        let allocator = transform_mgr.allocator();
        let mut angle = allocator.transform(transform).angle;

        for &transform in transform_mgr.hierarchy().parents(transform) {
            angle += allocator.transform(transform).angle;
        }

        angle
    }

    pub fn set_world_angle(transform: u32, transform_mgr: &mut TransformManager, mut angle: f32) {
        let allocator = transform_mgr.allocator();

        for &transform in transform_mgr.hierarchy().parents(transform).iter().rev() {
            angle -= allocator.transform(transform).angle;
        }

        transform_mgr.allocator_mut().transform_mut(transform).angle = angle;
        transform_mgr.hierarchy_mut().set_dirty(transform);
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec2::zero(),
            scale: Vec2::one(),
            angle: 0f32,
        }
    }
}
