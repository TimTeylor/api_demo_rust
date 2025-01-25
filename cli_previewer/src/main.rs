use clap::Parser;
use image::imageops::FilterType;
use previewer::Size;

#[derive(Parser)]
struct Args {
    /// Path to the image
    #[arg(short, long)]
    image_path: String,

    /// Height of the image
    #[arg(long("he"))]
    height: u32,

    /// Width of the image
    #[arg(short, long)]
    width: u32,

    /// Filter of the image
    #[arg(short, long)]
    filter: String,
}

fn main() {
    let args = Args::parse();

    let image = match image::open(&args.image_path) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("Failed to open image: {}", e);
            return;
        },
    };

    let filter = previewer::str_to_filter(&args.filter).unwrap_or_else(|| FilterType::Gaussian);

    let size = Size {
        width: args.width,
        height: args.height,
    };

    let result = previewer::preview(image, size, filter).expect("preview creation should pass");

    result
        .save(format!("preview_{}", args.image_path))
        .expect("image save should pass");
}
