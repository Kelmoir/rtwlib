//! # Three.js Export
//! This module provides functionality to export a scene to a Three.js compatible JSON format.
//! It also includes a function to create an HTML file with a Three.js viewer for the exported scene.

use crate::hittable::HittableList;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::path::Path;
use std::fs::File;
use std::io::Write;

/// A struct to track ray paths for visualization
pub struct RayPath {
    /// Points along the ray path   
    pub points: Vec<Vec3>,  
    /// Color of the ray path
    pub color: String,     
}

/// Exports a scene to a Three.js compatible JSON format
/// 
/// # Arguments
/// * `world` - The scene to export
/// * `output_path` - The path to save the JSON file
/// * `include_rays` - Whether to include ray paths in the export
/// * `ray_color` - The color to use for rays (in hex format, e.g. "#ff0000")
/// * `ray_paths` - Optional vector of ray paths to include in the visualization
pub fn export_to_three(
    world: &HittableList,
    output_path: &Path,
    include_rays: bool,
    ray_color: &str,
    ray_paths: Option<Vec<RayPath>>,
) -> std::io::Result<()> {
    let mut file = File::create(output_path)?;
    
    // Start JSON structure
    writeln!(file, "{{")?;
    writeln!(file, "  \"metadata\": {{")?;
    writeln!(file, "    \"version\": 4.5,")?;
    writeln!(file, "    \"type\": \"scene\",")?;
    writeln!(file, "    \"generator\": \"rtwlib\"")?;
    writeln!(file, "  }},")?;
    
    // Start objects array
    writeln!(file, "  \"object\": {{")?;
    writeln!(file, "    \"uuid\": \"scene\",")?;
    writeln!(file, "    \"type\": \"Scene\",")?;
    writeln!(file, "    \"children\": [")?;
    
    // Export scene objects
    let mut first = true;
    let mut added_item = false;
    for (i, object) in world.objects.iter().enumerate() {
        if !first && added_item{
            writeln!(file, ",")?;
        }
        first = false;
        added_item = false;
        // Get object info
        let info = object.as_info_vec();
        let mut obj_type = object.as_string();
        obj_type = obj_type.split_whitespace().nth(0).unwrap().to_string();
        
        // Create Three.js object based on type
        match obj_type.as_str() {
            "Sphere" => {
                // Parse sphere info
                let radius = info[1].parse::<f64>().unwrap();
                let center = Vec3::new(
                    info[2].parse::<f64>().unwrap(),
                    info[3].parse::<f64>().unwrap(),
                    info[4].parse::<f64>().unwrap(),
                );
                
                writeln!(file, "      {{")?;
                writeln!(file, "        \"uuid\": \"sphere_{}\",", i)?;
                writeln!(file, "        \"type\": \"Mesh\",")?;
                writeln!(file, "        \"geometry\": {{")?;
                writeln!(file, "          \"uuid\": \"sphere_geom_{}\",", i)?;
                writeln!(file, "          \"type\": \"SphereGeometry\",")?;
                writeln!(file, "          \"radius\": {},", radius)?;
                writeln!(file, "          \"widthSegments\": 32,")?;
                writeln!(file, "          \"heightSegments\": 32")?;
                writeln!(file, "        }},")?;
                writeln!(file, "        \"material\": {{")?;
                writeln!(file, "          \"uuid\": \"sphere_mat_{}\",", i)?;
                writeln!(file, "          \"type\": \"MeshStandardMaterial\",")?;
                writeln!(file, "          \"color\": \"#808080\",")?;
                writeln!(file, "          \"metalness\": 0.0,")?;
                writeln!(file, "          \"roughness\": 0.5")?;
                writeln!(file, "        }},")?;
                writeln!(file, "        \"position\": [{}, {}, {}],", center.x, center.y, center.z)?;
                writeln!(file, "        \"scale\": [1, 1, 1],")?;
                writeln!(file, "        \"rotation\": [0, 0, 0]")?;
                writeln!(file, "      }}")?;
                added_item = true;
            },
            "Plane" => {
                // Parse plane info
                let point = parse_vec3(&info[0]);
                let normal = parse_vec3(&info[1]);
                
                // Create a large plane in Three.js
                writeln!(file, "      {{")?;
                writeln!(file, "        \"uuid\": \"plane_{}\",", i)?;
                writeln!(file, "        \"type\": \"Mesh\",")?;
                writeln!(file, "        \"geometry\": {{")?;
                writeln!(file, "          \"uuid\": \"plane_geom_{}\",", i)?;
                writeln!(file, "          \"type\": \"PlaneGeometry\",")?;
                writeln!(file, "          \"width\": 100,")?;
                writeln!(file, "          \"height\": 100")?;
                writeln!(file, "        }},")?;
                writeln!(file, "        \"material\": {{")?;
                writeln!(file, "          \"uuid\": \"plane_mat_{}\",", i)?;
                writeln!(file, "          \"type\": \"MeshStandardMaterial\",")?;
                writeln!(file, "          \"color\": \"#808080\",")?;
                writeln!(file, "          \"metalness\": 0.0,")?;
                writeln!(file, "          \"roughness\": 0.5,")?;
                writeln!(file, "          \"side\": 2")?;
                writeln!(file, "        }},")?;
                writeln!(file, "        \"position\": [{}, {}, {}],", point.x, point.y, point.z)?;
                writeln!(file, "        \"rotation\": [{}, {}, {}],", 
                    normal.x.atan2(normal.z), 
                    normal.y.atan2(normal.z), 
                    0.0)?;
                writeln!(file, "        \"scale\": [1, 1, 1]")?;
                writeln!(file, "      }}")?;
                added_item = true;
            },
            "ExtrudedObject" => {
                // Parse extruded object info
                let points = parse_points(&info[4..].join(";"));
                let height = parse_f64(&info[2]).unwrap();
                let direction = parse_vec3(&info[1]);
                
                // Create a mesh from the points
                writeln!(file, "      {{")?;
                writeln!(file, "        \"uuid\": \"extruded_{}\",", i)?;
                writeln!(file, "        \"type\": \"Mesh\",")?;
                writeln!(file, "        \"geometry\": {{")?;
                writeln!(file, "          \"uuid\": \"extruded_geom_{}\",", i)?;
                writeln!(file, "          \"type\": \"ExtrudeGeometry\",")?;
                writeln!(file, "          \"shapes\": [")?;
                writeln!(file, "            {{")?;
                writeln!(file, "              \"type\": \"Shape\",")?;
                writeln!(file, "              \"points\": [")?;
                
                // Add points
                for (j, point) in points.iter().enumerate() {
                    if j > 0 {
                        writeln!(file, ",")?;
                    }
                    writeln!(file, "                [{}, {}]", point.x, point.y)?;
                }
                
                writeln!(file, "              ]")?;
                writeln!(file, "            }}")?;
                writeln!(file, "          ],")?;
                writeln!(file, "          \"options\": {{")?;
                writeln!(file, "            \"depth\": {},", height)?;
                writeln!(file, "            \"bevelEnabled\": false")?;
                writeln!(file, "          }}")?;
                writeln!(file, "        }},")?;
                writeln!(file, "        \"material\": {{")?;
                writeln!(file, "          \"uuid\": \"extruded_mat_{}\",", i)?;
                writeln!(file, "          \"type\": \"MeshStandardMaterial\",")?;
                writeln!(file, "          \"color\": \"#808080\",")?;
                writeln!(file, "          \"metalness\": 0.0,")?;
                writeln!(file, "          \"roughness\": 0.5,")?;
                writeln!(file, "          \"transparent\": true,")?;
                writeln!(file, "          \"opacity\": 0.8")?;
                writeln!(file, "        }},")?;
                writeln!(file, "        \"position\": [0, 0, 0],")?;
                writeln!(file, "        \"rotation\": [{}, {}, {}],", 
                    direction.x.atan2(direction.z), 
                    direction.y.atan2(direction.z), 
                    0.0)?;
                writeln!(file, "        \"scale\": [1, 1, 1]")?;
                writeln!(file, "      }}")?;
                added_item = true;
            },
            _ => {
                // Unknown object type, skip
                continue;
            }
        }
    }
    
    // Export rays if requested
    if include_rays {
        if let Some(paths) = ray_paths {
            for (i, path) in paths.iter().enumerate() {
                if !first {
                    writeln!(file, ",")?;
                }
                first = false;
                
                // Create a line for each ray path
                writeln!(file, "      {{")?;
                writeln!(file, "        \"uuid\": \"ray_{}\",", i)?;
                writeln!(file, "        \"type\": \"Line\",")?;
                writeln!(file, "        \"geometry\": {{")?;
                writeln!(file, "          \"uuid\": \"ray_geom_{}\",", i)?;
                writeln!(file, "          \"type\": \"BufferGeometry\",")?;
                writeln!(file, "          \"attributes\": {{")?;
                writeln!(file, "            \"position\": {{")?;
                writeln!(file, "              \"itemSize\": 3,")?;
                writeln!(file, "              \"type\": \"Float32Array\",")?;
                writeln!(file, "              \"array\": [")?;
                
                // Add points
                for (j, point) in path.points.iter().enumerate() {
                    if j > 0 {
                        writeln!(file, ",")?;
                    }
                    writeln!(file, "                {}, {}, {}", point.x, point.y, point.z)?;
                }
                
                writeln!(file, "              ]")?;
                writeln!(file, "            }}")?;
                writeln!(file, "          }}")?;
                writeln!(file, "        }},")?;
                writeln!(file, "        \"material\": {{")?;
                writeln!(file, "          \"uuid\": \"ray_mat_{}\",", i)?;
                writeln!(file, "          \"type\": \"LineBasicMaterial\",")?;
                writeln!(file, "          \"color\": \"{}\",", path.color)?;
                writeln!(file, "          \"linewidth\": 1")?;
                writeln!(file, "        }}")?;
                writeln!(file, "      }}")?;
            }
        }
    }
    
    // Close JSON structure
    writeln!(file, "    ]")?;
    writeln!(file, "  }}")?;
    writeln!(file, "}}")?;
    
    Ok(())
}

