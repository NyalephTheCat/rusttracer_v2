use image::RgbImage;
use rusttracer::{
    engine::{camera::Camera, scene::Scene},
    generators::metaball::{Metaball, MetaballPoint},
    light::point::PointLight,
    objects::Object,
};

fn main() {
    let mut image = RgbImage::new(1000, 1000);
    let scene = Scene::default()
        .with_camera(Camera::default())
        .with_background("#1CB5E0".into())
        .with_object(
            Metaball::default()
                .with_bounds(((-10.0, -10.0, -10.0).into(), (10.0, 10.0, 10.0).into()))
                .with_point(MetaballPoint::default().with_position((5.0, 0.0, 0.0).into()))
                //.with_point(MetaballPoint::default().with_position((5.0, 0.0, -1.0).into()))
                .with_resolution(0.1)
                .with_fronteer(1.5)
                .into(),
        )
        .with_light(
            PointLight::default()
                .with_position((5.0, 10.0, 10.0).into())
                .with_color("#00FF00".into())
                .with_intensity(10.0)
                .into(),
        )
        .with_light(
            PointLight::default()
                .with_position((5.0, 10.0, -10.0).into())
                .with_color("#FF0000".into())
                .with_intensity(10.0)
                .into(),
        );

    match &scene.objects[0] {
        Object::Mesh(mesh) => {
            let mut file = std::fs::File::create("mesh.obj").unwrap();
            mesh.write_to_obj(&mut file);
        }
        Object::Sphere(_) => todo!(),
        Object::Plane(_) => todo!(),
        Object::Triangle(_) => todo!(),
        Object::SmoothTriangle(_) => todo!(),
    }

    scene.render_into(&mut image);
    image.save("image.png").unwrap();
}
