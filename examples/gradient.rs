use chromatic::{Colour, ColourMap, Rgb};
use image::{ImageBuffer, Rgb as ImageRgb};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define colours for our gradient
    let colours = vec![
        Rgb::new(1.0, 0.0, 0.0), // Red
        Rgb::new(1.0, 1.0, 0.0), // Yellow
        Rgb::new(0.0, 1.0, 0.0), // Green
        Rgb::new(0.0, 1.0, 1.0), // Cyan
        Rgb::new(0.0, 0.0, 1.0), // Blue
        Rgb::new(1.0, 0.0, 1.0), // Magenta
    ];

    // Define positions for each colour (evenly spaced)
    let positions: Vec<f32> = (0..colours.len()).map(|i| i as f32 / (colours.len() - 1) as f32).collect();

    // Create the colour map
    let colour_map = ColourMap::new(&colours, &positions);

    // Create a gradient PNG
    create_gradient_png(&colour_map, "gradient.png", 800, 100)?;

    println!("Generated gradient.png");

    // Create a circular gradient PNG
    create_circular_gradient(&colour_map, "circular_gradient.png", 400)?;

    println!("Generated circular_gradient.png");

    Ok(())
}

// Function to create a horizontal gradient PNG
fn create_gradient_png<P: AsRef<Path>>(
    colour_map: &ColourMap<Rgb<f32>, f32, 3>,
    path: P,
    width: u32,
    height: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a new RGB image
    let mut image = ImageBuffer::new(width, height);

    // Fill the image with colours sampled from the colour map
    for x in 0..width {
        // Sample position in range [0, 1]
        let position = x as f32 / (width - 1) as f32;

        // Sample the colour map at this position
        let colour = colour_map.sample(position);

        // Convert to image RGB format (0-255 range)
        let bytes = colour.to_bytes();
        let pixel = ImageRgb([bytes[0], bytes[1], bytes[2]]);

        // Draw a vertical line with this colour
        for y in 0..height {
            image.put_pixel(x, y, pixel);
        }
    }

    // Save the image
    image.save(path)?;

    Ok(())
}

// Function to create a circular gradient PNG
fn create_circular_gradient<P: AsRef<Path>>(
    colour_map: &ColourMap<Rgb<f32>, f32, 3>,
    path: P,
    size: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a new RGB image
    let mut image = ImageBuffer::new(size, size);

    let center_x = size as f32 / 2.0;
    let center_y = size as f32 / 2.0;
    let max_radius = size as f32 / 2.0;

    // Fill the image with colours sampled from the colour map
    for y in 0..size {
        for x in 0..size {
            // Calculate distance from center (normalized to [0, 1])
            let dx = x as f32 - center_x;
            let dy = y as f32 - center_y;
            let distance = (dx * dx + dy * dy).sqrt() / max_radius;

            // Angle in radians
            let angle = dy.atan2(dx);

            // Map angle to [0, 1] range
            let angle_position = (angle + std::f32::consts::PI) / (2.0 * std::f32::consts::PI);

            // Sample the colour map based on the angle
            let position = if distance <= 1.0 {
                angle_position
            } else {
                // Use white for pixels outside the circle
                image.put_pixel(x, y, ImageRgb([255, 255, 255]));
                continue;
            };

            // Sample the colour map at this position
            let colour = colour_map.sample(position);

            // Convert to image RGB format (0-255 range)
            let bytes = colour.to_bytes();
            let pixel = ImageRgb([bytes[0], bytes[1], bytes[2]]);

            // Set the pixel
            image.put_pixel(x, y, pixel);
        }
    }

    // Save the image
    image.save(path)?;

    Ok(())
}
