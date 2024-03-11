// use image::{imageops::FilterType, GenericImageView};
// use ndarray::{s, Array, Axis};
// use ort::{inputs, CUDAExecutionProvider, Session, SessionOutputs};
// use raqote::{DrawOptions, DrawTarget, LineJoin, PathBuilder, SolidSource, Source, StrokeStyle};
// use show_image::{event, AsImageView, WindowOptions};

use ort;

const ONNX_URL: &str = "https://huggingface.co/trpakov/vit-face-expression/resolve/main/onnx/model.onnx";

// #[show_image::main]
fn main()
{
    let session = ort::Session::builder().unwrap();
    let model = session.with_model_downloaded(ONNX_URL);

    match model {
        Ok(v) => println!("working with version: {v:?}"),
        Err(e) => println!("error parsing header: {e:?}"),
    }

    // println!("All done!");
}
