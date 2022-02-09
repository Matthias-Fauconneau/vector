mod mod_xy {
vector!(2 xy T T, x y, X Y);

impl<T> xy<T> { pub fn yx(self) -> xy<T> { xy{x: self.y, y: self.x} } }

impl xy<u32> { pub const fn signed(self) -> xy<i32> { xy{x: self.x as i32, y: self.y as i32} } }
impl xy<i32> { pub const fn unsigned(self) -> xy<u32> { xy{x: self.x as u32, y: self.y as u32} } }
impl From<xy<i32>> for xy<u32> { fn from(i: xy<i32>) -> Self { i.unsigned() } }
impl From<xy<u32>> for xy<i32> { fn from(u: xy<u32>) -> Self { u.signed() } }
impl From<xy<u32>> for xy<f32> { fn from(f: xy<u32>) -> Self { xy{x: f.x as f32, y: f.y as f32} } }
impl From<xy<i32>> for xy<f32> { fn from(f: xy<i32>) -> Self { xy{x: f.x as f32, y: f.y as f32} } }

#[allow(non_camel_case_types)] pub type uint2 = xy<u32>;
#[allow(non_camel_case_types)] pub type int2 = xy<i32>;
#[allow(non_camel_case_types)] pub type size = xy<u32>;
#[allow(non_camel_case_types)] pub type vec2 = xy<f32>;

pub fn div_ceil(n: uint2, d: u32) -> uint2 { xy{x:num::div_ceil(n.x,d), y:num::div_ceil(n.y,d)} }
pub fn lerp(t: f32, a: vec2, b: vec2) -> xy<f32> { (1.-t)*a + t*b }
pub fn dot(a: vec2, b: vec2) -> f32 { a.x*b.x + a.y*b.y }
pub fn cross(a: vec2, b: vec2) -> f32 { a.x*b.y - a.y*b.x }
pub fn sq(x:vec2) -> f32 { dot(x, x) }
pub fn norm(v:vec2) -> f32 { sq(v).sqrt() }
pub fn atan(v:vec2) -> f32 { v.y.atan2(v.x) }

pub type Rect = crate::MinMax<int2>;
impl Rect { pub fn size(&self) -> size { (self.max-self.min).unsigned() } }

impl From<size> for Rect { fn from(size: size) -> Self { Self{ min: num::zero(), max: size.signed()} } }

impl std::ops::Add<Rect> for int2 { type Output=Rect; #[track_caller] fn add(self, r: Rect) -> Self::Output { Rect{min:self+r.min, max:self+r.max} } }
impl std::ops::Sub<uint2> for Rect { type Output=Rect; #[track_caller] fn sub(self, b: uint2) -> Self::Output { -b.signed()+self } }

pub fn ceil(scale: num::Ratio, v: uint2) -> uint2 { v.map(|&c| scale.ceil(c)) }
pub fn ifloor(scale: num::Ratio, v: int2) -> int2 { v.map(|&c| scale.ifloor(c)) }
pub fn iceil(scale: num::Ratio, v: int2) -> int2 { v.map(|&c| scale.iceil(c)) }

macro_rules! forward_ref_binop {{$Op:ident, $op:ident, $u:ty, $t:ty} => {
	impl<'t> std::ops::$Op<$u> for &'t $t { type Output = <$t as std::ops::$Op<$u>>::Output; fn $op(self, b: $u) -> Self::Output { std::ops::$Op::$op(*self, b) } }
	impl std::ops::$Op<&$u> for $t { type Output = <$t as std::ops::$Op<$u>>::Output; fn $op(self, b: &$u) -> Self::Output { std::ops::$Op::$op(self, *b) } }
	impl std::ops::$Op<&$u> for &$t { type Output = <$t as std::ops::$Op<$u>>::Output; fn $op(self, b: &$u) -> Self::Output { std::ops::$Op::$op(*self, *b) } }
}}

impl std::ops::Mul<int2> for num::Ratio { type Output=int2; #[track_caller] fn mul(self, b: int2) -> Self::Output { ifloor(self, b) } }
forward_ref_binop!{Mul, mul, int2, num::Ratio}

impl std::ops::Div<num::Ratio> for uint2 { type Output=uint2; fn div(self, r: num::Ratio) -> Self::Output { xy{x:self.x/r, y:self.y/r} } }
forward_ref_binop!{Div, div, num::Ratio, uint2}

impl std::ops::Mul<Rect> for num::Ratio { type Output=Rect; fn mul(self, b: Rect) -> Self::Output { Rect{min: ifloor(self, b.min), max: iceil(self, b.max)} } }
}
pub use mod_xy::*;

mod mod_xyz {
	vector!(3 xyz T T T, x y z, X Y Z);
	#[allow(non_camel_case_types)] pub type vec3 = xyz<f32>;
}
pub use mod_xyz::*;