use crate::structure::Vec2;
use crate::transform::TransformManager;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Transform {
    pub position: Vec2,
    pub scale: Vec2,
    pub angle: f32,
    pub flags: u32,
}

impl Transform {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_dirty(&self) -> bool {
        (self.flags & 0b1000_0000_0000_0000_0000_0000_0000_0000) != 0
    }

    pub fn parent_index(&self) -> Option<u32> {
        match self.flags & 0b0111_1111_1111_1111_1111_1111_1111_1111 {
            0 => None,
            index => Some(index - 1),
        }
    }

    pub fn reset_flags(&mut self) {
        self.flags = 0b0000_0000_0000_0000_0000_0000_0000_0000
    }

    pub fn reset_dirty(&mut self) {
        self.flags &= 0b0111_1111_1111_1111_1111_1111_1111_1111
    }

    pub fn mark_as_dirty(&mut self) {
        self.flags |= 0b1000_0000_0000_0000_0000_0000_0000_0000
    }

    pub fn set_parent_index(&mut self, index: Option<u32>) {
        match index {
            None => self.flags &= 0b1000_0000_0000_0000_0000_0000_0000_0000,
            Some(index) => self.flags |= (index + 1) & 0b0111_1111_1111_1111_1111_1111_1111_1111,
        }
    }

    pub fn translate(&mut self, d: Vec2) {
        let rad = self.angle.to_radians();
        let cos = rad.cos();
        let sin = rad.sin();

        self.translate_world(Vec2::new(cos * d.x - sin * d.y, sin * d.x + cos * d.y));
    }

    pub fn translate_world(&mut self, d: Vec2) {
        self.position += d;
        self.mark_as_dirty();
    }

    pub fn transform_point(&self, p: Vec2) -> Vec2 {
        let rad = self.angle.to_radians();
        let cos = rad.cos();
        let sin = rad.sin();

        Vec2::new(
            self.scale.x * cos * p.x - self.scale.y * sin * p.y + self.position.x,
            self.scale.x * sin * p.x + self.scale.y * cos * p.y + self.position.y,
        )
    }

    pub fn transform_point_inverse(&self, p: Vec2) -> Vec2 {
        let rad = self.angle.to_radians();
        let cos = rad.cos();
        let sin = rad.sin();

        let scale_inv_x = 1.0 / self.scale.x;
        let scale_inv_y = 1.0 / self.scale.y;

        Vec2::new(
            scale_inv_x * cos * (p.x - self.position.x)
                + scale_inv_y * sin * (p.y - self.position.y),
            scale_inv_x * -sin * (p.x - self.position.x)
                + scale_inv_y * cos * (p.y - self.position.y),
        )
    }

    pub fn transform_direction(&self, d: Vec2) -> Vec2 {
        let rad = self.angle.to_radians();
        let cos = rad.cos();
        let sin = rad.sin();

        Vec2::new(cos * d.x - sin * d.y, sin * d.x + cos * d.y)
    }

    pub fn transform_direction_inverse(&self, d: Vec2) -> Vec2 {
        let rad = self.angle.to_radians();
        let cos = rad.cos();
        let sin = rad.sin();

        Vec2::new(cos * d.x + sin * d.y, -sin * d.x + cos * d.y)
    }

    pub fn to_matrix(&self, matrix: &mut [f32; 9]) {
        // Affine order: scale, rotate -> translate
        let rad = self.angle.to_radians();
        let cos = rad.cos();
        let sin = rad.sin();

        matrix[0] = self.scale.x * cos;
        matrix[1] = self.scale.x * sin;
        matrix[2] = 0.0;
        matrix[3] = -self.scale.y * sin;
        matrix[4] = self.scale.y * cos;
        matrix[5] = 0.0;
        matrix[6] = self.position.x;
        matrix[7] = self.position.y;
        matrix[8] = 1.0;
    }

    pub fn to_matrix_inverse(&self, matrix: &mut [f32; 9]) {
        self.to_matrix(matrix);

        let det_inv = 1f32
            / (matrix[0] * (matrix[4] * matrix[8] - matrix[7] * matrix[5])
                - matrix[1] * (matrix[3] * matrix[8] - matrix[5] * matrix[6])
                + matrix[2] * (matrix[3] * matrix[7] - matrix[4] * matrix[6]));

        let m0 = (matrix[4] * matrix[8] - matrix[7] * matrix[5]) * det_inv;
        let m1 = (matrix[2] * matrix[7] - matrix[1] * matrix[8]) * det_inv;
        let m2 = (matrix[1] * matrix[5] - matrix[2] * matrix[4]) * det_inv;
        let m3 = (matrix[5] * matrix[6] - matrix[3] * matrix[8]) * det_inv;
        let m4 = (matrix[0] * matrix[8] - matrix[2] * matrix[6]) * det_inv;
        let m5 = (matrix[3] * matrix[2] - matrix[0] * matrix[5]) * det_inv;
        let m6 = (matrix[3] * matrix[7] - matrix[6] * matrix[4]) * det_inv;
        let m7 = (matrix[6] * matrix[1] - matrix[0] * matrix[7]) * det_inv;
        let m8 = (matrix[0] * matrix[4] - matrix[3] * matrix[1]) * det_inv;

        matrix[0] = m0;
        matrix[1] = m1;
        matrix[2] = m2;
        matrix[3] = m3;
        matrix[4] = m4;
        matrix[5] = m5;
        matrix[6] = m6;
        matrix[7] = m7;
        matrix[8] = m8;
    }

