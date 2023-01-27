#![feature(associated_type_bounds, const_trait_impl, int_roundings, generic_arg_infer, array_zip)]

use std::{ops::{Mul,Div}, iter::Sum};
pub fn dot<T:Mul>(a: T, b: T) -> <T::Output as IntoIterator>::Item where T::Output: IntoIterator<Item: Sum> { (a*b).into_iter().sum() }
pub fn sq<T:Mul+Copy>(v: T) -> <T::Output as IntoIterator>::Item where T::Output: IntoIterator<Item: Sum> { dot(v, v) }
pub fn norm<T:Mul+Copy>(v: T) -> <T::Output as IntoIterator>::Item where T::Output: IntoIterator<Item: Sum+num::Sqrt> { num::Sqrt::sqrt(sq(v)) }
pub fn normalize<T:Mul+Copy+Div<<<T as Mul>::Output as IntoIterator>::Item>>(v: T) -> <T as Div<<<T as Mul>::Output as IntoIterator>::Item>>::Output where <T as Mul>::Output: IntoIterator<Item: Sum+num::Sqrt> { v/norm(v) }

pub trait ComponentWiseMinMax {
	fn component_wise_min(self, other: Self) -> Self;
	fn component_wise_max(self, other: Self) -> Self;
}
pub fn component_wise_min<T: ComponentWiseMinMax>(a: T, b: T) -> T { a.component_wise_min(b) }
pub fn component_wise_max<T: ComponentWiseMinMax>(a: T, b: T) -> T { a.component_wise_max(b) }

// /!\ not defining ComponentWiseMinMax implementation for any Ord to exclude types which might simultaneously have components (i.e vectors) but implement Ord
macro_rules! impl_ComponentWiseMinMax {
	($($T:ident)+) => {$(
		impl ComponentWiseMinMax for $T {
			fn component_wise_min(self, other: Self) -> Self { self.min(other) }
			fn component_wise_max(self, other: Self) -> Self { self.max(other) }
		}
	)+};
}
impl_ComponentWiseMinMax!{u8 i8 u16 i16 u32 i32 f32 u64 i64 f64}
pub fn min<T: ComponentWiseMinMax+Copy>(iter: impl IntoIterator<Item=T>) -> Option<T> { iter.into_iter().reduce(ComponentWiseMinMax::component_wise_min) }
pub fn max<T: ComponentWiseMinMax+Copy>(iter: impl IntoIterator<Item=T>) -> Option<T> { iter.into_iter().reduce(ComponentWiseMinMax::component_wise_max) }

#[derive(PartialEq,Eq,Clone,Copy,Debug)] pub struct MinMax<T> { pub min: T, pub max: T }
impl<T:num::Zero> num::Zero for MinMax<T> { const ZERO: Self = MinMax{min: T::ZERO, max: T::ZERO}; }
impl<T> From<MinMax<T>> for std::ops::Range<T> { fn from(MinMax{min,max}: MinMax<T>) -> Self { min .. max }}
impl<T:ComponentWiseMinMax> MinMax<T> {
	pub fn minmax(self, Self{min, max}: Self) -> Self { Self{min: component_wise_min(self.min, min), max: component_wise_max(self.max, max)} }
}
pub fn reduce_minmax<T: ComponentWiseMinMax>(iter: impl IntoIterator<Item=MinMax<T>>) -> Option<MinMax<T>> { iter.into_iter().reduce(|a,b| a.minmax(b)) }
pub fn minmax<T: ComponentWiseMinMax+Copy>(iter: impl IntoIterator<Item=T>) -> Option<MinMax<T>> { reduce_minmax(iter.into_iter().map(|x| MinMax{min: x, max: x})) }

impl<T:std::ops::AddAssign+Copy> MinMax<T> { pub fn translate(&mut self, offset: T) { self.min += offset; self.max += offset; } }
impl<T:ComponentWiseMinMax+Copy> MinMax<T> {
	pub fn clip(self, b: Self) -> Self { Self{
		min: component_wise_min(self.max, component_wise_max(self.min, b.min)),
		max: component_wise_max(self.min, component_wise_min(self.max, b.max))
	} }
}
impl MinMax<vec2> { pub fn size(&self) -> vec2 { self.max-self.min } }

pub trait Lerp<T> { fn lerp(&self, a: T, b: T) -> T; }

