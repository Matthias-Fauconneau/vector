#![no_std]
use core::{ops::{Mul,Div,Sub}, iter::Sum};

pub fn dot<T:Mul>(a: T, b: T) -> <T::Output as IntoIterator>::Item where T::Output: IntoIterator<Item: Sum> { (a*b).into_iter().sum() }
pub fn sq<T:Mul+Copy>(v: T) -> <T::Output as IntoIterator>::Item where T::Output: IntoIterator<Item: Sum> { dot(v, v) }
pub fn norm<T:Mul+Copy>(v: T) -> <T::Output as IntoIterator>::Item where T::Output: IntoIterator<Item: Sum+num::Sqrt> { num::Sqrt::sqrt(sq(v)) }

pub fn normalize<T:Mul+Copy+ Div<<<T as Mul>::Output as IntoIterator>::Item> >(v: T) -> <T as Div<<<T as Mul>::Output as IntoIterator>::Item> >::Output
  where <T as Mul>::Output: IntoIterator<Item: Sum+num::Sqrt>
	{ v/norm(v) }

pub fn distance<T:Sub>(a: T, b: T) -> <<<T as Sub>::Output as Mul>::Output as IntoIterator>::Item
	where T::Output: Mul+Copy, <<T as Sub>::Output as Mul>::Output: IntoIterator<Item: Sum+num::Sqrt>
	{ norm(b-a) }

// Yields min/max of each components. By comparison, std::cmp::{min,max}(impl Ord) yields either value completely.
pub trait ComponentWiseMinMax {
	fn component_wise_min(self, other: Self) -> Self;
	fn component_wise_max(self, other: Self) -> Self;
}
pub fn component_wise_min<T: ComponentWiseMinMax>(a: T, b: T) -> T { a.component_wise_min(b) }
pub fn component_wise_max<T: ComponentWiseMinMax>(a: T, b: T) -> T { a.component_wise_max(b) }

// /!\ cannot impl ComponentWiseMinMax for Ord since some types (i.e vectors) simultaneously have components but implement Ord
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
impl<T:core::fmt::Display> core::fmt::Display for MinMax<T> {  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { write!(f, "{}x{}", self.min, self.max) } }
impl<T:num::Zero> num::Zero for MinMax<T> { const ZERO: Self = MinMax{min: T::ZERO, max: T::ZERO}; }
impl<T> From<MinMax<T>> for core::ops::Range<T> { fn from(MinMax{min,max}: MinMax<T>) -> Self { min .. max }}
impl<T:ComponentWiseMinMax> MinMax<T> {
	pub fn minmax(self, Self{min, max}: Self) -> Self { Self{min: component_wise_min(self.min, min), max: component_wise_max(self.max, max)} }
}
pub fn reduce_minmax<T: ComponentWiseMinMax>(iter: impl IntoIterator<Item=MinMax<T>>) -> Option<MinMax<T>> { iter.into_iter().reduce(MinMax::minmax) }
pub fn minmax<T: ComponentWiseMinMax+Copy>(iter: impl IntoIterator<Item=T>) -> Option<MinMax<T>> { reduce_minmax(iter.into_iter().map(|x| MinMax{min: x, max: x})) }
impl<T:ComponentWiseMinMax+Copy> MinMax<T> {
	pub fn clip(self, b: Self) -> Self { Self{
		min: component_wise_min(self.max, component_wise_max(self.min, b.min)),
		max: component_wise_max(self.min, component_wise_min(self.max, b.max))
	} }
}
impl<T:ComponentWiseMinMax+Copy+PartialEq> MinMax<T> {
	pub fn contains(&self, p: T) -> bool { component_wise_min(self.min, p) == self.min && component_wise_max(self.max, p) == self.max }
}
impl<T:core::ops::AddAssign+Copy> MinMax<T> { pub fn translate(&mut self, offset: T) { self.min += offset; self.max += offset; } }
impl<T:core::ops::Sub> MinMax<T> { pub fn size(self) -> T::Output { self.max-self.min } }
impl<T> MinMax<T> {
	pub fn try_map<U>(self, mut f: impl FnMut(T)->Option<U>) -> Option<MinMax<U>> { let MinMax{min,max}=self; Some(MinMax{min: f(min)?, max: f(max)?}) }
	pub fn map<U>(self, mut f: impl FnMut(T)->U) -> MinMax<U> { let MinMax{min,max}=self; MinMax{min: f(min), max: f(max)} }
}

