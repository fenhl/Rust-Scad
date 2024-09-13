use scad::*;

#[wheel::main]
fn main() -> std::io::Result<()> {
    let mut scad_file = ScadFile::new();
    scad_file.add_object(scad!(Union; {
        // wall
        scad!(Difference; {
            scad!(Color(vec3(1.0, 1.0, 1.0)); scad!(Cube(vec3(10.0, 0.2, 2.5)))),
            // window
            scad!(Translate(vec3(3.0, -0.01, 1.0)); scad!(Cube(vec3(1.0, 0.22, 1.0)))),
            // door
            scad!(Translate(vec3(6.0, -0.01, -0.01)); scad!(Cube(vec3(1.0, 0.22, 2.01)))),
        }),
        // window
        scad!(Translate(vec3(3.0, 0.05, 1.0)); scad!(ColorAlpha(vec4(0.0, 1.0, 1.0, 0.5)); scad!(Cube(vec3(1.0, 0.1, 1.0))))),
        scad!(Translate(vec3(3.0, 0.1, 1.5)); scad!(Color(vec3(0.0, 0.0, 0.0)); scad!(CenteredCube(vec3(0.04, 0.24, 1.04))))),
        scad!(Translate(vec3(4.0, 0.1, 1.5)); scad!(Color(vec3(0.0, 0.0, 0.0)); scad!(CenteredCube(vec3(0.04, 0.24, 1.04))))),
        scad!(Translate(vec3(3.5, 0.1, 1.0)); scad!(Color(vec3(0.0, 0.0, 0.0)); scad!(CenteredCube(vec3(1.04, 0.24, 0.04))))),
        scad!(Translate(vec3(3.5, 0.1, 2.0)); scad!(Color(vec3(0.0, 0.0, 0.0)); scad!(CenteredCube(vec3(1.04, 0.24, 0.04))))),
        // door
        scad!(Translate(vec3(6.0, 0.0, 0.0)); scad!(Color(vec3(0.5, 0.5, 0.5)); scad!(Cube(vec3(1.0, 0.04, 2.0))))),
        scad!(Translate(vec3(6.0, 0.1, 1.0)); scad!(Color(vec3(0.0, 0.0, 0.0)); scad!(CenteredCube(vec3(0.04, 0.24, 2.04))))),
        scad!(Translate(vec3(7.0, 0.1, 1.0)); scad!(Color(vec3(0.0, 0.0, 0.0)); scad!(CenteredCube(vec3(0.04, 0.24, 2.04))))),
        scad!(Translate(vec3(6.5, 0.1, 2.0)); scad!(Color(vec3(0.0, 0.0, 0.0)); scad!(CenteredCube(vec3(1.04, 0.24, 0.04))))),
        scad!(Translate(vec3(6.85, -0.035, 1.0)); scad!(Color(vec3(1.0, 1.0, 1.0)); scad!(Sphere(Radius(0.05))))),
        scad!(Translate(vec3(6.85, 0.075, 1.0)); scad!(Color(vec3(1.0, 1.0, 1.0)); scad!(Sphere(Radius(0.05))))),
    }));
    scad_file.write_to_file("assets/generated.scad")?;
    Ok(())
}
