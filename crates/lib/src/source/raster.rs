use crate::error::IconError;
use image::RgbaImage;
use std::path::Path;

pub struct Raster(RgbaImage);

impl Raster {
    pub fn load_from_path(path: impl AsRef<Path>) -> Result<Self, IconError> {
        let image = image::open(path)?;
        Ok(Self(image.to_rgba8()))
    }
    pub fn inner(&self) -> &RgbaImage {
        &self.0
    }
}

impl From<Raster> for RgbaImage {
    fn from(val: Raster) -> Self {
        val.0
    }
}

#[cfg(test)]
mod test {
    use crate::source::raster::Raster;
    use image::RgbaImage;

    #[test]
    fn load_from_path() {
        let cwd = std::env::current_dir().unwrap();
        println!("cwd: {:?}", cwd);
        let image: RgbaImage = Raster::load_from_path(cwd.join(r#"examples\github-icon.png"#))
            .unwrap()
            .into();
        assert!(!image.is_empty())
    }
}
