use nalgebra as na;
use std::fmt;
use std::string::*;
use std::vec::Vec;

use crate::scad_type::*;

/// Since scad allows creation of circle like objects using either radius or diameter,
/// this enum specifies which format to use
#[derive(Clone)]
pub enum CircleType {
    Radius(f32),
    Diameter(f32),
}

/////////////////////////////////////////////////////////////////////////////

/// Parameters for the linear extrude function.
///
/// These are in a struct because  there are so many of them and
/// most of them  can have a default value.
#[derive(Clone)]
pub struct LinExtrudeParams {
    pub height: f32,
    pub center: bool,
    pub convexity: i32,
    pub twist: f32,
    pub slices: i32,
}

impl Default for LinExtrudeParams {
    fn default() -> LinExtrudeParams {
        LinExtrudeParams {
            height: 1.,
            center: false,
            convexity: 10,
            twist: 0.,
            slices: 1,
        }
    }
}

impl ScadType for LinExtrudeParams {
    fn fmt_code(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "height=")?;
        self.height.fmt_code(f)?;
        write!(f, ",center=")?;
        self.center.fmt_code(f)?;
        write!(f, ",convexity=")?;
        self.convexity.fmt_code(f)?;
        write!(f, ",twist=")?;
        self.twist.fmt_code(f)?;
        write!(f, ",slices=")?;
        self.slices.fmt_code(f)?;
        Ok(())
    }
}

/////////////////////////////////////////////////////////////////////////////

/// Parameters for the rotate extrude function
#[derive(Clone)]
pub struct RotateExtrudeParams {
    pub angle: f32,
    pub convexity: usize,
}

impl Default for RotateExtrudeParams {
    fn default() -> RotateExtrudeParams {
        RotateExtrudeParams {
            angle: 360.,
            convexity: 10,
        }
    }
}

impl ScadType for RotateExtrudeParams {
    fn fmt_code(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "angle=")?;
        self.angle.fmt_code(f)?;
        write!(f, ",convexity=")?;
        self.convexity.fmt_code(f)?;
        Ok(())
    }
}
/////////////////////////////////////////////////////////////////////////////
/**
  Parameters for the polygon function.
*/
#[derive(Clone)]
enum PolygonPathType {
    Default,
    SingleVector(Vec<usize>),
    MultipleVectors(Vec<Vec<usize>>),
}
impl ScadType for PolygonPathType {
    fn fmt_code(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PolygonPathType::Default => write!(f, "undef")?,
            PolygonPathType::SingleVector(ref val) => val.fmt_code(f)?,
            PolygonPathType::MultipleVectors(ref val) => val.fmt_code(f)?,
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct PolygonParameters {
    points: Vec<na::Vector2<f32>>,
    path: PolygonPathType,
    convexity: u64,
}

impl PolygonParameters {
    pub fn new(points: Vec<na::Vector2<f32>>) -> PolygonParameters {
        PolygonParameters {
            points,
            path: PolygonPathType::Default,
            convexity: 10,
        }
    }

    pub fn single_vector_path(mut self, path: Vec<usize>) -> PolygonParameters {
        self.path = PolygonPathType::SingleVector(path);
        self
    }

    pub fn multi_vector_path(mut self, path: Vec<Vec<usize>>) -> PolygonParameters {
        self.path = PolygonPathType::MultipleVectors(path);
        self
    }

    pub fn convexity(mut self, convexity: u64) -> PolygonParameters {
        self.convexity = convexity;
        self
    }
}

impl ScadType for PolygonParameters {
    fn fmt_code(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "points=")?;
        self.points.fmt_code(f)?;
        write!(f, ",paths=")?;
        self.path.fmt_code(f)?;
        write!(f, ",convexity=")?;
        self.convexity.fmt_code(f)?;
        Ok(())
    }
}
/////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub enum OffsetType {
    Delta(f32),
    Radius(f32),
}

