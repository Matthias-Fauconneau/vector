mod mod_xy {
vector!(2 xy T T, x y, X Y);

impl<T:core::fmt::Display> core::fmt::Display for xy<T> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { write!(f, "{},{}", self.x, self.y) }
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
#[cfg(feature="std")] pub fn atan(v:vec2) -> f32 { v.y.atan2(v.x) }
#[cfg(feature="std")] pub fn rotate(angle: f32, xy{x,y}: vec2) -> vec2 { let (s,c)=f32::sin_cos(angle); xy{x: c*x - s*y, y: s*x + c*y} }

#[cfg(feature="int_roundings")] use num::Ratio;
#[cfg(feature="int_roundings")] pub fn ceil(scale: Ratio, v: uint2) -> uint2 { v.map(|c| scale.ceil(c)) }
#[cfg(feature="int_roundings")] pub fn ifloor(scale: Ratio, v: int2) -> int2 { v.map(|c| scale.ifloor(c)) }
#[cfg(feature="int_roundings")] pub fn iceil(scale: Ratio, v: int2) -> int2 { v.map(|c| scale.iceil(c)) }

#[cfg(feature="int_roundings")] macro_rules! forward_ref_binop {{$Op:ident, $op:ident, $u:ty, $t:ty} => {
	impl<'t> $Op<$u> for &'t $t { type Output = <$t as $Op<$u>>::Output; fn $op(self, b: $u) -> Self::Output { $Op::$op(*self, b) } }
	impl $Op<&$u> for $t { type Output = <$t as $Op<$u>>::Output; fn $op(self, b: &$u) -> Self::Output { $Op::$op(self, *b) } }
	impl $Op<&$u> for &$t { type Output = <$t as $Op<$u>>::Output; fn $op(self, b: &$u) -> Self::Output { $Op::$op(*self, *b) } }
}}

#[cfg(feature="int_roundings")] use core::ops::{Mul,Div};
#[cfg(feature="int_roundings")] impl Mul<uint2> for Ratio { type Output=uint2; #[track_caller] fn mul(self, b: uint2) -> Self::Output { ceil(self, b) } }
#[cfg(feature="int_roundings")] forward_ref_binop!{Mul, mul, uint2, Ratio}
#[cfg(feature="int_roundings")] impl Mul<int2> for Ratio { type Output=int2; #[track_caller] fn mul(self, b: int2) -> Self::Output { ifloor(self, b) } }
#[cfg(feature="int_roundings")] forward_ref_binop!{Mul, mul, int2, Ratio}
#[cfg(feature="int_roundings")] impl Div<Ratio> for uint2 { type Output=uint2; fn div(self, r: Ratio) -> Self::Output { xy{x:self.x/r, y:self.y/r} } }
#[cfg(feature="int_roundings")] impl Div<Ratio> for int2 { type Output=int2; fn div(self, r: Ratio) -> Self::Output { xy{x:self.x/r, y:self.y/r} } }
#[cfg(feature="int_roundings")] forward_ref_binop!{Div, div, Ratio, uint2}

use crate::MinMax;
impl MinMax<uint2> {
	pub fn signed(self) -> MinMax<int2> { self.map(|p| p.signed()) }
	pub fn area(self) -> u32 { self.signed().area() }
}
impl MinMax<int2> {
	pub fn try_unsigned(self) -> Option<MinMax<uint2>, > { self.try_map(|p| p.try_unsigned()) }
	#[track_caller] pub fn unsigned(self) -> MinMax<uint2> { self.try_unsigned().unwrap() }
	pub fn area(self) -> u32 { let xy{x,y} = self.size().unsigned(); x*y }
	pub fn extend(self, pad: u32) -> MinMax<int2> { let MinMax{min,max}=self; MinMax{min: min-xy::from(pad as i32), max: max+xy::from(pad as i32)} }
}
pub type Rect = MinMax<int2>;

use core::ops::{Add,Sub};
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
}
pub use mod_xyz::*;

