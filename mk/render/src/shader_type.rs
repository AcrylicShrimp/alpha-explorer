#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderUniformType {
    B1,
    B2,
    B3,
    B4,
    I1,
    I2,
    I3,
    I4,
    U1,
    U2,
    U3,
    U4,
    F1,
    F2,
    F3,
    F4,
    F22,
    F23,
    F24,
    F32,
    F33,
    F34,
    F42,
    F43,
    F44,
    Sampler,
}

impl ShaderUniformType {
    pub fn component(self) -> u32 {
        match self {
            ShaderUniformType::B1 => 1,
            ShaderUniformType::B2 => 2,
            ShaderUniformType::B3 => 3,
            ShaderUniformType::B4 => 4,
            ShaderUniformType::I1 => 1,
            ShaderUniformType::I2 => 2,
            ShaderUniformType::I3 => 3,
            ShaderUniformType::I4 => 4,
            ShaderUniformType::U1 => 1,
            ShaderUniformType::U2 => 2,
            ShaderUniformType::U3 => 3,
            ShaderUniformType::U4 => 4,
            ShaderUniformType::F1 => 1,
            ShaderUniformType::F2 => 2,
            ShaderUniformType::F3 => 3,
            ShaderUniformType::F4 => 4,
            ShaderUniformType::F22 => 2,
            ShaderUniformType::F23 => 2,
            ShaderUniformType::F24 => 2,
            ShaderUniformType::F32 => 3,
            ShaderUniformType::F33 => 3,
            ShaderUniformType::F34 => 3,
            ShaderUniformType::F42 => 4,
            ShaderUniformType::F43 => 4,
            ShaderUniformType::F44 => 4,
            ShaderUniformType::Sampler => 1,
        }
    }

    pub fn size(self) -> u32 {
        match self {
            ShaderUniformType::B1 => 1,
            ShaderUniformType::B2 => 2,
            ShaderUniformType::B3 => 3,
            ShaderUniformType::B4 => 4,
            ShaderUniformType::I1 => 4,
            ShaderUniformType::I2 => 8,
            ShaderUniformType::I3 => 12,
            ShaderUniformType::I4 => 16,
            ShaderUniformType::U1 => 4,
            ShaderUniformType::U2 => 8,
            ShaderUniformType::U3 => 12,
            ShaderUniformType::U4 => 16,
            ShaderUniformType::F1 => 4,
            ShaderUniformType::F2 => 8,
            ShaderUniformType::F3 => 12,
            ShaderUniformType::F4 => 16,
            ShaderUniformType::F22 => 16,
            ShaderUniformType::F23 => 24,
            ShaderUniformType::F24 => 32,
            ShaderUniformType::F32 => 24,
            ShaderUniformType::F33 => 36,
            ShaderUniformType::F34 => 48,
            ShaderUniformType::F42 => 32,
            ShaderUniformType::F43 => 48,
            ShaderUniformType::F44 => 56,
            ShaderUniformType::Sampler => 4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderAttributeType {
    I1,
    I2,
    I3,
    I4,
    U1,
    U2,
    U3,
    U4,
    F1,
    F2,
    F3,
    F4,
    F22,
    F23,
    F24,
    F32,
    F33,
    F34,
    F42,
    F43,
    F44,
}

impl ShaderAttributeType {
    pub fn component(self) -> u32 {
        match self {
            ShaderAttributeType::I1 => 1,
            ShaderAttributeType::I2 => 2,
            ShaderAttributeType::I3 => 3,
            ShaderAttributeType::I4 => 4,
            ShaderAttributeType::U1 => 1,
            ShaderAttributeType::U2 => 2,
            ShaderAttributeType::U3 => 3,
            ShaderAttributeType::U4 => 4,
            ShaderAttributeType::F1 => 1,
            ShaderAttributeType::F2 => 2,
            ShaderAttributeType::F3 => 3,
            ShaderAttributeType::F4 => 4,
            ShaderAttributeType::F22 => 2,
            ShaderAttributeType::F23 => 2,
            ShaderAttributeType::F24 => 2,
            ShaderAttributeType::F32 => 3,
            ShaderAttributeType::F33 => 3,
            ShaderAttributeType::F34 => 3,
            ShaderAttributeType::F42 => 4,
            ShaderAttributeType::F43 => 4,
            ShaderAttributeType::F44 => 4,
        }
    }

    pub fn component_count(self) -> u32 {
        match self {
            ShaderAttributeType::I1 => 1,
            ShaderAttributeType::I2 => 1,
            ShaderAttributeType::I3 => 1,
            ShaderAttributeType::I4 => 1,
            ShaderAttributeType::U1 => 1,
            ShaderAttributeType::U2 => 1,
            ShaderAttributeType::U3 => 1,
            ShaderAttributeType::U4 => 1,
            ShaderAttributeType::F1 => 1,
            ShaderAttributeType::F2 => 1,
            ShaderAttributeType::F3 => 1,
            ShaderAttributeType::F4 => 1,
            ShaderAttributeType::F22 => 2,
            ShaderAttributeType::F23 => 3,
            ShaderAttributeType::F24 => 4,
            ShaderAttributeType::F32 => 2,
            ShaderAttributeType::F33 => 3,
            ShaderAttributeType::F34 => 4,
            ShaderAttributeType::F42 => 2,
            ShaderAttributeType::F43 => 3,
            ShaderAttributeType::F44 => 4,
        }
    }

    pub fn size(self) -> u32 {
        match self {
            ShaderAttributeType::I1 => 4,
            ShaderAttributeType::I2 => 8,
            ShaderAttributeType::I3 => 12,
            ShaderAttributeType::I4 => 16,
            ShaderAttributeType::U1 => 4,
            ShaderAttributeType::U2 => 8,
            ShaderAttributeType::U3 => 12,
            ShaderAttributeType::U4 => 16,
            ShaderAttributeType::F1 => 4,
            ShaderAttributeType::F2 => 8,
            ShaderAttributeType::F3 => 12,
            ShaderAttributeType::F4 => 16,
            ShaderAttributeType::F22 => 16,
            ShaderAttributeType::F23 => 24,
            ShaderAttributeType::F24 => 32,
            ShaderAttributeType::F32 => 24,
            ShaderAttributeType::F33 => 36,
            ShaderAttributeType::F34 => 48,
            ShaderAttributeType::F42 => 32,
            ShaderAttributeType::F43 => 48,
            ShaderAttributeType::F44 => 56,
        }
    }
}

#[derive(Debug, Clone, Hash)]
pub struct ShaderUniform {
    pub name: String,
    pub location: u32,
    pub ty: ShaderUniformType,
    pub count: u32,
}

#[derive(Debug, Clone, Hash)]
pub struct ShaderUniformBlock {
    pub name: String,
    pub index: u32,
}

#[derive(Debug, Clone, Hash)]
pub struct ShaderAttribute {
    pub name: String,
    pub location: u32,
    pub ty: ShaderAttributeType,
    pub count: u32,
}
