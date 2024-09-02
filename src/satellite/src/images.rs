use std::sync::Arc;
use ic_cdk::print;
use junobuild_satellite::OnSetDocContext;
use junobuild_utils::decode_doc_data;
use png::Encoder;
use resvg::usvg::fontdb::Database;
use resvg::usvg::{Options, Tree};
use tiny_skia::{Pixmap, Transform};
use crate::templates::SOCIAL_IMAGE_TEMPLATE;
use crate::types::EventsData;

pub fn generate_social_image(context: &OnSetDocContext) -> Result<(), String> {
    // Returns if this is not a new event
    if context.data.data.before.is_some() {
        return Ok(())
    }

    let svg_data = prepare_svg(context)?;

    let _png_bytes = convert_svg_to_png(&svg_data, 1200, 630).expect("Failed to convert SVG to PNG");

    // 4. We upload that PNG to the Storage

    Ok(())
}

fn convert_svg_to_png(svg_data: &str, width: u32, height: u32) -> Result<Vec<u8>, String> {
    // Initialize the font database and add the embedded font
    let mut fontdb = Database::new();
    fontdb.load_font_data(include_bytes!("OpenSans-Regular.ttf").to_vec());

    // Set up options with the custom font database
    let opt = Options {
        fontdb: Arc::new(fontdb),
        font_family: "Open Sans".to_owned(),
        font_size: 48.0,
        ..Options::default()
    };

    // Parse SVG
    let rtree = Tree::from_str(svg_data, &opt).map_err(|e| e.to_string())?;

    // Get the size of the SVG from the tree's viewBox
    let svg_size = rtree.size();

    // Create a Pixmap for rendering
    let mut pixmap = Pixmap::new(width, height).ok_or("Failed to create Pixmap")?;

    // Set up transformation to scale the SVG to the desired size
    let scale_x = width as f32 / svg_size.width();
    let scale_y = height as f32 / svg_size.height();
    let transform = Transform::from_scale(scale_x, scale_y);

    // Render SVG to Pixmap with the specified transform
    resvg::render(&rtree, transform, &mut pixmap.as_mut());

    // Encode the Pixmap to PNG
    let mut png_data = Vec::new();
    {
        let mut encoder = Encoder::new(&mut png_data, width, height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header().map_err(|e| e.to_string())?;
        writer
            .write_image_data(pixmap.data())
            .map_err(|e| e.to_string())?;
        // writer is dropped here, releasing the borrow on png_data
    }

    // Now it's safe to move png_data
    Ok(png_data)
}

fn prepare_svg(context: &OnSetDocContext) -> Result<String, String> {
    let event: EventsData = decode_doc_data(&context.data.data.after.data)?;

    let svg_data = SOCIAL_IMAGE_TEMPLATE.replace("{{title}}", &event.title);

    Ok(svg_data)
}