vector!(4 xyzw T T T T, x y z w, X Y Z W);
#[allow(non_camel_case_types)] pub type vec4 = xyzw<f32>;
#[allow(non_camel_case_types)] pub type mat4 = xyzw<vec4>;

use core::array::from_fn as eval;
pub fn transpose<T: Copy, const M: usize, const N:usize>(m: [[T; N]; M]) -> [[T; M]; N] { eval(|i| eval(|j| m[j][i])) }
#[allow(non_camel_case_types)] type Matrix<const M: usize, const N:usize> = [[f32; N]; M];
pub fn mul<const M: usize, const N:usize, const P:usize>(a: Matrix<M,N>, b: Matrix<N,P>) -> Matrix<M,P> { eval(|i| eval(|j| (0..N).map(|k| a[i][k]*b[k][j]).sum())) }
pub fn mulv<const M: usize, const N:usize>(a: Matrix<M,N>, b: [f32; N]) -> [f32; M] { mul(a, b.map(|k| [k])).map(|[k]| k) }
//pub fn mulv<const M: usize, const N:usize>(a: Matrix<M,N>, b: [f32; N]) -> [f32; M] { eval(|i| (0..N).map(|k| a[i][k]*b[k]).sum()) }
pub fn mul1<const M: usize, const N:usize, const P:usize>(a: f32, b: Matrix<N,P>) -> Matrix<M,P> { eval(|i| eval(|j| a*b[i][j])) }

pub fn diagonal<const N: usize>(diagonal: [f32; N]) -> [[f32; N]; N] { eval(|i| eval(|j| if i==j { diagonal[i]  } else { 0. })) }

#[cfg(feature="generic_const_exprs")] fn minor<const N:usize>(m: [[f32; N]; N], i: usize, j: usize) -> f32 where [[f32; N-1]; N-1]:Det {
	fn from_iter<T, const N: usize>(mut iter: impl Iterator<Item=T>) -> [T; N] { let a = [(); N].map(|_| iter.next().unwrap()); assert!(iter.next().is_none()); a }
	det(from_iter(m.into_iter().enumerate().filter(|&(row,_)| row!=i).map(|(_,row)| from_iter(row.into_iter().enumerate().filter(|&(column,_)| column!=j).map(|(_,m)| m)))))
}
#[cfg(feature="generic_const_exprs")] fn cofactor<const N:usize>(m: [[f32; N]; N], i: usize, j: usize) -> f32 where [[f32; N-1]; N-1]:Det { (match (i+j)%2 { 0=>1., 1=>-1., _=>unreachable!()})*minor(m,i,j) }
#[cfg(feature="generic_const_exprs")] fn det_n<const N:usize>(m: [[f32; N]; N]) -> f32 where [[f32; N-1]; N-1]:Det { (0..N).map(|j| m[0][j]*cofactor(m, 0,j)).sum() }
#[cfg(feature="generic_const_exprs")] trait Det { fn det(self) -> f32; }
#[cfg(feature="generic_const_exprs")] impl Det for [[f32; 3]; 3] { fn det(self) -> f32 { det_n(self) }}
#[cfg(feature="generic_const_exprs")] impl Det for [[f32; 2]; 2] { fn det(self) -> f32 { det_n(self) }}
#[cfg(feature="generic_const_exprs")] impl Det for [[f32; 1]; 1] { fn det(self) -> f32 { self[0][0] }}
#[cfg(feature="generic_const_exprs")] fn det<T: Det>(t: T) -> f32 { t.det() }
#[cfg(feature="generic_const_exprs")] fn adjugate<const N:usize>(m: [[f32; N]; N]) -> [[f32; N]; N] where [[f32; N-1]; N-1]:Det { eval(|i| eval(|j| cofactor(m, j,i)) ) }
#[cfg(feature="generic_const_exprs")] #[allow(private_bounds)] pub fn inverse<const N:usize>(m: [[f32; N]; N]) -> [[f32; N]; N] where [[f32; N]; N]:Det, [[f32; N-1]; N-1]:Det { mul1(1./det(m), adjugate(m)) }

#[allow(non_camel_case_types)] pub type mat3 = Matrix<3,3>;
