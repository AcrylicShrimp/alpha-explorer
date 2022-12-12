use smartstring::alias::String as SmartString;
use std::collections::HashMap;

#[derive(Debug)]
pub struct TransformNameManager {
    names: Vec<Option<SmartString>>,
    name_map: HashMap<SmartString, Vec<u32>>,
}

impl TransformNameManager {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn name(&self, index: u32) -> Option<&SmartString> {
        self.names[index as usize].as_ref()
    }

    pub fn transforms_by_name(&self, name: impl AsRef<str>) -> &[u32] {
        self.name_map
            .get(name.as_ref())
            .map_or(&[], |transforms| transforms.as_slice())
    }

    pub fn add(&mut self, transform: u32) {
        let transform_usize = transform as usize;

        if transform_usize < self.names.len() {
            self.names[transform_usize] = None;
        } else {
            debug_assert!(transform_usize == self.names.len());
            self.names.push(None);
        }
    }

    pub fn set_name(&mut self, transform: u32, name: Option<SmartString>) {
        self.remove_from_name_map(transform);

        if let Some(name) = name.clone() {
            self.name_map.entry(name).or_default().push(transform);
        }

        self.names[transform as usize] = name;
    }

    pub fn remove_from_name_map(&mut self, transform: u32) {
        if let Some(name) = &self.names[transform as usize] {
            if let Some(transforms) = self.name_map.get_mut(name) {
                if let Some(index) = transforms
                    .iter()
                    .position(|&named_transform| named_transform == transform)
                {
                    transforms.swap_remove(index);
                }
            }
        }
    }
}

impl Default for TransformNameManager {
    fn default() -> Self {
        Self {
            names: Vec::with_capacity(1024),
            name_map: HashMap::with_capacity(1024),
        }
    }
}
