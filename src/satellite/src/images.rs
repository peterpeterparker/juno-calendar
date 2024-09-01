use crate::templates::SOCIAL_IMAGE_TEMPLATE;
use crate::types::EventsData;
use ic_cdk::{id, print};
use junobuild_satellite::{set_asset_handler, OnSetDocContext};
use junobuild_storage::http::types::HeaderField;
use junobuild_storage::types::store::AssetKey;
use junobuild_utils::decode_doc_data;
use png::Encoder;
use resvg::usvg::fontdb::Database;
use resvg::usvg::{Options, Transform, Tree};
use std::sync::Arc;
use tiny_skia::Pixmap;

pub fn generate_social_image(context: &OnSetDocContext) -> Result<(), String> {
    let svg_data = prepare_svg(context)?;

    let png_bytes = convert_svg_to_png(&svg_data, 1200, 630).expect("Failed to convert SVG to PNG");

    insert_asset(&context.data.key, &png_bytes)?;

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

pub fn insert_asset(name: &String, data: &Vec<u8>) -> Result<(), String> {
    let collection = "images".to_string();

    let full_path = format!("/{}/{}.png", collection, name.clone()).to_string();

    let key: AssetKey = AssetKey {
        name: name.clone(),
        full_path: full_path.clone(),
        token: None,
        collection,
        owner: id(),
        description: None,
    };

    let headers = vec![HeaderField(
        "content-type".to_string(),
        "image/png".to_string(),
    )];

    set_asset_handler(&key, data, &headers)?;

    print(format!(
        "Image generated to: http://{}.localhost:5987{}",
        id(),
        full_path
    ));

    Ok(())
}

pub fn prepare_svg(context: &OnSetDocContext) -> Result<String, String> {
    let event: EventsData = decode_doc_data(&context.data.data.after.data)?;

    fn truncate_text(text: &str) -> String {
        let max_chars = 13;

        if text.chars().count() <= max_chars {
            text.to_string()
        } else {
            let truncated: String = text.chars().take(max_chars).collect();
            format!("{}...", truncated)
        }
    }

    let svg_data = SOCIAL_IMAGE_TEMPLATE.replace("{{title}}", &truncate_text(&event.title));

    Ok(svg_data)
}
