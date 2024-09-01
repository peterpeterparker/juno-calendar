use ic_cdk::{id, print};
use junobuild_satellite::{OnSetDocContext, set_asset_handler};
use png::Encoder;
use resvg::usvg::{Options, Transform, Tree};
use tiny_skia::Pixmap;
use junobuild_storage::types::store::AssetKey;
use junobuild_storage::http::types::HeaderField;
use junobuild_utils::decode_doc_data;
use crate::templates::{FONT_DATA, SOCIAL_IMAGE_TEMPLATE};
use crate::types::EventsData;
use rusttype::{point, Font, Scale};
use crate::impls::SvgPathBuilder;

pub fn generate_social_image(context: &OnSetDocContext) -> Result<(), String> {
    let svg_data = prepare_svg(context)?;

    print(format!("Svg ----> {}", svg_data));

    let png_bytes = convert_svg_to_png(&svg_data, 1200, 630).expect("Failed to convert SVG to PNG");

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

fn text_to_svg_path(text: &str, font_data: &[u8], font_size: f32) -> Result<String, String> {
    let font = Font::try_from_bytes(font_data).ok_or("Error loading font")?;
    let scale = Scale::uniform(font_size);
    let v_metrics = font.v_metrics(scale);

    let mut path_data = String::new();

    for glyph in font.layout(text, scale, point(0.0, v_metrics.ascent)) {
        let mut builder = SvgPathBuilder {
            path_data: String::new(),
        };

        glyph.build_outline(&mut builder);

        path_data.push_str(&builder.path_data);
    }

    Ok(format!(r#"<path fill="black" stroke="none" d="{}"/>"#, path_data))
}

pub fn prepare_svg(context: &OnSetDocContext) -> Result<String, String> {
    let event: EventsData = decode_doc_data(&context.data.data.after.data)?;

    let svg_path_data = text_to_svg_path(&event.title, FONT_DATA, 48.0)?;

    let svg_data = SOCIAL_IMAGE_TEMPLATE
        .replace("{{title}}", &svg_path_data);

    Ok(svg_data)
}