# rtwlib_ir

This crate is a fork from  [rtwlib](https://github.com/jamdotjar/rtwlib).
While keeping the original code mostly around, this library is more focused around tracking diffuse emissions of IR light from materials, and where they travel. To that end, Am emitter structure is supposed to provide a very simplified emission area, IR_dielectrics, that can handle sellmeier refraction index curves and more specific absorption curves.

Actual Pictures are secondary, for what I built this library, as it was mainly made to get a better understanding of rust, and help me with a private project. But they can still be generated through the detector material, even counting up, how much energy of the rays were recorded.

It is not really optimized, but that might be added, later on.

I Mainly indented 3 Materials to be used with the ir context:
- IR_dielectric: Essentially a normal dielectric, with optional sellmeier coefficients for wavelength specific optical densities and wavelength specific absorption spectra.
- Perfect_mirror: Solely a tool to reduce scene complexity, as they ought to replace symmetry planes in real life.
- detector: It will detect and log all incoming rays with their remaining power. 

The crate provides a basic svg representation of the scene, if it can easily be projected onto a plane - My scenes did provide this, and I extruded them along the z-axis.

Also, Rays can be logged, if needed for more in depth debugging.

Usage example, with a more focussed ray emission:

```rust
use rtwlib::emitter::Emitter;
use rtwlib::hittable::extruded::polygon::Polygon;
use rtwlib::hittable::extruded::ExtrudedObject;
use rtwlib::hittable::extruded::ExtrudableOutline;
use rtwlib::hittable::HittableList;
use rtwlib::material::IrDielectric;
use rtwlib::material::Material;
use rtwlib::vec3::Vec3;
use std::path::Path;
use std::rc::Rc;

fn main() {
    let mut world = HittableList::new();
    create_prism(&mut world);
    let number_of_rays:u32 = 200;

    let mat = IrDielectric::new_simple(1.0, vec![(0., 0.), (100., 0.)]);
    let mut emitter = Emitter::new(
        Vec3::new(-12.0, -5.0, 5.0),
        Vec3::new(-12.0, -5.0, 5.0),
        Vec3::new(1., 0.5, 0.).normalized(),
        25.+273.15,
        Rc::new(mat),
        number_of_rays,
        true,
    );
    emitter.enable_logging();
    emitter.emit_focussed_rays(1, number_of_rays, &world,0.01);
    save_svg(&world, &emitter, "prism".to_string());
    emitter.print_ray_summary();
    for item in emitter.get_ray_logs() {
        for index in 0..item.interactions.len()-1 {
            if item.interactions[index].object_name == item.interactions[index+1].object_name {
                item.print_summary();
            }
        }
    }
}

fn save_svg(world: &HittableList, emitter: &Emitter, filename:String) {
    let filename = format!("results/svg/{}.svg", filename);
    let normal = Vec3::new(0., 0., 1.);
    rtwlib::svg_export::export_to_svg(world, &Path::new(&filename), normal, Some(emitter)).unwrap();
}

fn create_prism(world: &mut HittableList) {
    let outline: Rc<dyn ExtrudableOutline> = Rc::new(Polygon::new(vec![
        Vec3::new(-10.0, -10.0, 0.0),
        Vec3::new(0.0, 10.0, 0.0),
        Vec3::new(10.0, -10.0, 0.0),
    ]));
    let mat = IrDielectric::new_simple(1.5, vec![(0., 0.), (100., 0.)]);
    //let mat = IrDielectric::new( vec![(0., 0.), (100., 0.)], 1.0, 0.2, 0.01);
    let prism = ExtrudedObject::new(outline, 10.0, Vec3::new(0.0, 0.0, 1.0), Rc::new(mat));
    world.add(prism);
}
```
