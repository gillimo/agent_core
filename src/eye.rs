use anyhow::Result;
use image::DynamicImage;
use xcap::Monitor;

pub struct Eye {
    monitor: Monitor,
}

impl Eye {
    pub fn new() -> Result<Self> {
        let monitors = Monitor::all().map_err(|e| anyhow::anyhow!(e))?;
        let monitor = monitors.into_iter().next().ok_or_else(|| anyhow::anyhow!("No monitor found"))?;
        
        Ok(Self { monitor })
    }

    pub fn capture(&self) -> Result<DynamicImage> {
        // xcap returns an image buffer, we convert it to DynamicImage
        let image = self.monitor.capture_image().map_err(|e| anyhow::anyhow!(e))?;
        
        // Convert RgbaImage to DynamicImage
        Ok(DynamicImage::ImageRgba8(image))
    }
}
