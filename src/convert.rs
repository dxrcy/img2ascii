use image::{imageops::FilterType, DynamicImage, GenericImageView, Rgba};

/// ASCII characters used to render image
const CHAR_PALETTE: &[char] = &[
    ' ', '.', '\'', '`', '^', '"', ',', ':', ';', 'I', 'l', '!', 'i', '>', '<', '~', '+', '_', '-',
    '?', ']', '[', '}', '{', '1', ')', '(', '|', '\\', '/', 't', 'f', 'j', 'r', 'x', 'n', 'u', 'v',
    'c', 'z', 'X', 'Y', 'U', 'J', 'C', 'L', 'Q', '0', 'O', 'Z', 'm', 'w', 'q', 'p', 'd', 'b', 'k',
    'h', 'a', 'o', '*', '#', 'M', 'W', '&', '8', '%', 'B', '@', '$',
];
/// Minimum alpha value, for a pixel to be rendered in `color` mode (0-255)
const MIN_ALPHA: u8 = 200;

/// Convert an image into an ASCII string
pub fn image_to_ascii(image: DynamicImage, color: bool) -> String {
    // Fit image to terminal size
    let (max_width, max_height) = term_size::dimensions().expect("Failed to get terminal size");
    let image = resize_image_to_fit(&image, max_width as u32, max_height as u32);

    // Loop every pixel in image
    let (image_width, image_height) = image.dimensions();
    let mut output = String::new();
    for y in 0..image_height {
        if y > 0 {
            if color {
                // 'Reset'
                output += "\x1b[0m";
            }
            output.push('\n');
        }
        for x in 0..image_width {
            // Get RGB of pixel
            let pixel = image.get_pixel(x, y);
            let Rgba([r, g, b, a]) = pixel;

            if color {
                // Color code to apply to space character
                if a < MIN_ALPHA {
                    // 'Reset'
                    output += "\x1b[0m"
                } else {
                    // 'Background RGB'
                    output += &format!("\x1b[48;2;{};{};{}m", r, g, b)
                }
                output.push(' ');
            } else {
                // Get color value, as average of R,G,B (0-255)
                let value = (r as usize + g as usize + b as usize) / 3;
                // Multiply value by alpha (0-255)
                let value = value * a as usize / 256;
                // Scale value to length of characters (0-len)
                let index = value * CHAR_PALETTE.len() / 256;
                // Get ASCII character
                let char = *CHAR_PALETTE.get(index).unwrap_or(&' ');
                output.push(char);
            }
        }
    }

    output
}

/// Resize an image to fit maximum width and height bounds
///
/// This was copied from `image.resize`
fn resize_image_to_fit(image: &DynamicImage, max_width: u32, max_height: u32) -> DynamicImage {
    let (width, height) = image.dimensions();

    // Scale width by 2, to account for 1:2 character ratio in terminal
    // If this wasn't here, `image.resize` could have been used instead
    // This could have also been done in 2 passes (`resize_exact` and `resize`), but this would be
    // slightly slower
    let width = 2 * width;

    // Minimum of ratios between original and max (width or height)
    let ratio = f32::min(
        max_width as f32 / width as f32,
        max_height as f32 / height as f32,
    );

    // Scale based on minimum ratio
    let new_width = (width as f32 * ratio).round().max(1.0) as u32;
    let new_height = (height as f32 * ratio).round().max(1.0) as u32;

    // Resize image, assuming aspect ratio is already preserved
    image.resize_exact(new_width as u32, new_height as u32, FilterType::Nearest)
}
