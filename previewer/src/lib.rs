use image::{imageops::FilterType, DynamicImage};

pub fn preview(image: DynamicImage, preview_size: Size, filter: FilterType) -> Result<DynamicImage, Error> {
    if preview_size.width == 0 {
        return Err(Error::InvalidWidth);
    }

    if preview_size.height == 0 {
        return Err(Error::InvalidHeight);
    }

    let image = image.resize(preview_size.width, preview_size.height, filter);

    Ok(image)
}

pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub enum Error {
    InvalidHeight,
    InvalidWidth,
}

pub fn filter_to_string(filter: &FilterType) -> String {
    match filter {
        FilterType::Nearest => "nearest".into(),
        FilterType::Triangle => "triangle".into(),
        FilterType::CatmullRom => "cubic".into(),
        FilterType::Gaussian => "gauss".into(),
        FilterType::Lanczos3 => "lanczos3".into(),
    }
}

pub fn str_to_filter(filter: &str) -> Option<FilterType> {
    match filter {
        "nearest" => Some(FilterType::Nearest),
        "triangle" => Some(FilterType::Triangle),
        "cubic" => Some(FilterType::CatmullRom),
        "gauss" => Some(FilterType::Gaussian),
        "lanczos3" => Some(FilterType::Lanczos3),
        _ => None,
    }
}