    pub fn to_matrix_inverse_with_scale(&self, scale_x: f32, scale_y: f32, matrix: &mut [f32; 9]) {
        self.to_matrix(matrix);

        matrix[0] *= scale_x;
        matrix[1] *= scale_x;
        matrix[3] *= scale_y;
        matrix[4] *= scale_y;

        let det_inv = 1f32
            / (matrix[0] * (matrix[4] * matrix[8] - matrix[7] * matrix[5])
                - matrix[1] * (matrix[3] * matrix[8] - matrix[5] * matrix[6])
                + matrix[2] * (matrix[3] * matrix[7] - matrix[4] * matrix[6]));

        let m0 = (matrix[4] * matrix[8] - matrix[7] * matrix[5]) * det_inv;
        let m1 = (matrix[2] * matrix[7] - matrix[1] * matrix[8]) * det_inv;
        let m2 = (matrix[1] * matrix[5] - matrix[2] * matrix[4]) * det_inv;
        let m3 = (matrix[5] * matrix[6] - matrix[3] * matrix[8]) * det_inv;
        let m4 = (matrix[0] * matrix[8] - matrix[2] * matrix[6]) * det_inv;
        let m5 = (matrix[3] * matrix[2] - matrix[0] * matrix[5]) * det_inv;
        let m6 = (matrix[3] * matrix[7] - matrix[6] * matrix[4]) * det_inv;
        let m7 = (matrix[6] * matrix[1] - matrix[0] * matrix[7]) * det_inv;
        let m8 = (matrix[0] * matrix[4] - matrix[3] * matrix[1]) * det_inv;

        matrix[0] = m0;
        matrix[1] = m1;
        matrix[2] = m2;
        matrix[3] = m3;
        matrix[4] = m4;
        matrix[5] = m5;
        matrix[6] = m6;
        matrix[7] = m7;
        matrix[8] = m8;
    }
}

impl Transform {
    pub fn world_position(index: u32, transform_mgr: &TransformManager) -> Vec2 {
        let mut transform_index = Some(index);
        let mut position = Vec2::new(0f32, 0f32);

        while let Some(index) = transform_index {
            let transform = transform_mgr.transform(index);
            transform_index = transform.parent_index();
            position = transform.transform_point(position);
        }

        position
    }

    pub fn set_world_position(
        index: u32,
        transform_mgr: &mut TransformManager,
        mut position: Vec2,
    ) {
        let mut transform_indices = Vec::with_capacity(8);
        let mut transform_index = Some(index);

        while let Some(index) = transform_index {
            let transform = transform_mgr.transform(index);
            transform_indices.push(index);
            transform_index = transform.parent_index();
        }

        let mut iter = transform_indices.iter().rev().peekable();

        while let Some(&index) = iter.next() {
            if iter.peek().is_none() {
                break;
            }

            let transform = transform_mgr.transform(index);
            position = transform.transform_point_inverse(position);
        }

        let transform = transform_mgr.transform_mut(index);
        transform.position = position;
        transform.mark_as_dirty();
    }

    pub fn world_scale(index: u32, transform_mgr: &TransformManager) -> Vec2 {
        let mut transform_index = Some(index);
        let mut scale = Vec2::new(1f32, 1f32);

        while let Some(index) = transform_index {
            let transform = transform_mgr.transform(index);
            transform_index = transform.parent_index();
            scale *= transform.scale;
        }

        scale
    }

    pub fn set_world_scale(index: u32, transform_mgr: &mut TransformManager, mut scale: Vec2) {
        let mut transform_indices = Vec::with_capacity(8);
        let mut transform_index = Some(index);

        while let Some(index) = transform_index {
            let transform = transform_mgr.transform(index);
            transform_indices.push(index);
            transform_index = transform.parent_index();
        }

        let mut iter = transform_indices.iter().rev().peekable();

        while let Some(&index) = iter.next() {
            if iter.peek().is_none() {
                break;
            }

            let transform = transform_mgr.transform(index);
            scale /= transform.scale;
        }

        let transform = transform_mgr.transform_mut(index);
        transform.scale = scale;
        transform.mark_as_dirty();
    }

    pub fn world_angle(index: u32, transform_mgr: &TransformManager) -> f32 {
        let mut transform_index = Some(index);
        let mut angle = 0f32;

        while let Some(index) = transform_index {
            let transform = transform_mgr.transform(index);
            transform_index = transform.parent_index();
            angle += transform.angle;
        }

        angle
    }

    pub fn set_world_angle(index: u32, transform_mgr: &mut TransformManager, mut angle: f32) {
        let mut transform_indices = Vec::with_capacity(8);
        let mut transform_index = Some(index);

        while let Some(index) = transform_index {
            let transform = transform_mgr.transform(index);
            transform_indices.push(index);
            transform_index = transform.parent_index();
        }

        let mut iter = transform_indices.iter().rev().peekable();

        while let Some(&index) = iter.next() {
            if iter.peek().is_none() {
                break;
            }

            let transform = transform_mgr.transform(index);
            angle -= transform.angle;
        }

        let transform = transform_mgr.transform_mut(index);
        transform.angle = angle;
        transform.mark_as_dirty();
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec2::new(0f32, 0f32),
            scale: Vec2::new(1f32, 1f32),
            angle: 0f32,
            flags: 0b1000_0000_0000_0000_0000_0000_0000_0000,
        }
    }
}