impl ScadType for OffsetType {
    fn fmt_code(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            OffsetType::Delta(val) => {
                write!(f, "delta=")?;
                val.fmt_code(f)?;
            }
            OffsetType::Radius(val) => {
                write!(f, "r=")?;
                val.fmt_code(f)?;
            }
        }
        Ok(())
    }
}
/////////////////////////////////////////////////////////////////////////////

/// Different kinds of scad modules and function. These are parameters
/// for `ScadObjects`.
///
/// Most of these have  the same name as the openscad counterparts so see
/// their documentation for details
#[derive(Clone)]
pub enum ScadElement {
    //Transformation stuff
    Translate(na::Vector3<f32>),
    Scale(na::Vector3<f32>),
    Resize(na::Vector3<f32>, bool),
    Rotate(f32, na::Vector3<f32>),
    RotateVec(na::Vector3<f32>),
    Mirror(na::Vector3<f32>),
    LinearExtrude(LinExtrudeParams),
    RotateExtrude(RotateExtrudeParams),

    Difference,
    Union,
    Hull,
    Intersection,
    Minkowski,

    //Object stuff
    Cube(na::Vector3<f32>),
    CenteredCube(na::Vector3<f32>),
    Cylinder(f32, CircleType),
    Sphere(CircleType),
    Cone(f32, CircleType, CircleType),

    Polyhedron(Vec<na::Vector3<f32>>, Vec<Vec<i32>>),
    Import(String),

    //2D stuff
    Square(na::Vector2<f32>),
    Circle(CircleType),
    Polygon(PolygonParameters),
    Offset(OffsetType, bool),
    Projection(bool),

    Rotate2d(f32),
    Translate2d(na::Vector2<f32>),
    Scale2d(na::Vector2<f32>),

    Color(na::Vector3<f32>),
    ColorAlpha(na::Vector4<f32>),
    NamedColor(String),
}

