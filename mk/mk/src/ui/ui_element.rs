use crate::ui::{UIAnchor, UIMargin};

#[derive(Debug, Clone)]
pub struct UIElement {
    pub anchor: UIAnchor,
    pub margin: UIMargin,
    pub flags: u32,
}

impl UIElement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_dirty(&self) -> bool {
        (self.flags & 0b1000_0000_0000_0000_0000_0000_0000_0000) != 0
    }

    pub fn is_interactible(&self) -> bool {
        (self.flags & 0b0100_0000_0000_0000_0000_0000_0000_0000) != 0
    }

    pub fn order_index(&self) -> u32 {
        self.flags & 0b0011_1111_1111_1111_1111_1111_1111_1111
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

    pub fn set_interactible(&mut self, interactible: bool) {
        if interactible {
            self.flags |= 0b0100_0000_0000_0000_0000_0000_0000_0000
        } else {
            self.flags &= 0b1011_1111_1111_1111_1111_1111_1111_1111
        }
    }

    pub fn set_order_index(&mut self, index: u32) {
        self.flags |= index.min(0b0011_1111_1111_1111_1111_1111_1111_1111)
    }
}

impl Default for UIElement {
    fn default() -> Self {
        Self {
            anchor: UIAnchor::default(),
            margin: UIMargin::default(),
            flags: 0b1100_0000_0000_0000_0000_0000_0000_0000,
        }
    }
}