#[macro_export] macro_rules! forward_ref_binop {{impl $Op:ident, $op:ident for $t:ty, $u:ty} => {
	impl<'t, T:$Op+Copy> core::ops::$Op<$u> for &'t $t { type Output = <$t as core::ops::$Op<$u>>::Output; fn $op(self, b: $u) -> Self::Output { core::ops::$Op::$op(*self, b) } }
	impl<T:$Op+Copy> core::ops::$Op<&$u> for $t { type Output = <$t as core::ops::$Op<$u>>::Output; fn $op(self, b: &$u) -> Self::Output { core::ops::$Op::$op(self, *b) } }
	impl<T:$Op+Copy> core::ops::$Op<&$u> for &$t { type Output = <$t as core::ops::$Op<$u>>::Output; fn $op(self, b: &$u) -> Self::Output { core::ops::$Op::$op(*self, *b) } }
}}

#[macro_export] macro_rules! impl_Op { { $Vector:ident $($c:ident)+: $Op:ident $op:ident $OpAssign:ident $op_assign:ident } => {
	impl<T:$Op> $Op for $Vector<T> { type Output=$Vector<T::Output>; fn $op(self, b: Self) -> Self::Output { Self::Output{$($c: self.$c.$op(b.$c)),+} } }
	$crate::forward_ref_binop!{ impl $Op, $op for $Vector<T>, $Vector<T> }
	impl<T:$OpAssign> $OpAssign for $Vector<T> { fn $op_assign(&mut self, b: Self) { $(self.$c.$op_assign(b.$c);)+ } }
	impl<T:$OpAssign+Copy> $OpAssign<T> for $Vector<T> { fn $op_assign(&mut self, b: T) { $(self.$c.$op_assign(b);)+ } }
}}

pub extern crate num;
pub extern crate bytemuck;
#[cfg(feature="serde")] pub extern crate serde;