/// Creates an HTML file with Three.js viewer for the exported scene
/// 
/// # Arguments
/// * `json_path` - Path to the exported JSON file
/// * `output_path` - Path to save the HTML file
pub fn create_viewer_html(json_path: &Path, output_path: &Path) -> std::io::Result<()> {
    let mut file = File::create(output_path)?;
    
    let html_template = format!(r#"<!DOCTYPE html>
<html>
<head>
    <title>RTW Scene Viewer</title>
    <style>
        body {{ margin: 0; }}
        canvas {{ display: block; }}
        #controls {{
            position: absolute;
            top: 10px;
            left: 10px;
            background: rgba(255, 255, 255, 0.8);
            padding: 10px;
            border-radius: 5px;
        }}
    </style>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js"></script>
    <script src="https://cdn.jsdelivr.net/npm/three@0.128.0/examples/js/controls/OrbitControls.js"></script>
</head>
<body>
    <div id="controls">
        <label>
            <input type="checkbox" id="showRays" checked>
            Show Rays
        </label>
    </div>
    <script>
        // Wait for DOM and scripts to be fully loaded
        window.addEventListener('load', function() {{
            // Scene setup
            const scene = new THREE.Scene();
            const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
            const renderer = new THREE.WebGLRenderer();
            renderer.setSize(window.innerWidth, window.innerHeight);
            document.body.appendChild(renderer.domElement);
            
            // Add lights
            const ambientLight = new THREE.AmbientLight(0xffffff, 0.5);
            scene.add(ambientLight);
            
            const directionalLight = new THREE.DirectionalLight(0xffffff, 0.5);
            directionalLight.position.set(10, 10, 10);
            scene.add(directionalLight);
            
            // Controls
            const controls = new THREE.OrbitControls(camera, renderer.domElement);
            camera.position.set(20, 20, 20);
            controls.update();
            
            // Load scene
            fetch('{}')
                .then(response => response.json())
                .then(data => {{
                    // Process each object in the scene
                    data.object.children.forEach(child => {{
                        if (child.type === 'Mesh') {{
                            // Create geometry
                            const geometry = new THREE.SphereGeometry(
                                child.geometry.radius,
                                child.geometry.widthSegments,
                                child.geometry.heightSegments
                            );

                            // Create material
                            const material = new THREE.MeshStandardMaterial({{
                                color: child.material.color,
                                metalness: child.material.metalness,
                                roughness: child.material.roughness
                            }});

                            // Create mesh
                            const mesh = new THREE.Mesh(geometry, material);
                            
                            // Set position, rotation, and scale
                            mesh.position.set(...child.position);
                            mesh.rotation.set(...child.rotation);
                            mesh.scale.set(...child.scale);

                            scene.add(mesh);
                        }}
                    }});
                }})
                .catch(error => {{
                    console.error('Error loading scene:', error);
                }});
            
            // Handle window resize
            window.addEventListener('resize', onWindowResize, false);
            function onWindowResize() {{
                camera.aspect = window.innerWidth / window.innerHeight;
                camera.updateProjectionMatrix();
                renderer.setSize(window.innerWidth, window.innerHeight);
            }}
            
            // Animation loop
            function animate() {{
                requestAnimationFrame(animate);
                controls.update();
                renderer.render(scene, camera);
            }}
            animate();
            
            // Handle ray visibility toggle
            document.getElementById('showRays').addEventListener('change', function(e) {{
                if (window.rayObjects) {{
                    window.rayObjects.forEach(ray => {{
                        ray.visible = e.target.checked;
                    }});
                }}
            }});
        }});
    </script>
