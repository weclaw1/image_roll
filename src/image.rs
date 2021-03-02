use std::path::Path;

use gdk_pixbuf::{InterpType, Pixbuf};

pub struct Image {
    image_buffer: Pixbuf,
    preview_image_buffer: Pixbuf,
}

impl Image {
    pub fn load_from_path<P: AsRef<Path>>(path: P) -> Image {
        let image_buffer = Pixbuf::from_file(path).expect("Couldn't load image");
        let preview_image_buffer = image_buffer.clone();
        Image {
            image_buffer,
            preview_image_buffer,
        }
    }

    // pub fn image_buffer(&self) -> &Pixbuf {
    //     &self.image_buffer
    // }

    fn image_buffer_scale_to_fit(&self, canvas_width: i32, canvas_height: i32) -> Pixbuf {
        let image_width = self.image_buffer.get_width();
        let image_height = self.image_buffer.get_height();
        let width_ratio = canvas_width as f32 / image_width as f32;
        let height_ratio = canvas_height as f32 / image_height as f32;
        let scale_ratio = width_ratio.min(height_ratio);
        self.image_buffer
            .scale_simple(
                (image_width as f32 * scale_ratio) as i32,
                (image_height as f32 * scale_ratio) as i32,
                InterpType::Bilinear,
            )
            .unwrap()
    }

    fn image_buffer_resize(&self, scale: f32) -> Pixbuf {
        self.image_buffer
            .scale_simple(
                (self.image_buffer.get_width() as f32 * scale) as i32,
                (self.image_buffer.get_height() as f32 * scale) as i32,
                InterpType::Bilinear,
            )
            .unwrap()
    }

    pub fn create_preview_image_buffer(&mut self, preview_size: PreviewSize) {
        self.preview_image_buffer = match preview_size {
            PreviewSize::BestFit(canvas_width, canvas_height) => {
                self.image_buffer_scale_to_fit(canvas_width, canvas_height)
            }
            PreviewSize::OriginalSize => self.image_buffer.clone(),
            PreviewSize::Resized(scale) => self.image_buffer_resize(scale),
        };
    }

    pub fn preview_image_buffer(&self) -> &Pixbuf {
        &self.preview_image_buffer
    }
}

#[derive(Clone, Copy)]
pub enum PreviewSize {
    BestFit(i32, i32),
    OriginalSize,
    Resized(f32),
}

impl From<&str> for PreviewSize {
    fn from(value: &str) -> Self {
        match value {
            "preview_fit_screen" => PreviewSize::BestFit(0, 0),
            "preview_10" => PreviewSize::Resized(0.1),
            "preview_25" => PreviewSize::Resized(0.25),
            "preview_33" => PreviewSize::Resized(0.33),
            "preview_50" => PreviewSize::Resized(0.5),
            "preview_66" => PreviewSize::Resized(0.66),
            "preview_75" => PreviewSize::Resized(0.75),
            "preview_100" => PreviewSize::OriginalSize,
            "preview_133" => PreviewSize::Resized(1.33),
            "preview_150" => PreviewSize::Resized(1.5),
            "preview_200" => PreviewSize::Resized(2.0),
            _ => panic!("Cannot create PreviewSize from value: {}", value),
        }
    }
}

impl From<PreviewSize> for String {
    fn from(value: PreviewSize) -> Self {
        match value {
            PreviewSize::BestFit(_, _) => String::from("preview_fit_screen"),
            PreviewSize::Resized(value) if value == 0.1 => String::from("preview_10"),
            PreviewSize::Resized(value) if value == 0.25 => String::from("preview_25"),
            PreviewSize::Resized(value) if value == 0.33 => String::from("preview_33"),
            PreviewSize::Resized(value) if value == 0.5 => String::from("preview_50"),
            PreviewSize::Resized(value) if value == 0.66 => String::from("preview_66"),
            PreviewSize::Resized(value) if value == 0.75 => String::from("preview_75"),
            PreviewSize::OriginalSize => String::from("preview_100"),
            PreviewSize::Resized(value) if value == 1.33 => String::from("preview_133"),
            PreviewSize::Resized(value) if value == 1.5 => String::from("preview_150"),
            PreviewSize::Resized(value) if value == 2.0 => String::from("preview_200"),
            _ => panic!("Cannot create PreviewSize for this value"),
        }
    }
}

impl PreviewSize {
    pub fn smaller(self) -> PreviewSize {
        match self {
            PreviewSize::BestFit(_, _) => PreviewSize::OriginalSize,
            PreviewSize::OriginalSize => PreviewSize::Resized(0.75),
            PreviewSize::Resized(value) if value == 2.0 => PreviewSize::Resized(1.5),
            PreviewSize::Resized(value) if value == 1.5 => PreviewSize::Resized(1.33),
            PreviewSize::Resized(value) if value == 1.33 => PreviewSize::OriginalSize,
            PreviewSize::Resized(value) if value == 0.75 => PreviewSize::Resized(0.66),
            PreviewSize::Resized(value) if value == 0.66 => PreviewSize::Resized(0.5),
            PreviewSize::Resized(value) if value == 0.5 => PreviewSize::Resized(0.33),
            PreviewSize::Resized(value) if value == 0.33 => PreviewSize::Resized(0.25),
            PreviewSize::Resized(value) if value == 0.25 => PreviewSize::Resized(0.1),
            PreviewSize::Resized(_) => panic!("Preview size cannot be smaller than 10%"),
        }
    }

    pub fn can_be_smaller(&self) -> bool {
        match self {
            PreviewSize::Resized(value) if value <= &0.1 => false,
            _ => true,
        }
    }

    pub fn larger(self) -> PreviewSize {
        match self {
            PreviewSize::BestFit(_, _) => PreviewSize::OriginalSize,
            PreviewSize::OriginalSize => PreviewSize::Resized(1.33),
            PreviewSize::Resized(value) if value == 0.1 => PreviewSize::Resized(0.25),
            PreviewSize::Resized(value) if value == 0.25 => PreviewSize::Resized(0.33),
            PreviewSize::Resized(value) if value == 0.33 => PreviewSize::Resized(0.5),
            PreviewSize::Resized(value) if value == 0.5 => PreviewSize::Resized(0.66),
            PreviewSize::Resized(value) if value == 0.66 => PreviewSize::Resized(0.75),
            PreviewSize::Resized(value) if value == 0.75 => PreviewSize::OriginalSize,
            PreviewSize::Resized(value) if value == 1.33 => PreviewSize::Resized(1.5),
            PreviewSize::Resized(value) if value == 1.5 => PreviewSize::Resized(2.0),
            PreviewSize::Resized(_) => panic!("Preview size cannot be larger than 200%"),
        }
    }

    pub fn can_be_larger(&self) -> bool {
        match self {
            PreviewSize::Resized(value) if value >= &2.0 => false,
            _ => true,
        }
    }
}