#[macro_export] macro_rules! vector {
($N:literal $Vector:ident $($tuple:ident)+, $($c:ident)+, $($C:ident)+) => {
mod mod_vector {
use core::ops::{Add,Sub,Mul,Div,AddAssign,SubAssign,MulAssign,DivAssign};
#[allow(non_camel_case_types)]
#[repr(C)] #[derive(Clone, Copy, Debug, PartialEq, Eq, Default, core::hash::Hash)]
//#[cfg_attr(feature="serde", derive($crate::serde::Serialize, $crate::serde::Deserialize))]
pub struct $Vector<T> { $( pub $c: T ),+ }
//impl<T: Into<U>, U> From<$Vector<T>> for $Vector<U> { fn from(v: $Vector<T>) -> Self { $Vector{$($c:v.$c.into()),+} } } // conflicts with impl<T> From<T> for T
impl From<$Vector<u8>> for $Vector<u16> { fn from(v: $Vector<u8>) -> Self { $Vector{$($c:v.$c.into()),+} } }
impl From<$Vector<u16>> for $Vector<u8> { fn from(v: $Vector<u16>) -> Self { $Vector{$($c:v.$c.try_into().unwrap()),+} } }
impl From<$Vector<u16>> for $Vector<u32> { fn from(v: $Vector<u16>) -> Self { $Vector{$($c:v.$c.into()),+} } }
impl From<$Vector<u32>> for $Vector<f32> { fn from(v: $Vector<u32>) -> Self { $Vector{$($c:v.$c as f32),+} } }
impl From<$Vector<u16>> for $Vector<f32> { fn from(v: $Vector<u16>) -> Self { $Vector{$($c:v.$c.into()),+} } }
impl From<$Vector<u8>> for $Vector<f32> { fn from(v: $Vector<u8>) -> Self { $Vector{$($c:v.$c.into()),+} } }
impl From<$Vector<f32>> for $Vector<u32> { fn from(v: $Vector<f32>) -> Self { $Vector{$($c:unsafe{v.$c.to_int_unchecked()}),+} } }
impl From<$Vector<f32>> for $Vector<u8> { fn from(v: $Vector<f32>) -> Self { $Vector{$($c:unsafe{v.$c.to_int_unchecked()}),+} } }
impl From<$Vector<f32>> for $Vector<f64> { fn from(v: $Vector<f32>) -> Self { $Vector{$($c:v.$c as f64),+} } }
impl From<$Vector<f64>> for $Vector<f32> { fn from(v: $Vector<f64>) -> Self { $Vector{$($c:v.$c as f32),+} } }
impl<T> From<$Vector<T>> for [T; $N] { fn from(v : $Vector<T>) -> Self { [$(v.$c),+] } }
impl<T> From<[T; $N]> for $Vector<T> { fn from(a: [T; $N]) -> Self { let [$($c),+] = a; $Vector{$($c),+} } }
impl<T> From<($($tuple),+)> for $Vector<T> { fn from(($($c),+): ($($tuple),+)) -> Self { $Vector{$($c),+} } }
impl<T> From<$Vector<T>> for ($($tuple),+) { fn from(v : $Vector<T>) -> Self { ($(v.$c),+) } }
impl<T:Copy> /*const*/ From<T> for $Vector<T> { fn from(v: T) -> Self { $Vector{$($c:v),+} } }
impl<T:$crate::num::Zero> $crate::num::Zero for $Vector<T> { const ZERO : Self = $Vector{$($c: T::ZERO),+}; }
unsafe impl<T: $crate::bytemuck::Zeroable> $crate::bytemuck::Zeroable for $Vector<T> {}
unsafe impl<T: $crate::bytemuck::Pod> $crate::bytemuck::Pod for $Vector<T> {}

impl<T> $Vector<T> {
	//#[cfg(feature="generic_arg_infer")] pub fn map<U>(self, mut f: impl FnMut(T)->U) -> $Vector<U> { <[_; _]>::from(self).map(|c| f(c)).into() }
	pub fn map<U>(self, mut f: impl FnMut(T)->U) -> $Vector<U> { <[T; $N]>::from(self).map(|c| f(c)).into() }
	pub fn zip<B>(self, b: $Vector<B>) -> impl Iterator<Item=(T, B)> { self.into_iter().zip(b.into_iter()) }
	pub fn each_ref(&self) -> [&T; $N] { [$(&self.$c),+] }
	pub fn each_mut(&mut self) -> [&mut T; $N] { [$(&mut self.$c),+] }
	pub fn iter(&self) -> core::array::IntoIter<&T, $N> { self.each_ref().into_iter() }
	pub fn map_mut<U>(&mut self, mut f: impl FnMut(&mut T)->U) -> $Vector<U> { self.each_mut().map(|c| f(c)).into() }
}

impl<T> IntoIterator for $Vector<T> {
    type Item = T;
    type IntoIter = core::array::IntoIter<T, $N>;
    fn into_iter(self) -> Self::IntoIter { Into::<[T; $N]>::into(self).into_iter() }
}
impl<'t, T> IntoIterator for &'t $Vector<T> {
    type Item = &'t T;
    type IntoIter = core::array::IntoIter<Self::Item, $N>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl<T> core::iter::FromIterator<T> for $Vector<T> { fn from_iter<I:IntoIterator<Item=T>>(into_iter: I) -> Self {
	let mut iter = into_iter.into_iter();
	let v = $Vector{$($c: iter.next().unwrap()),+};
	assert!(iter.next().is_none());
	v
} }

#[derive(Clone, Copy)] pub enum Component { $($C),+ }
impl Component { pub fn enumerate() -> [Self; $N] { [$(Self::$C),+] } }
impl<T> $Vector<T> { pub fn enumerate() -> [Component; $N] { Component::enumerate() } }
impl<T> core::ops::Index<Component> for $Vector<T> {
    type Output = T;
    fn index(&self, component: Component) -> &Self::Output {
        match component {
            $(Component::$C => &self.$c),+
        }
    }
}

impl<T:Eq> PartialEq<T> for $Vector<T> { fn eq(&self, b: &T) -> bool { self.iter().map(|a| a.eq(b)).reduce(|a,e| a && e).unwrap() } }
impl<T:PartialOrd> PartialOrd for $Vector<T> { fn partial_cmp(&self, b: &Self) -> Option<core::cmp::Ordering> {
	self.into_iter().zip(b).map(|(a,b)| a.partial_cmp(b)).reduce(|a,e| if a == Some(core::cmp::Ordering::Equal) || a == e { e } else { None }).flatten()
} }
impl<T:$crate::ComponentWiseMinMax> $crate::ComponentWiseMinMax for $Vector<T> {
	fn component_wise_min(self, b: Self) -> Self { self.zip(b).map(|(a,b)| a.component_wise_min(b)).collect() }
	fn component_wise_max(self, b: Self) -> Self { self.zip(b).map(|(a,b)| a.component_wise_max(b)).collect() }
}

impl<T:core::ops::Neg> core::ops::Neg for $Vector<T> { type Output=$Vector<T::Output>; fn neg(self) -> Self::Output { Self::Output{$($c: self.$c.neg()),+} } }
$crate::impl_Op!{$Vector $($c)+: Add add AddAssign add_assign}
$crate::impl_Op!{$Vector $($c)+: Sub sub SubAssign sub_assign}
$crate::impl_Op!{$Vector $($c)+: Mul mul MulAssign mul_assign}
$crate::impl_Op!{$Vector $($c)+: Div div DivAssign div_assign}

impl<T:Div+Copy> Div<T> for $Vector<T> { type Output=$Vector<T::Output>; fn div(self, b: T) -> Self::Output { Self::Output{$($c: self.$c/b),+} } }

//impl<T:Add> core::iter::Sum<$Vector<T>> for $Vector<T> where Self:num::Zero+Add { fn sum<I:Iterator<Item=A>>(iter: I) -> Self { iter.fold(<Self as num::Zero>::ZERO, core::ops::Add::add) } }
impl core::iter::Sum<$Vector<f32>> for $Vector<f32> { fn sum<I:Iterator<Item=$Vector<f32>>>(iter: I) -> Self { iter.fold(<Self as num::Zero>::ZERO, core::ops::Add::add) } }
impl core::iter::Sum<$Vector<f64>> for $Vector<f64> { fn sum<I:Iterator<Item=$Vector<f64>>>(iter: I) -> Self { iter.fold(<Self as num::Zero>::ZERO, core::ops::Add::add) } }

impl<T:Copy+Mul> $Vector<T> { fn mul(s: T, v: Self) -> $Vector<T::Output> { $Vector{$($c: s*v.$c),+} } }
impl Mul<$Vector<u8>> for u8 { type Output=$Vector<u8>; fn mul(self, v: $Vector<u8>) -> Self::Output { $Vector::mul(self, v) } }
impl Mul<$Vector<u16>> for u16 { type Output=$Vector<u16>; fn mul(self, v: $Vector<u16>) -> Self::Output { $Vector::mul(self, v) } }
impl Mul<$Vector<u32>> for u32 { type Output=$Vector<u32>; fn mul(self, v: $Vector<u32>) -> Self::Output { $Vector::mul(self, v) } }
impl Mul<u32> for $Vector<u32> { type Output=$Vector<u32>; fn mul(self, b: u32) -> Self::Output { $Vector::mul(b, self) } }
impl Mul<$Vector<f32>> for f32 { type Output=$Vector<f32>; fn mul(self, v: $Vector<f32>) -> Self::Output { $Vector::mul(self, v) } }
impl Mul<$Vector<f64>> for f64 { type Output=$Vector<f64>; fn mul(self, v: $Vector<f64>) -> Self::Output { $Vector::mul(self, v) } }

impl<T> num::Lerp<$Vector<T>> for f32 where f32: num::Lerp<T> { fn lerp(&self, a: $Vector<T>, b: $Vector<T>) -> $Vector<T> {
	a.zip(b).map(|(a,b)| self.lerp(a,b)).collect()
} }

impl<T:Copy+Div> $Vector<T> { fn div(s: T, v: Self) -> $Vector<T::Output> { $Vector{$($c: s/v.$c),+} } }
impl Div<$Vector<u32>> for u32 { type Output=$Vector<u32>; fn div(self, v: $Vector<u32>) -> Self::Output { $Vector::div(self, v) } }
impl Div<$Vector<f32>> for f32 { type Output=$Vector<f32>; fn div(self, v: $Vector<f32>) -> Self::Output { $Vector::div(self, v) } }
impl Div<$Vector<f64>> for f64 { type Output=$Vector<f64>; fn div(self, v: $Vector<f64>) -> Self::Output { $Vector::div(self, v) } }

impl<T> $Vector<Option<T>> {
	pub fn transpose(self) -> Option<$Vector<T>> { Some($Vector{$($c: self.$c?),+}) }
	//pub fn unwrap_or_else(self, f: impl Fn()->T+Copy) -> $Vector<T> { self.map(move |x| x.unwrap_or_else(f)) }
}
}
pub use mod_vector::$Vector;
}
}

#[path="xyz.rs"] mod mod_xyz;
pub use mod_xyz::*;
