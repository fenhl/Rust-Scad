use nalgebra as na;
use std::fmt;
use std::string::String;
use std::vec::Vec;

/// Trait for converting from rust types to strings compatible with openscad
pub trait ScadType {
    fn fmt_code(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;

    fn get_code(&self) -> String {
        struct ScadFormatter<'a, T: ScadType + ?Sized>(&'a T);

        impl<'a, T: ScadType + ?Sized> fmt::Display for ScadFormatter<'a, T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt_code(f)
            }
        }

        ScadFormatter(self).to_string()
    }
}

impl ScadType for na::Vector4<f32> {
    fn fmt_code(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        self.x.fmt_code(f)?;
        write!(f, ",")?;
        self.y.fmt_code(f)?;
        write!(f, ",")?;
        self.z.fmt_code(f)?;
        write!(f, ",")?;
        self.w.fmt_code(f)?;
        write!(f, "]")?;
        Ok(())
    }
}
impl ScadType for na::Vector3<f32> {
    fn fmt_code(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        self.x.fmt_code(f)?;
        write!(f, ",")?;
        self.y.fmt_code(f)?;
        write!(f, ",")?;
        self.z.fmt_code(f)?;
        write!(f, "]")?;
        Ok(())
    }
}
impl ScadType for na::Vector2<f32> {
    fn fmt_code(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        self.x.fmt_code(f)?;
        write!(f, ",")?;
        self.y.fmt_code(f)?;
        write!(f, "]")?;
        Ok(())
    }
}

impl ScadType for f32 {
    fn fmt_code(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}
impl ScadType for i32 {
    fn fmt_code(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}
impl ScadType for usize {
    fn fmt_code(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}
impl ScadType for u64 {
    fn fmt_code(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}
impl ScadType for bool {
    fn fmt_code(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl<T: ScadType> ScadType for Vec<T> {
    fn fmt_code(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for elem in self {
            elem.fmt_code(f)?;
            write!(f, ",")?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl ScadType for String {
    fn fmt_code(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[cfg(test)]
mod type_tests {
    use crate::scad_type::*;

    #[test]
    fn type_test() {
        //No more tests needed for now. I assume the to_string() function works
        //as expected
        assert_eq!(na::Vector3::new(0.0, 0.0, 0.0).get_code(), "[0,0,0]");
        assert_eq!(na::Vector3::new(-5.0, 0.0, 0.0).get_code(), "[-5,0,0]");
        assert_eq!(na::Vector3::new(1.0, 2.0, 3.0).get_code(), "[1,2,3]");

        assert_eq!(na::Vector2::new(1.0, 3.3).get_code(), "[1,3.3]");

        assert_eq!(vec!(1, 2, 3, 4, 5, 6).get_code(), "[1,2,3,4,5,6,]");
    }
}