</body>
</html>"#, json_path.to_str().unwrap());
    
    writeln!(file, "{}", html_template)?;
    
    Ok(())
}

/// Helper function to parse a f64 from a string
fn parse_f64(s: &str) -> Result<f64, std::num::ParseFloatError> {
    let input = if let Some(second) = s.split(":").nth(1) {
        second.trim()
    } else {
        s.trim()
    };
    let parts: Vec<&str> = input.trim_matches(|c| c == '(' || c == ')')
        .split(',')
        .map(|input| input.trim())
        .collect();
        
    parts[0].parse::<f64>()
}

/// Helper function to parse a Vec3 from a string
fn parse_vec3(s: &str) -> Vec3 {
    let input = if let Some(second) = s.split(":").nth(1) {
        second.trim()
    } else {
        s.trim()
    };
    let parts: Vec<&str> = input.trim_matches(|c| c == '(' || c == ')')
        .split(',')
        .map(|input| input.trim())
        .collect();
        
    Vec3::new(
        parts[0].parse::<f64>().unwrap(),
        parts[1].parse::<f64>().unwrap(),
        parts[2].parse::<f64>().unwrap()
    )
}

/// Helper function to parse a list of points from a string
fn parse_points(s: &str) -> Vec<Vec3> {
    let points_str = s.trim_matches(|c| c == '[' || c == ']');
    let points: Vec<&str> = points_str.split(';')
        .map(|s| s.trim())
        .collect();
    
    points.iter()
        .map(|&p| parse_vec3(p))
        .collect()
} 