#[macro_export] macro_rules! forward_ref_binop {{impl $Op:ident, $op:ident for $t:ty, $u:ty} => {
	impl<'t, T:$Op+Copy> std::ops::$Op<$u> for &'t $t { type Output = <$t as std::ops::$Op<$u>>::Output; fn $op(self, b: $u) -> Self::Output { std::ops::$Op::$op(*self, b) } }
	impl<T:$Op+Copy> std::ops::$Op<&$u> for $t { type Output = <$t as std::ops::$Op<$u>>::Output; fn $op(self, b: &$u) -> Self::Output { std::ops::$Op::$op(self, *b) } }
	impl<T:$Op+Copy> std::ops::$Op<&$u> for &$t { type Output = <$t as std::ops::$Op<$u>>::Output; fn $op(self, b: &$u) -> Self::Output { std::ops::$Op::$op(*self, *b) } }
}}

#[macro_export] macro_rules! impl_Op { { $Vector:ident $($c:ident)+: $Op:ident $op:ident $OpAssign:ident $op_assign:ident } => {
	impl<T:$Op> $Op for $Vector<T> { type Output=$Vector<T::Output>; fn $op(self, b: Self) -> Self::Output { Self::Output{$($c: self.$c.$op(b.$c)),+} } }
	$crate::forward_ref_binop!{ impl $Op, $op for $Vector<T>, $Vector<T> }
	impl<T:$OpAssign> $OpAssign for $Vector<T> { fn $op_assign(&mut self, b: Self) { $(self.$c.$op_assign(b.$c);)+ } }
}}

pub extern crate num;
pub extern crate bytemuck;

