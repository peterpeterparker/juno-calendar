use rusttype::{point, Font, PositionedGlyph, Scale, OutlineBuilder};
use std::fmt::Write;

pub struct SvgPathBuilder {
    pub path_data: String,
    pub x: f32,
    pub y: f32,
}

impl SvgPathBuilder {
    pub fn new(initial_x: f32, initial_y: f32) -> Self {
        Self {
            path_data: String::new(),
            x: initial_x,
            y: initial_y,
        }
    }

    pub fn append_glyph(&mut self, glyph: &PositionedGlyph<'_>) {
        // Build the outline of the glyph and add it to the path
        glyph.build_outline(self);
        // Advance the x position for the next glyph
        self.x += glyph.unpositioned().h_metrics().advance_width;
    }
}

impl OutlineBuilder for SvgPathBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        write!(&mut self.path_data, "M {} {} ", x + self.x, y + self.y).unwrap();
    }

    fn line_to(&mut self, x: f32, y: f32) {
        write!(&mut self.path_data, "L {} {} ", x + self.x, y + self.y).unwrap();
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        write!(&mut self.path_data, "Q {} {} {} {} ", x1 + self.x, y1 + self.y, x + self.x, y + self.y).unwrap();
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        write!(
            &mut self.path_data,
            "C {} {} {} {} {} {} ",
            x1 + self.x, y1 + self.y, x2 + self.x, y2 + self.y, x + self.x, y + self.y
        ).unwrap();
    }

    fn close(&mut self) {
        self.path_data.push('Z');
    }
}

pub fn text_to_svg_path(text: &str, font_data: &[u8], font_size: f32) -> Result<String, String> {
    let font = Font::try_from_bytes(font_data).ok_or("Error loading font")?;
    let scale = Scale::uniform(font_size);
    let v_metrics = font.v_metrics(scale);

    // Start at baseline (ascent)
    let start_x = 0.0;
    let start_y = v_metrics.ascent;

    let mut builder = SvgPathBuilder::new(start_x, start_y);

    let glyphs: Vec<PositionedGlyph> = font.layout(text, scale, point(start_x, start_y)).collect();

    for glyph in &glyphs {
        builder.append_glyph(glyph);
    }

    Ok(format!(r#"<path fill="black" stroke="none" d="{}"/>"#, builder.path_data))
}
