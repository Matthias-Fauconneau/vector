use std::{ops::{Mul,Div,Sub}, iter::Sum};
pub fn dot<T:Mul>(a: T, b: T) -> <T::Output as IntoIterator>::Item where T::Output: IntoIterator<Item: Sum> { (a*b).into_iter().sum() }
pub fn sq<T:Mul+Copy>(v: T) -> <T::Output as IntoIterator>::Item where T::Output: IntoIterator<Item: Sum> { dot(v, v) }
pub fn norm<T:Mul+Copy>(v: T) -> <T::Output as IntoIterator>::Item where T::Output: IntoIterator<Item: Sum+num::Sqrt> { num::Sqrt::sqrt(sq(v)) }
pub fn normalize<T:Mul+Copy+ Div<<<T as Mul>::Output as IntoIterator>::Item> >(v: T) -> <T as Div<<<T as Mul>::Output as IntoIterator>::Item> >::Output 
    where <T as Mul>::Output: IntoIterator<Item: Sum+num::Sqrt> 
{
        v/norm(v) 
}
pub fn distance<T:Sub>(a: T, b: T) -> <<<T as Sub>::Output as Mul>::Output as IntoIterator>::Item where T::Output: Mul+Copy, <<T as Sub>::Output as Mul>::Output: IntoIterator<Item: Sum+num::Sqrt> { norm(b-a) }