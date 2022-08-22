use raster::Color;
use raster::Image;

fn main() {
    let mut canvas = Image::blank(200, 100); // Mutable 200x100 image with black background

    canvas
        .set_pixel(10, 10, Color::rgba(255, 0, 0, 255))
        .unwrap(); // Set pixel at 10, 10 to red

    raster::save(&canvas, "test_tmp.png"); // Save
}
