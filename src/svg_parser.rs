use std::path::Path;

use crate::{ChosenGlyph, Id, SvgIcon};

use usvg::{
    Node, Options, Tree,
    tiny_skia_path::{self, PathBuilder, PathSegment, Transform},
};

pub fn parse_image(icon_path: &Path, name: String, code: u64, uid: String) -> Option<ChosenGlyph> {
    let mut svg_path = String::new();

    let svg_str = std::fs::read_to_string(icon_path).unwrap_or_else(|error| {
        panic!(
            "SVG icon file \"{}\" could not be read: {error}",
            icon_path.display()
        )
    });

    let usvg_tree = Tree::from_str(&svg_str, &Options::default()).unwrap_or_else(|error| {
        panic!(
            "SVG icon file \"{}\" could not be parsed: {error}",
            icon_path.display()
        )
    });

    let mut svg_width = 0.0;

    // Create new `PathBuilder`
    let mut path_builder = PathBuilder::new();

    // Process the SVG tree using the path_builder
    usvg_tree
        .root()
        .children()
        .iter()
        .for_each(|n| process_node(n, &mut path_builder));

    // Scale the icon's frame (its viewBox, already mapped by usvg so the
    // frame origin is at 0,0) onto the 1000-unit glyph grid. Normalizing the
    // frame instead of the geometry bounds preserves the padding icon sets
    // design into their viewBox, so glyphs keep consistent relative sizes.
    if let Some(final_path) = path_builder.finish() {
        let size = usvg_tree.size();
        let scale = 1000.0 / size.height();

        let final_path = final_path
            .transform(Transform::from_scale(scale, scale))
            .unwrap_or_else(|| {
                panic!(
                    "SVG icon file \"{}\" failed to apply the glyph grid scale",
                    icon_path.display()
                )
            });

        svg_width = size.width() * scale;

        // Write path to string
        svg_path = write_path(final_path);
    }

    if !svg_path.is_empty() && svg_width != 0.0 {
        Some(ChosenGlyph {
            uid: Id(uid),
            css: name,
            code,
            src: "custom_icons".into(),
            selected: Some(true),
            svg: Some(SvgIcon {
                path: svg_path,
                width: svg_width,
            }),
        })
    } else {
        None
    }
}

/// Process the nodes of an SVG tree by pushing each new path to the `path_builder`. The paths are
/// transformed before being pushed using their absolute transform which includes the parents
/// transforms.
fn process_node(node: &Node, path_builder: &mut PathBuilder) {
    match node {
        Node::Group(group) => {
            group
                .children()
                .iter()
                .for_each(|n| process_node(n, path_builder));
        }
        Node::Path(path) => {
            let t = path.abs_transform();
            let transform = |p: tiny_skia_path::Path| p.transform(t);
            // A path with no paint at all keeps the legacy treat-as-fill
            // behavior, because icon sources are often exported with
            // fill="none" while relying on this pipeline to fill the bare
            // geometry.
            if path.fill().is_some() || path.stroke().is_none() {
                match transform(path.data().clone()) {
                    Some(geometry) => path_builder.push_path(&geometry),
                    None => path_builder.push_path(path.data()),
                }
            }
            // Stroke in local coordinates and transform the outline, matching
            // SVG semantics (the transform maps the painted band, so a
            // non-uniform scale correctly yields a non-uniform thickness).
            if let Some(stroke) = path.stroke() {
                match expand_stroke(path.data(), stroke, t).and_then(transform) {
                    Some(outline) => path_builder.push_path(&outline),
                    None => eprintln!("warning: failed to expand a stroke into a fill outline"),
                }
            }
        }
        Node::Image(_image) => {}
        Node::Text(_text) => {}
    }
}

/// Expands a stroked path into its fill outline, so stroke-based icons render
/// as font glyphs instead of contributing their zero-area centerlines.
fn expand_stroke(
    geometry: &tiny_skia_path::Path,
    stroke: &usvg::Stroke,
    transform: Transform,
) -> Option<tiny_skia_path::Path> {
    let sk_stroke = tiny_skia_path::Stroke {
        width: stroke.width().get(),
        miter_limit: stroke.miterlimit().get(),
        line_cap: match stroke.linecap() {
            usvg::LineCap::Butt => tiny_skia_path::LineCap::Butt,
            usvg::LineCap::Round => tiny_skia_path::LineCap::Round,
            usvg::LineCap::Square => tiny_skia_path::LineCap::Square,
        },
        line_join: match stroke.linejoin() {
            usvg::LineJoin::Miter | usvg::LineJoin::MiterClip => tiny_skia_path::LineJoin::Miter,
            usvg::LineJoin::Round => tiny_skia_path::LineJoin::Round,
            usvg::LineJoin::Bevel => tiny_skia_path::LineJoin::Bevel,
        },
        dash: stroke
            .dasharray()
            .and_then(|d| tiny_skia_path::StrokeDash::new(d.to_vec(), stroke.dashoffset())),
    };
    // The resolution scale sets the curve flattening precision. The outline is
    // magnified afterwards (group transform plus the 1000-unit glyph grid), so
    // flatten finer than the local coordinates alone would call for.
    let resolution_scale =
        10.0 * tiny_skia_path::PathStroker::compute_resolution_scale(&transform).max(1.0);
    tiny_skia_path::PathStroker::new().stroke(geometry, &sk_stroke, resolution_scale)
}

