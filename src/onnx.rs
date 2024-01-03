use std::path::Path;

use image::{imageops::FilterType, GenericImageView};
use ndarray::{s, Array, Axis};
use ort::{inputs, CUDAExecutionProvider, Session, SessionOutputs};
use raqote::{DrawOptions, DrawTarget, LineJoin, PathBuilder, SolidSource, Source, StrokeStyle};
use show_image::{event, AsImageView, WindowOptions};

const ONNX_URL: &str = "https://huggingface.co/trpakov/vit-face-expression/resolve/main/onnx/model.onnx";

#[show_image::main]
fn main() -> ort::Result<()>
{
    let model = Session::builder()?.with_model_downloaded(ONNX_URL)?;
}
