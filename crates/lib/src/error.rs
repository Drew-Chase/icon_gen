use thiserror::Error;

#[derive(Debug, Error)]
pub enum IconError {
    #[error("Failed to load image")]
    ImageLoadError(#[from] image::ImageError),
}
