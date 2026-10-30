use std::sync::Arc;
use vello_svg::usvg::{self};

use super::asset::VelloSvg;
use crate::integrations::VectorLoaderError;

struct SvgScene(vello::Scene);

impl vello_svg::RenderSink for SvgScene {
    fn push_layer(
        &mut self,
        blend: impl Into<vello::peniko::BlendMode>,
        alpha: f32,
        transform: vello::kurbo::Affine,
        shape: &impl vello::kurbo::Shape,
    ) {
        self.0
            .push_layer(vello::peniko::Fill::NonZero, blend, alpha, transform, shape);
    }

    fn pop_layer(&mut self) {
        self.0.pop_layer();
    }

    fn fill(
        &mut self,
        fill: vello::peniko::Fill,
        transform: vello::kurbo::Affine,
        brush: &vello::peniko::Brush,
        brush_transform: vello::kurbo::Affine,
        shape: &impl vello::kurbo::Shape,
    ) {
        self.0
            .fill(fill, transform, brush, Some(brush_transform), shape);
    }

    fn stroke(
        &mut self,
        stroke: &vello::kurbo::Stroke,
        transform: vello::kurbo::Affine,
        brush: &vello::peniko::Brush,
        brush_transform: vello::kurbo::Affine,
        shape: &impl vello::kurbo::Shape,
    ) {
        self.0
            .stroke(stroke, transform, brush, Some(brush_transform), shape);
    }
}

/// Deserialize an SVG file from bytes.
pub fn load_svg_from_bytes(bytes: &[u8]) -> Result<VelloSvg, VectorLoaderError> {
    let svg_str = std::str::from_utf8(bytes)?;

    // Parse SVG
    let tree =
        usvg::Tree::from_str(svg_str, &usvg::Options::default()).map_err(vello_svg::Error::Svg)?;

    // Process the loaded SVG into Vello-compatible data
    let mut scene = SvgScene(vello::Scene::new());
    vello_svg::append_tree(&mut scene, &tree);

    let width = tree.size().width();
    let height = tree.size().height();

    let asset = VelloSvg {
        scene: Arc::new(scene.0),
        width,
        height,
        alpha: 1.0,
    };

    Ok(asset)
}

/// Deserialize an SVG file from a string slice.
pub fn load_svg_from_str(svg_str: &str) -> Result<VelloSvg, VectorLoaderError> {
    let bytes = svg_str.as_bytes();

    load_svg_from_bytes(bytes)
}
