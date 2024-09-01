use ic_cdk::{id, print};
use junobuild_satellite::{OnSetDocContext, set_asset_handler};
use png::Encoder;
use resvg::usvg::{Options, Transform, Tree};
use tiny_skia::Pixmap;
use junobuild_storage::types::store::AssetKey;
use junobuild_storage::http::types::HeaderField;

pub fn generate_social_image(context: &OnSetDocContext) -> Result<(), String> {
    let svg_data = r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
                      <circle cx="50" cy="50" r="40" stroke="green" stroke-width="4" fill="yellow" />
                      </svg>"#;

    print(format!("Svg ----> {}", svg_data));

    let png_bytes = convert_svg_to_png(svg_data, 100, 100).expect("Failed to convert SVG to PNG");

    print(format!("Length ----> {}", png_bytes.len()));

    insert_asset(&context.data.key, &png_bytes)?;

    Ok(())
}

fn convert_svg_to_png(svg_data: &str, width: u32, height: u32) -> Result<Vec<u8>, String> {
    // Parse SVG
    let opt = Options::default();
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
    print(format!("Image: {} {}", name, data.len()));

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

    print(format!("Image generated to: http://{}.localhost:5987{}", id(), full_path));

    Ok(())
}