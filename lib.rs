pub trait ComponentWiseMinMax {
	fn component_wise_min(self, other: Self) -> Self;
	fn component_wise_max(self, other: Self) -> Self;
}
pub fn component_wise_min<V: ComponentWiseMinMax>(a: V, b: V) -> V { a.component_wise_min(b) }
pub fn component_wise_max<V: ComponentWiseMinMax>(a: V, b: V) -> V { a.component_wise_max(b) }

macro_rules! impl_ComponentWiseMinMax {
	($($T:ident)+) => {$(
		impl ComponentWiseMinMax for $T { // /!\ semantic break if impl Ord for Vector
			fn component_wise_min(self, other: Self) -> Self { self.min(other) }
			fn component_wise_max(self, other: Self) -> Self { self.max(other) }
		}
	)+};
}
impl_ComponentWiseMinMax!{u8 i8 u16 i16 u32 i32 f32 u64 i64 f64}

#[derive(PartialEq, Eq, Clone, Copy, Debug)] pub struct MinMax<T> { pub min: T, pub max: T }
impl<T:ComponentWiseMinMax> MinMax<T> {
	pub fn minmax(self, Self{min, max}: Self) -> Self { Self{min: component_wise_min(self.min, min), max: component_wise_max(self.max, max)} }
}
pub fn minmax<T: ComponentWiseMinMax+Copy>(iter: impl Iterator<Item=T>) -> Option<MinMax<T>> { iter.map(|x| MinMax{min: x, max: x}).reduce(MinMax::minmax) }

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

#[cfg(feature="num")] pub extern crate num;
pub extern crate bytemuck;
#[macro_export] macro_rules! vector { ($N:literal $Vector:ident $($tuple:ident)+, $($c:ident)+, $($C:ident)+) => {
use std::ops::{Add,Sub,Mul,Div,AddAssign,SubAssign,MulAssign,DivAssign};
#[allow(non_camel_case_types)]
#[repr(C)] #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)] pub struct $Vector<T> { $( pub $c: T ),+ }
unsafe impl<T: $crate::bytemuck::Zeroable> $crate::bytemuck::Zeroable for $Vector<T> {}
unsafe impl<T: $crate::bytemuck::Pod> $crate::bytemuck::Pod for $Vector<T> {}

impl<T> From<$Vector<T>> for [T; $N] { fn from(v : $Vector<T>) -> Self { [$(v.$c),+] } }

impl<T> IntoIterator for $Vector<T> {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, $N>;
    fn into_iter(self) -> Self::IntoIter { Into::<[T; $N]>::into(self).into_iter() }
}

impl<T> From<($($tuple),+)> for $Vector<T> { fn from(($($c),+): ($($tuple),+)) -> Self { $Vector{$($c),+} } }
impl<T> From<$Vector<T>> for ($($tuple),+) { fn from(v : $Vector<T>) -> Self { ($(v.$c),+) } }

impl<T> std::iter::FromIterator<T> for $Vector<T> { fn from_iter<I:IntoIterator<Item=T>>(into_iter: I) -> Self {
	let mut iter = into_iter.into_iter();
	$Vector{$($c: iter.next().unwrap()),+}
} }
impl<T> From<[T; $N]> for $Vector<T> { fn from(a: [T; $N]) -> Self { a.into_iter().collect() } }

#[derive(Clone, Copy)] pub enum Component { $($C),+ }
impl Component { pub fn enumerate() -> [Self; $N] { [$(Self::$C),+] } }
impl<T> std::ops::Index<Component> for $Vector<T> {
    type Output = T;
    fn index(&self, component: Component) -> &Self::Output {
        match component {
            $(Component::$C => &self.$c),+
        }
    }
}
type Iter<'t, T> = std::iter::Map<std::array::IntoIter<Component, $N>, impl FnMut(Component) -> &'t T>;
impl<T> $Vector<T> {
	pub fn iter(&self) -> Iter<'_, T> { Component::enumerate().into_iter().map(move |c| &self[c] ) }
	pub fn map<U>(&self, mut f: impl FnMut(&T)->U) -> $Vector<U> { self.iter().map(|c| f(c)).collect() }
}
impl<'t, T> IntoIterator for &'t $Vector<T> {
    type Item = &'t T;
    type IntoIter = Iter<'t, T>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}

impl<T:Eq> PartialEq<T> for $Vector<T> { fn eq(&self, b: &T) -> bool { self.iter().map(|a| a.eq(b)).reduce(|a,e| a && e).unwrap() } }
impl<T:PartialOrd> PartialOrd for $Vector<T> { fn partial_cmp(&self, b: &Self) -> Option<std::cmp::Ordering> {
	self.into_iter().zip(b).map(|(a,b)| a.partial_cmp(b)).reduce(|a,e| if a == Some(std::cmp::Ordering::Equal) || a == e { e } else { None }).flatten()
} }
impl<T:$crate::ComponentWiseMinMax> $crate::ComponentWiseMinMax for $Vector<T> {
	fn component_wise_min(self, b: Self) -> Self { self.into_iter().zip(b).map(|(a,b)| a.component_wise_min(b)).collect() }
	fn component_wise_max(self, b: Self) -> Self { self.into_iter().zip(b).map(|(a,b)| a.component_wise_max(b)).collect() }
}

impl<T:std::ops::Neg> std::ops::Neg for $Vector<T> { type Output=$Vector<T::Output>; fn neg(self) -> Self::Output { Self::Output{$($c: self.$c.neg()),+} } }
$crate::impl_Op!{$Vector $($c)+: Add add AddAssign add_assign}
$crate::impl_Op!{$Vector $($c)+: Sub sub SubAssign sub_assign}
$crate::impl_Op!{$Vector $($c)+: Mul mul MulAssign mul_assign}
$crate::impl_Op!{$Vector $($c)+: Div div DivAssign div_assign}

impl<T:Div+Copy> Div<T> for $Vector<T> { type Output=$Vector<T::Output>; fn div(self, b: T) -> Self::Output { Self::Output{$($c: self.$c/b),+} } }

impl<T:Copy> From<T> for $Vector<T> { fn from(v: T) -> Self { $Vector{$($c:v),+} } }

impl<T:Copy+Mul> $Vector<T> { fn mul(s: T, v: Self) -> $Vector<T::Output> { $Vector{$($c: s*v.$c),+} } }
impl Mul<$Vector<u32>> for u32 { type Output=$Vector<u32>; fn mul(self, v: $Vector<u32>) -> Self::Output { $Vector::mul(self, v) } }
impl Mul<$Vector<f32>> for f32 { type Output=$Vector<f32>; fn mul(self, v: $Vector<f32>) -> Self::Output { $Vector::mul(self, v) } }
impl Mul<$Vector<f64>> for f64 { type Output=$Vector<f64>; fn mul(self, v: $Vector<f64>) -> Self::Output { $Vector::mul(self, v) } }

impl<T:Copy+Div> $Vector<T> { fn div(s: T, v: Self) -> $Vector<T::Output> { $Vector{$($c: s/v.$c),+} } }
impl Div<$Vector<u32>> for u32 { type Output=$Vector<u32>; fn div(self, v: $Vector<u32>) -> Self::Output { $Vector::div(self, v) } }
impl Div<$Vector<f32>> for f32 { type Output=$Vector<f32>; fn div(self, v: $Vector<f32>) -> Self::Output { $Vector::div(self, v) } }

#[cfg(feature="num")] impl<T:Copy+$crate::num::Zero> $crate::num::Zero for $Vector<T> { const ZERO : Self = $Vector{$($c: T::ZERO),+}; }
}}
