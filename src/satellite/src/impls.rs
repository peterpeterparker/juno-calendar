use rusttype::OutlineBuilder;
use std::fmt::Write;

pub struct SvgPathBuilder {
    pub path_data: String,
}

impl OutlineBuilder for SvgPathBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        write!(&mut self.path_data, "M {} {} ", x, y).unwrap();
    }

    fn line_to(&mut self, x: f32, y: f32) {
        write!(&mut self.path_data, "L {} {} ", x, y).unwrap();
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        write!(&mut self.path_data, "Q {} {} {} {} ", x1, y1, x, y).unwrap();
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        write!(
            &mut self.path_data,
            "C {} {} {} {} {} {} ",
            x1, y1, x2, y2, x, y
        )
            .unwrap();
    }

    fn close(&mut self) {
        self.path_data.push('Z');
    }
}