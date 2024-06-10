mod mod_xy {
vector!(2 xy T T, x y, X Y);

impl<T:std::fmt::Display> std::fmt::Display for xy<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "({}, {})", self.x, self.y) }
}

impl<T> xy<T> { pub fn yx(self) -> xy<T> { xy{x: self.y, y: self.x} } }

impl xy<u32> { pub const fn signed(self) -> xy<i32> { xy{x: self.x as i32, y: self.y as i32} } }
impl xy<i32> { #[track_caller] pub fn try_unsigned(self) -> Option<xy<u32>> { self.map(|s| s.try_into().ok()).transpose() } }
impl xy<i32> { #[track_caller] pub fn unsigned(self) -> xy<u32> { self.try_unsigned().unwrap() } }
impl From<xy<i32>> for xy<u32> { fn from(i: xy<i32>) -> Self { i.unsigned() } }
impl From<xy<u32>> for xy<i32> { fn from(u: xy<u32>) -> Self { u.signed() } }
impl From<xy<i32>> for xy<f32> { fn from(f: xy<i32>) -> Self { xy{x: f.x as f32, y: f.y as f32} } }
impl From<xy<f32>> for xy<i32> { fn from(f: xy<f32>) -> Self { xy{x: f.x as i32, y: f.y as i32} } }
//impl xy<f32> { pub const fn round(self) -> xy<i32> { xy{x: self.x.round() as i32, y: self.y.round() as i32} } }

#[allow(non_camel_case_types)] pub type uint2 = xy<u32>;
#[allow(non_camel_case_types)] pub type int2 = xy<i32>;
#[allow(non_camel_case_types)] pub type size = uint2;
#[allow(non_camel_case_types)] pub type vec2 = xy<f32>;

pub fn cross2(a: vec2, b: vec2) -> f32 { a.x*b.y - a.y*b.x }
pub fn atan(v:vec2) -> f32 { v.y.atan2(v.x) }
pub fn rotate(angle: f32, xy{x,y}: vec2) -> vec2 { let (s,c)=f32::sin_cos(angle); xy{x: c*x - s*y, y: s*x + c*y} }

#[cfg(feature="int_roundings")] use num::Ratio;
#[cfg(feature="int_roundings")] pub fn ceil(scale: Ratio, v: uint2) -> uint2 { v.map(|c| scale.ceil(c)) }
#[cfg(feature="int_roundings")] pub fn ifloor(scale: Ratio, v: int2) -> int2 { v.map(|c| scale.ifloor(c)) }
#[cfg(feature="int_roundings")] pub fn iceil(scale: Ratio, v: int2) -> int2 { v.map(|c| scale.iceil(c)) }

#[cfg(feature="int_roundings")] macro_rules! forward_ref_binop {{$Op:ident, $op:ident, $u:ty, $t:ty} => {
	impl<'t> $Op<$u> for &'t $t { type Output = <$t as $Op<$u>>::Output; fn $op(self, b: $u) -> Self::Output { $Op::$op(*self, b) } }
	impl $Op<&$u> for $t { type Output = <$t as $Op<$u>>::Output; fn $op(self, b: &$u) -> Self::Output { $Op::$op(self, *b) } }
	impl $Op<&$u> for &$t { type Output = <$t as $Op<$u>>::Output; fn $op(self, b: &$u) -> Self::Output { $Op::$op(*self, *b) } }
}}

#[cfg(feature="int_roundings")] use std::ops::{Mul,Div};
#[cfg(feature="int_roundings")] impl Mul<uint2> for Ratio { type Output=uint2; #[track_caller] fn mul(self, b: uint2) -> Self::Output { ceil(self, b) } }
#[cfg(feature="int_roundings")] forward_ref_binop!{Mul, mul, uint2, Ratio}
#[cfg(feature="int_roundings")] impl Mul<int2> for Ratio { type Output=int2; #[track_caller] fn mul(self, b: int2) -> Self::Output { ifloor(self, b) } }
#[cfg(feature="int_roundings")] forward_ref_binop!{Mul, mul, int2, Ratio}
#[cfg(feature="int_roundings")] impl Div<Ratio> for uint2 { type Output=uint2; fn div(self, r: Ratio) -> Self::Output { xy{x:self.x/r, y:self.y/r} } }
#[cfg(feature="int_roundings")] impl Div<Ratio> for int2 { type Output=int2; fn div(self, r: Ratio) -> Self::Output { xy{x:self.x/r, y:self.y/r} } }
#[cfg(feature="int_roundings")] forward_ref_binop!{Div, div, Ratio, uint2}

pub type Rect = crate::MinMax<int2>;

use std::ops::{Add,Sub};
impl Add<Rect> for int2 { type Output=Rect; #[track_caller] fn add(self, r: Rect) -> Self::Output { Rect{min:self+r.min, max:self+r.max} } }
impl Sub<uint2> for Rect { type Output=Rect; #[track_caller] fn sub(self, b: uint2) -> Self::Output { -b.signed()+self } }

impl From<size> for Rect { fn from(size: size) -> Self { Self{ min: num::zero(), max: size.signed()} } }

#[cfg(feature="int_roundings")] pub fn div_ceil(n: uint2, d: u32) -> uint2 { xy{x: u32::div_ceil(n.x,d), y: u32::div_ceil(n.y,d)} }

#[cfg(feature="int_roundings")] impl Mul<Rect> for Ratio { type Output=Rect; fn mul(self, b: Rect) -> Self::Output { Rect{min: ifloor(self, b.min), max: iceil(self, b.max)} } }
}
pub use mod_xy::*;

mod mod_xyz {
	vector!(3 xyz T T T, x y z, X Y Z);
	#[allow(non_camel_case_types)] pub type vec3 = xyz<f32>;
	impl<T> xyz<T> {
		pub fn xy_z(super::xy{x,y}: super::xy<T>, z: T) -> Self { xyz{x,y,z} }
		pub fn xy(self) -> super::xy<T> { let xyz{x,y,..} = self; super::xy{x, y} }
		pub fn yz(self) -> super::xy<T> { let xyz{y,z,..} = self; super::xy{x: y, y: z} }
		pub fn zx(self) -> super::xy<T> { let xyz{z,x,..} = self; super::xy{x: z, y: x} }
		pub fn xz(self) -> super::xy<T> { let xyz{x,z,..} = self; super::xy{x, y: z} }
	}
	pub fn cross(a: vec3, b: vec3) -> vec3 { xyz{x: a.y*b.z - a.z*b.y, y: a.z*b.x - a.x*b.z, z: a.x*b.y - a.y*b.x} }
  pub fn tangent_space(n@xyz{x,y,z}: vec3) -> (vec3, vec3) { let t = crate::normalize(if x > y { xyz{x: -z, y: 0., z: x} } else { xyz{x: 0., y: z, z: -y} }); (t, crate::normalize(cross(n, t))) }
}
pub use mod_xyz::*;
