use crate::Render;
use anyhow::Result;
use raw_window_handle::HasRawWindowHandle;

pub(crate) struct VulkanRenderBackend;

impl Render for VulkanRenderBackend {
    fn render(
        &mut self,
        _dimensions: &[u32; 2],
    ) -> Result<()> {
        Ok(())
    }
}

impl VulkanRenderBackend {
    pub fn new(
        _window_handle: &impl HasRawWindowHandle,
        _dimensions: &[u32; 2],
    ) -> Result<Self> {
        log::info!("Created Vulkan render backend");
        Ok(Self{})
    }
}