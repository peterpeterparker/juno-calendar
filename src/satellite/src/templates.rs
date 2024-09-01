pub const FONT_DATA: &[u8] = include_bytes!("OpenSans-Regular.ttf");

pub const SOCIAL_IMAGE_TEMPLATE: &str = r#"
<svg width="100%" height="100%" viewBox="0 0 900 473" version="1.1"
     xmlns="http://www.w3.org/2000/svg">
    <rect x="0" y="0" width="900" height="473" fill="white"/>
    <g transform="translate(20, 60)">
        {{title}}
    </g>
</svg>
"#;