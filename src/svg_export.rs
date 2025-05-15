use crate::hittable::HittableList;
use crate::vec3::Vec3;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Exports the HittableList to an SVG file
/// 
/// # Arguments
/// * `hittable_list` - The list of hittable objects to export
/// * `target_file` - The path to save the SVG file to
/// 
/// # Returns
/// Result indicating success or failure of the export
pub fn export_to_svg(hittable_list: &HittableList, target_file: &Path, normal: Vec3) -> std::io::Result<()> {
    // Create SVG header with viewBox
    let mut svg_content = String::from(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<svg width="100%" height="100%" viewBox="-50 -50 100 100" xmlns="http://www.w3.org/2000/svg">
"#,
    );

    // Add SVG content from each hittable object in reverse order
    // This ensures objects added later are drawn on top
    for object in hittable_list.objects.iter().rev() {
        svg_content.push_str(&object.to_svg(normal));
    }

    // Close SVG tag
    svg_content.push_str("</svg>");

    // Write to file
    let mut file = File::create(target_file)?;
    file.write_all(svg_content.as_bytes())?;

    Ok(())
}
