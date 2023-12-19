use std::ops::{Index,IndexMut,Mul,MulAssign,Div,DivAssign,Sub,SubAssign,Add,AddAssign,Deref,DerefMut};
use std::slice::SliceIndex;                                         
use std::marker::Copy;
use std::fmt::{Display,Debug,Formatter,Result};
#[cfg(feature = "fractions")]
use my_fractions::MyFraction;

#[derive(Debug, Clone, Copy)]
pub struct MyArr<T, const N: usize> {
    pub data: [T; N],
}
impl<T, const N: usize> MyArr<T, N>
where
    T: Debug,
{
    pub fn new(data: [T; N]) -> Self {
        Self { data }
    }
}
//indexing into array
impl<T, const N: usize, Idx> Index<Idx> for MyArr<T, N>
where
    Idx: SliceIndex<[T]>,
{
    type Output = <Idx as SliceIndex<[T]>>::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.data[index]
    }
}
impl<T, const N: usize, Idx> IndexMut<Idx> for MyArr<T, N>
where
    Idx: SliceIndex<[T]>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.data[index]
    }
}

//i64 * MyArr
impl<T, const N: usize> Mul<MyArr<T, N>> for i64
where
    T: Mul<i64, Output = T> + Copy + Debug,
{
    type Output = MyArr<T, N>;

    fn mul(self, rhs: MyArr<T, N>) -> MyArr<T, N> {
        let mut temp = rhs;
        for i in 0..N {
            temp[i] = temp[i] * self;
        }
        temp
    }
}
// MyArr * i64
impl<T, const N: usize> Mul<i64> for MyArr<T, N>
where
    T: Mul<i64, Output = T> + Copy + Debug,
{
    type Output = MyArr<T, N>;

    fn mul(self, rhs: i64) -> MyArr<T, N> {
        let mut temp = self;
        for i in 0..N {
            temp[i] = temp[i] * rhs;
        }
        temp
    }
}
#[cfg(feature = "fractions")]
impl<T, const N: usize> Mul<MyArr<T, N>> for MyFraction
where
    T: Mul<MyFraction, Output = T> + Copy + Debug,
{
    type Output = MyArr<T, N>;

    fn mul(self, rhs: MyArr<T, N>) -> MyArr<T, N> {
        let mut temp = rhs;
        for i in 0..N {
            temp[i] = temp[i] * self;
        }
        temp
    }
}
#[cfg(feature = "fractions")]
impl<T, const N: usize> Mul<MyFraction> for MyArr<T, N>
where
    T: Mul<MyFraction, Output = T> + Copy + Debug,
{
    type Output = MyArr<T, N>;

    fn mul(self, rhs: MyFraction) -> MyArr<T, N> {
        let mut temp = self;
        for i in 0..N {
            temp[i] = temp[i] * rhs;
        }
        temp
    }
}

impl<T, const N: usize> Add<MyArr<T, N>> for MyArr<T, N>
where
    T: Add<T, Output = T> + Copy + Debug,
{
    type Output = MyArr<T, N>;

    fn add(self, rhs: MyArr<T, N>) -> MyArr<T, N> {
        let mut temp = self;
        for i in 0..N {
            temp[i] = self[i] + rhs[i]
        }
        temp
    }
}

impl<T, const N: usize> Sub<MyArr<T, N>> for MyArr<T, N>
where
    T: Sub<T, Output = T> + Copy + Debug,
{
    type Output = MyArr<T, N>;

    fn sub(self, rhs: MyArr<T, N>) -> MyArr<T, N> {
        let mut temp = self;
        for i in 0..N {
            temp[i] = self[i] - rhs[i]
        }
        temp
    }
}
impl<T, const N: usize> Deref for MyArr<T, N> {
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
//i64 / MyArr
impl<T, const N: usize> Div<MyArr<T, N>> for i64
where
    T: Div<i64, Output = T> + Copy + Debug,
{
    type Output = MyArr<T, N>;

    fn div(self, rhs: MyArr<T, N>) -> MyArr<T, N> {
        let mut temp = rhs;
        for i in 0..N {
            temp[i] = temp[i] / self;
        }
        temp
    }
}
impl<T, const N: usize> Div<i64> for MyArr<T, N>
where
    T: Div<i64, Output = T> + Copy + Debug,
{
    type Output = MyArr<T, N>;

    fn div(self, rhs: i64) -> MyArr<T, N> {
        let mut temp = self;
        for i in 0..N {
            temp[i] = temp[i] / rhs;
        }
        temp
    }
}
impl<T, const N: usize> AddAssign for MyArr<T, N>
where
    T: Add<T, Output = T> + Copy + Debug,
{
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self[i] = self[i] + rhs[i];
        }
    }
}
impl<T, const N: usize> SubAssign for MyArr<T, N>
where
    T: Sub<T, Output = T> + Copy + Debug,
{
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self[i] = self[i] - rhs[i];
        }
    }
}
impl<T, const N: usize> DivAssign<i64> for MyArr<T, N>
where
    T: Div<i64, Output = T> + Copy + Debug,
{
    fn div_assign(&mut self, rhs: i64) {
        for i in 0..N {
            self[i] = self[i] / rhs;
        }
    }
}
impl<T, const N: usize> MulAssign<i64> for MyArr<T, N>
where
    T: Mul<i64, Output = T> + Copy + Debug,
{
    fn mul_assign(&mut self, rhs: i64) {
        for i in 0..N {
            self[i] = self[i] * rhs;
        }
    }
}
impl<T, const N: usize> Display for MyArr<T, N>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[ ")?;
        for i in &self.data {
            write!(f, "{}", i)?;
            if !std::ptr::eq(i, self.data.last().unwrap()) {
                write!(f, ", ")?;
            }
        }
        write!(f, " ]")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn modulo(a: i64, b: i64) -> usize {
        (((a % b) + b) % b) as usize
    }


    const HEIGHT: usize = 4;
    const WIDTH: usize = HEIGHT + 1;

    #[test]
    fn it_works() {
    }
}
