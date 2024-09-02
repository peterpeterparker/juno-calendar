use ic_cdk::print;
use junobuild_satellite::OnSetDocContext;
use junobuild_utils::decode_doc_data;
use crate::templates::SOCIAL_IMAGE_TEMPLATE;
use crate::types::EventsData;

pub fn generate_social_image(context: &OnSetDocContext) -> Result<(), String> {
    // Returns if this is not a new event
    if context.data.data.before.is_some() {
        return Ok(())
    }

    let svg_data = prepare_svg(context)?;

    print(svg_data);

    // 3. We convert the SVG to PNG
    // 4. We upload that PNG to the Storage

    Ok(())
}

pub fn prepare_svg(context: &OnSetDocContext) -> Result<String, String> {
    let event: EventsData = decode_doc_data(&context.data.data.after.data)?;

    let svg_data = SOCIAL_IMAGE_TEMPLATE.replace("{{title}}", &event.title);

    Ok(svg_data)
}