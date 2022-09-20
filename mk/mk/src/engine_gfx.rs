use anyhow::Result;
use ash::{extensions::khr::Surface, vk, Entry, Instance};
use winit::window::Window;

pub struct EngineGfx {
    entry: Entry,
    instance: Instance,
    surface: vk::SurfaceKHR,
    surface_fn: Surface,
}

impl EngineGfx {
    pub unsafe fn new(window: &Window, app_info: &vk::ApplicationInfo) -> Result<Self> {
        let entry = ash::Entry::linked();
        let surface_extensions = ash_window::enumerate_required_extensions(&window)?;
        let instance_info_builder = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(surface_extensions);

        #[cfg(debug_assertions)]
        instance_info_builder
            .enabled_layer_names(&[b"VK_LAYER_KHRONOS_validation\0".as_ptr() as _]);

        let instance_info = instance_info_builder.build();

        let instance = entry.create_instance(&instance_info, None)?;

        let surface = ash_window::create_surface(&entry, &instance, &window, None)?;
        let surface_fn = ash::extensions::khr::Surface::new(&entry, &instance);

        Ok(Self {
            entry,
            instance,
            surface,
            surface_fn,
        })
    }

    pub fn entry(&self) -> &Entry {
        &self.entry
    }

    pub fn instance(&self) -> &Instance {
        &self.instance
    }

    pub fn surface(&self) -> &vk::SurfaceKHR {
        &self.surface
    }

    pub fn surface_fn(&self) -> &Surface {
        &self.surface_fn
    }
}