impl ScadType for ScadElement {
    /// Returns scad code for each of the elements
    fn fmt_code(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            //Transformation things
            ScadElement::Translate(value) => {
                write!(f, "translate(")?;
                value.fmt_code(f)?;
                write!(f, ")")?;
            }
            ScadElement::Scale(value) => {
                write!(f, "scale(")?;
                value.fmt_code(f)?;
                write!(f, ")")?;
            }
            ScadElement::Resize(vector, auto) => {
                write!(f, "resize(")?;
                vector.fmt_code(f)?;
                write!(f, ", auto = ")?;
                auto.fmt_code(f)?;
                write!(f, ")")?;
            }
            ScadElement::Rotate(angle, vector) => {
                write!(f, "rotate(")?;
                angle.fmt_code(f)?;
                write!(f, ",")?;
                vector.fmt_code(f)?;
                write!(f, ")")?;
            }
            ScadElement::RotateVec(vector) => {
                write!(f, "rotate(")?;
                vector.fmt_code(f)?;
                write!(f, ")")?;
            }
            ScadElement::Mirror(vector) => {
                write!(f, "mirror(")?;
                vector.fmt_code(f)?;
                write!(f, ")")?;
            }
            ScadElement::LinearExtrude(params) => {
                write!(f, "linear_extrude(")?;
                params.fmt_code(f)?;
                write!(f, ")")?;
            }
            ScadElement::RotateExtrude(params) => {
                write!(f, "rotate_extrude(")?;
                params.fmt_code(f)?;
                write!(f, ")")?;
            }

            //Primitive objects
            ScadElement::Cube(value) => {
                write!(f, "cube(")?;
                value.fmt_code(f)?;
                write!(f, ")")?;
            }
            ScadElement::CenteredCube(value) => {
                write!(f, "cube(")?;
                value.fmt_code(f)?;
                write!(f, ", center = true)")?;
            }
            ScadElement::Cylinder(height, width) => {
                write!(f, "cylinder(h=")?;
                height.fmt_code(f)?;
                write!(f, ",")?;
                match width {
                    CircleType::Radius(val) => {
                        write!(f, "r=")?;
                        val.fmt_code(f)?;
                    }
                    CircleType::Diameter(val) => {
                        write!(f, "d=")?;
                        val.fmt_code(f)?;
                    }
                }
                write!(f, ")")?;
            }
            ScadElement::Sphere(size) => {
                write!(f, "sphere(")?;
                match size {
                    CircleType::Radius(val) => {
                        write!(f, "r=")?;
                        val.fmt_code(f)?;
                    }
                    CircleType::Diameter(val) => {
                        write!(f, "d=")?;
                        val.fmt_code(f)?;
                    }
                }
                write!(f, ")")?;
            }
            ScadElement::Cone(height, size1, size2) => {
                write!(f, "cylinder(h=")?;
                height.fmt_code(f)?;
                write!(f, ",")?;
                match size1 {
                    CircleType::Radius(val) => {
                        write!(f, "r1=")?;
                        val.fmt_code(f)?;
                    }
                    CircleType::Diameter(val) => {
                        write!(f, "d1=")?;
                        val.fmt_code(f)?;
                    }
                }
                write!(f, ",")?;
                match size2 {
                    CircleType::Radius(val) => {
                        write!(f, "r2=")?;
                        val.fmt_code(f)?;
                    }
                    CircleType::Diameter(val) => {
                        write!(f, "d2=")?;
                        val.fmt_code(f)?;
                    }
                }
                write!(f, ")")?;
            }

            ScadElement::Polyhedron(points, faces) => {
                write!(f, "polyhedron(points=")?;
                points.fmt_code(f)?;
                write!(f, ",faces=")?;
                faces.fmt_code(f)?;
                write!(f, ")")?;
            }
            ScadElement::Import(path) => {
                write!(f, "import(")?;
                path.fmt_code(f)?;
                write!(f, ")")?;
            }

            //primitive 2d objects
            ScadElement::Square(value) => {
                write!(f, "square(")?;
                value.fmt_code(f)?;
                write!(f, ")")?;
            }
            ScadElement::Circle(circle_type) => {
                write!(f, "circle(")?;
                match circle_type {
                    CircleType::Radius(val) => {
                        write!(f, "r=")?;
                        val.fmt_code(f)?;
                    }
                    CircleType::Diameter(val) => {
                        write!(f, "d=")?;
                        val.fmt_code(f)?;
                    }
                }
                write!(f, ")")?;
            }

            ScadElement::Polygon(parameters) => {
                write!(f, "polygon(")?;
                parameters.fmt_code(f)?;
                write!(f, ")")?;
            }
            ScadElement::Offset(offset_type, chamfer) => {
                write!(f, "offset(")?;
                offset_type.fmt_code(f)?;
                write!(f, ",chamfer=")?;
                chamfer.fmt_code(f)?;
                write!(f, ")")?;
            }

            ScadElement::Rotate2d(angle) => {
                write!(f, "rotate(")?;
                angle.fmt_code(f)?;
                write!(f, ")")?;
            }
            ScadElement::Translate2d(position) => {
                write!(f, "translate(")?;
                position.fmt_code(f)?;
                write!(f, ")")?;
            }
            ScadElement::Scale2d(scale) => {
                write!(f, "scale(")?;
                scale.fmt_code(f)?;
                write!(f, ")")?;
            }
            ScadElement::Projection(cut) => {
                write!(f, "projection(cut=")?;
                cut.fmt_code(f)?;
                write!(f, ")")?;
            }

            //Colors
            ScadElement::Color(value) => {
                //Ensure that this is a valid color
                assert!(value.x >= 0. && value.x <= 1.);
                assert!(value.y >= 0. && value.y <= 1.);
                assert!(value.z >= 0. && value.z <= 1.);

                write!(f, "color(")?;
                value.fmt_code(f)?;
                write!(f, ")")?;
            }
            ScadElement::ColorAlpha(value) => {
                //Ensure that this is a valid color
                assert!(value.x >= 0. && value.x <= 1.);
                assert!(value.y >= 0. && value.y <= 1.);
                assert!(value.z >= 0. && value.z <= 1.);
                assert!(value.w >= 0. && value.w <= 1.);

                write!(f, "color(")?;
                value.fmt_code(f)?;
                write!(f, ")")?;
            }
            ScadElement::NamedColor(value) => {
                write!(f, "color(")?;
                value.fmt_code(f)?;
                write!(f,  ")")?;
            }

            //Combination constructs
            ScadElement::Difference => write!(f, "difference()")?,
            ScadElement::Union => write!(f, "union()")?,
            ScadElement::Hull => write!(f, "hull()")?,
            ScadElement::Minkowski => write!(f, "minkowski()")?,
            ScadElement::Intersection => write!(f, "intersection()")?,
        }
        Ok(())
    }
}
/////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod scad_tests {
    use super::*;

    #[test]
    fn simple_enum_test() {
        assert_eq!(
            ScadElement::Sphere(CircleType::Diameter(7.)).get_code(),
            "sphere(d=7)"
        );

        assert_eq!(
            ScadElement::Cone(5., CircleType::Radius(7.), CircleType::Radius(14.)).get_code(),
            "cylinder(h=5,r1=7,r2=14)"
        );
        assert_eq!(
            ScadElement::Cone(5., CircleType::Diameter(7.), CircleType::Diameter(14.)).get_code(),
            "cylinder(h=5,d1=7,d2=14)"
        );

        assert_eq!(
            ScadElement::Import("hello_world.stl".to_string()).get_code(),
            "import(\"hello_world.stl\")"
        );
        assert_eq!(
            ScadElement::Polygon(PolygonParameters::new(vec!(na::Vector2::new(1., 1.)))).get_code(),
            "polygon(points=[[1,1],],paths=undef,convexity=10)"
        );

        assert_eq!(ScadElement::Color(na::zero()).get_code(), "color([0,0,0])");
        assert_eq!(
            ScadElement::NamedColor("aqua".to_string()).get_code(),
            "color(\"aqua\")"
        );

        assert_eq!(
            ScadElement::Offset(OffsetType::Delta(5.), false).get_code(),
            "offset(delta=5,chamfer=false)"
        );

        assert_eq!(ScadElement::Minkowski.get_code(), "minkowski()");
        assert_eq!(
            ScadElement::Projection(true).get_code(),
            "projection(cut=true)"
        );
    }

    #[test]
    fn lin_extrude_test() {
        assert_eq!(
            LinExtrudeParams::default().get_code(),
            "height=1,center=false,convexity=10,twist=0,slices=1"
        );

        assert_eq!(
            LinExtrudeParams {
                twist: 720.,
                ..Default::default()
            }
            .get_code(),
            "height=1,center=false,convexity=10,twist=720,slices=1"
        );
    }

    #[test]
    fn rotate_extrude_test() {
        let obj = ScadElement::RotateExtrude(Default::default());

        assert_eq!(obj.get_code(), "rotate_extrude(angle=360,convexity=10)");
    }

    #[test]
    fn polygon_parameter_type() {
        assert_eq!(
            PolygonParameters::new(vec!(na::Vector2::new(1., 1.))).get_code(),
            "points=[[1,1],],paths=undef,convexity=10"
        );
        assert_eq!(
            PolygonParameters::new(vec!(na::Vector2::new(1., 1.)))
                .convexity(5)
                .single_vector_path(vec!(1))
                .get_code(),
            "points=[[1,1],],paths=[1,],convexity=5"
        );
    }

    #[test]
    fn test_2d() {
        assert_eq!(ScadElement::Rotate2d(90.).get_code(), "rotate(90)");
        assert_eq!(
            ScadElement::Translate2d(na::Vector2::new(1., 1.)).get_code(),
            "translate([1,1])"
        );
        assert_eq!(
            ScadElement::Scale2d(na::Vector2::new(1., 1.)).get_code(),
            "scale([1,1])"
        );

        assert_eq!(
            ScadElement::Circle(CircleType::Radius(10.)).get_code(),
            "circle(r=10)"
        );
        assert_eq!(
            ScadElement::Circle(CircleType::Diameter(10.)).get_code(),
            "circle(d=10)"
        );
    }
}
