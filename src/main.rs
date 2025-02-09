#![allow(unused_variables)]
use std::{
    path::Path,
    time::{Duration, Instant},
};

use nannou::{
    image::{DynamicImage, GenericImageView, RgbImage}, prelude::*
};
use video_rs::Decoder;

use jpeg_encoder::*;
struct Model {
    decoder: Decoder,
    video_frame: DynamicImage,
    frame_index: u32,
    frame_duration: Duration,
    since_last_frame: Duration,
}

const JPEG_QUALITY: u8 = 30;

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window().view(view).size(960, 1036).build().unwrap();

    video_rs::init().unwrap();
    let decoder = Decoder::new(Path::new("./assets/video.mp4")).unwrap();
    let video_frame = DynamicImage::new_rgb8(0, 0);

    let frame_duration = Duration::from_secs_f32(
        decoder.duration().unwrap().as_secs() / decoder.frames().unwrap() as f32,
    );

    let since_last_frame = Duration::ZERO;

    Model {
        decoder,
        video_frame,
        frame_index: 0,
        frame_duration,
        since_last_frame,
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let now = Instant::now();
    model.since_last_frame += update.since_last;

    if model.since_last_frame < model.frame_duration {
        // Break video frame rate
    }

    let video_frame: DynamicImage = match model.decoder.decode_raw() {
        Ok(video_frame) => {
            let image_buffer = RgbImage::from_vec(
                video_frame.width(),
                video_frame.height(),
                video_frame.data(0).to_vec(),
            )
            .unwrap();

            DynamicImage::ImageRgb8(image_buffer)
        }
        Err(_) => {
            return app.quit();
        }
    };

    model.video_frame = video_frame;
    model.frame_index += 1;
    model.since_last_frame = Duration::ZERO;
    println!("Update time: {:?}", now.elapsed());
}

fn view(app: &App, model: &Model, frame: Frame) {
    /*
    model
        .video_frame
        .save_with_format("./output.bmp", nannou::image::ImageFormat::Bmp)
        .unwrap();
    */

    
    let encode_time = Instant::now();
    encode_frame(&model.video_frame);
    println!("Encode time: {:?}", encode_time.elapsed());

    /*
    model
        .video_frame
        .save_with_format("./output.jpg", nannou::image::ImageFormat::Jpeg)
        .unwrap();
    */

    
    let render_time = Instant::now();
    let draw = app.draw();
    let texture = wgpu::Texture::from_image(app, &model.video_frame);
    draw.texture(&texture);

    let color= Rgba::new(0.0, 0.0, 0.0, 0.0);

    draw_bounding_box(&draw, (0.0, 0.0));

    draw.to_frame(app, &frame).unwrap();
    println!("Render time: {:?}", render_time.elapsed());
}

fn draw_bounding_box(draw: &Draw, pos: (f32, f32)) {
    draw.rect()
        .x(pos.0)
        .y(pos.1)
        .color(rgba(0.0, 0.0, 0.0, 0.0))
        .stroke_weight(2.0)
        .stroke(BLACK);   
}


fn encode_frame(frame: &DynamicImage) {
    let mut buffer: Vec<u8> = Vec::new();
    let encoder = jpeg_encoder::Encoder::new(&mut buffer, JPEG_QUALITY);
    encoder.encode(
        frame.as_bytes(),
        frame.width().try_into().unwrap(),
        frame.height().try_into().unwrap(),
        ColorType::Rgb,
    ).unwrap();
}