#[macro_export] macro_rules! vector {
($N:literal $Vector:ident $($tuple:ident)+, $($c:ident)+, $($C:ident)+) => {
mod mod_vector {
use std::ops::{Add,Sub,Mul,Div,AddAssign,SubAssign,MulAssign,DivAssign};
#[allow(non_camel_case_types)]
#[repr(C)] #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)] pub struct $Vector<T> { $( pub $c: T ),+ }
//impl<T: Into<U>, U> From<$Vector<T>> for $Vector<U> { fn from(v: $Vector<T>) -> Self { $Vector{$($c:v.$c.into()),+} } } // conflicts with impl<T> From<T> for T
impl From<$Vector<u16>> for $Vector<u32> { fn from(v: $Vector<u16>) -> Self { $Vector{$($c:v.$c.into()),+} } }
impl From<$Vector<u16>> for $Vector<f32> { fn from(v: $Vector<u16>) -> Self { $Vector{$($c:v.$c.into()),+} } }
impl<T> From<$Vector<T>> for [T; $N] { fn from(v : $Vector<T>) -> Self { [$(v.$c),+] } }
impl<T> From<[T; $N]> for $Vector<T> { fn from(a: [T; $N]) -> Self { let [$($c),+] = a; $Vector{$($c),+} } }
impl<T> From<($($tuple),+)> for $Vector<T> { fn from(($($c),+): ($($tuple),+)) -> Self { $Vector{$($c),+} } }
impl<T> From<$Vector<T>> for ($($tuple),+) { fn from(v : $Vector<T>) -> Self { ($(v.$c),+) } }
impl<T:Copy> const From<T> for $Vector<T> { fn from(v: T) -> Self { $Vector{$($c:v),+} } }
impl<T:$crate::num::Zero> $crate::num::Zero for $Vector<T> { const ZERO : Self = $Vector{$($c: T::ZERO),+}; }
unsafe impl<T: $crate::bytemuck::Zeroable> $crate::bytemuck::Zeroable for $Vector<T> {}
unsafe impl<T: $crate::bytemuck::Pod> $crate::bytemuck::Pod for $Vector<T> {}

impl<T> $Vector<T> {
	pub fn map<U>(self, mut f: impl FnMut(T)->U) -> $Vector<U> { <[_; _]>::from(self).map(|c| f(c)).into() }
	pub fn zip<B>(self, b: $Vector<B>) -> $Vector<(T, B)> { <[_; _]>::from(self).zip(b.into()).into() }
	pub fn each_ref(&self) -> [&T; $N] { [$(&self.$c),+] }
	//pub fn each_mut(&mut self) -> [&mut T; $N] { [$(&mut self.$c),+] }
	pub fn iter(&self) -> std::array::IntoIter<&T, $N> { self.each_ref().into_iter() }
	/*pub fn iter_mut(&mut self) -> std::array::IntoIter<&mut T, $N> { self.each_mut().into_iter() }
	//pub fn map<U>(&self, mut f: impl FnMut(&T)->U) -> $Vector<U> { self.each_ref().map(|c| f(c)) }
	pub fn map_mut<U>(&mut self, mut f: impl FnMut(&mut T)->U) -> $Vector<U> { self.each_mut().map(|c| f(c)) }*/
	//pub fn zip<B>(self, b: $Vector<B>) -> $Vector<(T, B)> { self.each_ref().zip(b.each_ref()) }
}

impl<T> IntoIterator for $Vector<T> {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, $N>;
    fn into_iter(self) -> Self::IntoIter { Into::<[T; $N]>::into(self).into_iter() }
}
impl<'t, T> IntoIterator for &'t $Vector<T> {
    type Item = &'t T;
    type IntoIter = std::array::IntoIter<Self::Item, $N>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
/*impl<T> std::iter::FromIterator<T> for $Vector<T> { fn from_iter<I:IntoIterator<Item=T>>(into_iter: I) -> Self {
	let mut iter = into_iter.into_iter();
	$Vector{$($c: iter.next().unwrap()),+}
} }*/

/*#[derive(Clone, Copy)] pub enum Component { $($C),+ }
impl Component { pub fn enumerate() -> [Self; $N] { [$(Self::$C),+] } }
impl<T> $Vector<T> { pub fn enumerate() -> [Component; $N] { Component::enumerate() } }
impl<T> std::ops::Index<Component> for $Vector<T> {
    type Output = T;
    fn index(&self, component: Component) -> &Self::Output {
        match component {
            $(Component::$C => &self.$c),+
        }
    }
}*/

impl<T:Eq> PartialEq<T> for $Vector<T> { fn eq(&self, b: &T) -> bool { self.iter().map(|a| a.eq(b)).reduce(|a,e| a && e).unwrap() } }
impl<T:PartialOrd> PartialOrd for $Vector<T> { fn partial_cmp(&self, b: &Self) -> Option<std::cmp::Ordering> {
	self.into_iter().zip(b).map(|(a,b)| a.partial_cmp(b)).reduce(|a,e| if a == Some(std::cmp::Ordering::Equal) || a == e { e } else { None }).flatten()
} }
impl<T:$crate::ComponentWiseMinMax> $crate::ComponentWiseMinMax for $Vector<T> {
	fn component_wise_min(self, b: Self) -> Self { self.zip(b).map(|(a,b)| a.component_wise_min(b)) }
	fn component_wise_max(self, b: Self) -> Self { self.zip(b).map(|(a,b)| a.component_wise_max(b)) }
}

impl<T:std::ops::Neg> std::ops::Neg for $Vector<T> { type Output=$Vector<T::Output>; fn neg(self) -> Self::Output { Self::Output{$($c: self.$c.neg()),+} } }
$crate::impl_Op!{$Vector $($c)+: Add add AddAssign add_assign}
$crate::impl_Op!{$Vector $($c)+: Sub sub SubAssign sub_assign}
$crate::impl_Op!{$Vector $($c)+: Mul mul MulAssign mul_assign}
$crate::impl_Op!{$Vector $($c)+: Div div DivAssign div_assign}

impl<T:Div+Copy> Div<T> for $Vector<T> { type Output=$Vector<T::Output>; fn div(self, b: T) -> Self::Output { Self::Output{$($c: self.$c/b),+} } }

impl<T:Copy+Mul> $Vector<T> { fn mul(s: T, v: Self) -> $Vector<T::Output> { $Vector{$($c: s*v.$c),+} } }
impl Mul<$Vector<u32>> for u32 { type Output=$Vector<u32>; fn mul(self, v: $Vector<u32>) -> Self::Output { $Vector::mul(self, v) } }
impl Mul<u32> for $Vector<u32> { type Output=$Vector<u32>; fn mul(self, b: u32) -> Self::Output { $Vector::mul(b, self) } }
impl Mul<$Vector<f32>> for f32 { type Output=$Vector<f32>; fn mul(self, v: $Vector<f32>) -> Self::Output { $Vector::mul(self, v) } }
impl Mul<$Vector<f64>> for f64 { type Output=$Vector<f64>; fn mul(self, v: $Vector<f64>) -> Self::Output { $Vector::mul(self, v) } }

impl $crate::Lerp<$Vector<f32>> for f32 { fn lerp(&self, a: $Vector<f32>, b: $Vector<f32>) -> $Vector<f32> { let t = *self; assert!(t >= 0. && t<= 1.); (1.-t)*a + t*b } }

impl<T:Copy+Div> $Vector<T> { fn div(s: T, v: Self) -> $Vector<T::Output> { $Vector{$($c: s/v.$c),+} } }
impl Div<$Vector<u32>> for u32 { type Output=$Vector<u32>; fn div(self, v: $Vector<u32>) -> Self::Output { $Vector::div(self, v) } }
impl Div<$Vector<f32>> for f32 { type Output=$Vector<f32>; fn div(self, v: $Vector<f32>) -> Self::Output { $Vector::div(self, v) } }
}
pub use mod_vector::$Vector;
}
}

#[path="xyz.rs"] mod mod_xyz;
pub use mod_xyz::*;