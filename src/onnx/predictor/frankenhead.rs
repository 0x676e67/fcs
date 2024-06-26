use super::{base::ImageClassifierPredictor, Predictor};
use crate::{onnx::ONNXConfig, Result};
use image::DynamicImage;
pub struct FrankenheadPredictor(ImageClassifierPredictor);

impl FrankenheadPredictor {
    /// Create a new instance of the Frankenhead
    pub async fn new(config: &ONNXConfig) -> Result<Self> {
        Ok(Self(
            ImageClassifierPredictor::new("frankenhead.onnx", None, config).await?,
        ))
    }
}

impl Predictor for FrankenheadPredictor {
    fn predict(&self, image: DynamicImage) -> Result<i32> {
        self.0.predict(image)
    }

    fn active(&self) -> bool {
        self.0.active()
    }
}
