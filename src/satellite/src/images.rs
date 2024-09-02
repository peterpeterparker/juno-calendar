use ic_cdk::print;
use junobuild_satellite::OnSetDocContext;

pub fn generate_social_image(context: &OnSetDocContext) -> Result<(), String> {
    print("________________ This is an event");

    Ok(())
}