/// Prints the `path` into a string of segments that can be used by fontello's glyph path
fn write_path(path: tiny_skia_path::Path) -> String {
    let mut s = String::new();
    for segment in path.segments() {
        match segment {
            PathSegment::MoveTo(p) => s.push_str(&format!("M {} {} ", p.x, p.y)),
            PathSegment::LineTo(p) => s.push_str(&format!("L {} {} ", p.x, p.y)),
            PathSegment::QuadTo(p0, p1) => {
                s.push_str(&format!("Q {} {} {} {} ", p0.x, p0.y, p1.x, p1.y))
            }
            PathSegment::CubicTo(p0, p1, p2) => s.push_str(&format!(
                "C {} {} {} {} {} {} ",
                p0.x, p0.y, p1.x, p1.y, p2.x, p2.y
            )),
            PathSegment::Close => s.push_str("Z "),
        }
    }
    s.pop(); // remove last trailing space
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_str(svg: &str, name: &str) -> Option<ChosenGlyph> {
        let path = std::env::temp_dir().join(format!("iced_fontello_test_{name}.svg"));
        std::fs::write(&path, svg).unwrap();
        let glyph = parse_image(&path, name.into(), 0xe800, "test".into());
        std::fs::remove_file(&path).ok();
        glyph
    }

    #[test]
    fn stroke_only_line_becomes_a_filled_outline() {
        // The viewBox is the band's exact bounds, so the frame normalization
        // gives the same numbers the geometry would: 80x8 -> 10000x1000.
        let glyph = parse_str(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="10 46 80 8">
                <path d="M10 50 L90 50" stroke="black" stroke-width="8" fill="none"/>
            </svg>"#,
            "stroked_line",
        )
        .expect("stroked line should produce a glyph");
        let svg = glyph.svg.expect("glyph should carry svg path data");
        assert!((svg.width - 10000.0).abs() < 1.0, "width was {}", svg.width);
    }

    #[test]
    fn viewbox_padding_is_preserved() {
        // An 18-unit drawing centered in a 24-unit frame must keep its
        // padding: the glyph is frame-sized (square, so width 1000), not
        // inflated to the geometry bounds (which would make it 9000 wide).
        let glyph = parse_str(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                <path d="M3 12 L21 12" stroke="black" stroke-width="2" fill="none"/>
            </svg>"#,
            "padded_line",
        )
        .expect("padded stroked line should produce a glyph");
        let svg = glyph.svg.unwrap();
        assert!((svg.width - 1000.0).abs() < 1.0, "width was {}", svg.width);
    }

    #[test]
    fn stroked_circle_keeps_its_hole() {
        let glyph = parse_str(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
                <circle cx="50" cy="50" r="30" stroke="black" stroke-width="10" fill="none"/>
            </svg>"#,
            "stroked_circle",
        )
        .expect("stroked circle should produce a glyph");
        let svg = glyph.svg.unwrap();
        let contours = svg.path.matches('M').count();
        assert_eq!(contours, 2, "ring should have outer and inner contours");
    }

    #[test]
    fn non_uniform_transform_scales_the_stroke_like_svg() {
        let glyph = parse_str(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="20 46 160 8">
                <g transform="scale(2 1)">
                    <path d="M10 50 L90 50" stroke="black" stroke-width="8" fill="none"/>
                </g>
            </svg>"#,
            "scaled_stroked_line",
        )
        .expect("stroked line in a scaled group should produce a glyph");
        let svg = glyph.svg.unwrap();
        // scale(2 1) stretches the painted band to 160x8, so normalizing the
        // height to 1000 units makes it 20000 wide.
        assert!((svg.width - 20000.0).abs() < 1.0, "width was {}", svg.width);
    }

    #[test]
    fn paintless_path_keeps_the_legacy_fill_behavior() {
        let glyph = parse_str(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="10 10 80 40" fill="none">
                <path d="M10 10 H90 V50 H10 Z"/>
            </svg>"#,
            "paintless_rect",
        )
        .expect("fill=none path without a stroke should still fill its geometry");
        let svg = glyph.svg.unwrap();
        assert!((svg.width - 2000.0).abs() < 1.0, "width was {}", svg.width);
    }

    #[test]
    fn fill_only_path_is_unchanged_by_stroke_support() {
        let glyph = parse_str(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="10 10 80 40">
                <rect x="10" y="10" width="80" height="40"/>
            </svg>"#,
            "filled_rect",
        )
        .expect("filled rect should produce a glyph");
        let svg = glyph.svg.unwrap();
        assert_eq!(svg.path.matches('M').count(), 1);
        assert!((svg.width - 2000.0).abs() < 1.0, "width was {}", svg.width);
    }
}
