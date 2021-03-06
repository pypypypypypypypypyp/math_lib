use crate::prelude::*;

use crate::vec2::*;
use crate::vec3::*;
use crate::mat2::*;
use crate::mat4::*;

#[repr(C)]
#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash,Serialize,Deserialize)]
pub struct Mat3<T> {
	pub x: Vec3<T>,
	pub y: Vec3<T>,
	pub z: Vec3<T>,
}

impl<T> Mat3<T> {
	pub fn det(self) -> T
		where T: Copy + Mul<Output=T> + Add<Output=T> + Sub<Output=T> {
		let Self{ x,y,z } = self;
		  x.x * y.y * z.z
		+ x.y * y.z * z.x
		+ x.z * y.x * z.y
		- x.x * y.z * z.y
		- x.y * y.x * z.z
		- x.z * y.y * z.x
	}
	
	pub fn cofactor(self) -> Self
		where T: Copy + Mul<Output=T> + Add<Output=T> + Sub<Output=T> + Div<Output=T> + Neg<Output=T> {
		let Mat3{ x,y,z } = self;
		mat3(
			vec3(
				mat2(vec2(y.y,y.z),vec2(z.y,z.z)).det(),
				-mat2(vec2(y.x,y.z),vec2(z.x,z.z)).det(),
				mat2(vec2(y.x,y.y),vec2(z.x,z.y)).det()),
			vec3(
				-mat2(vec2(x.y,x.z),vec2(z.y,z.z)).det(),
				mat2(vec2(x.x,x.z),vec2(z.x,z.z)).det(),
				-mat2(vec2(x.x,x.y),vec2(z.x,z.y)).det()),
			vec3(
				mat2(vec2(x.y,x.z),vec2(y.y,y.z)).det(),
				-mat2(vec2(x.x,x.z),vec2(y.x,y.z)).det(),
				mat2(vec2(x.x,x.y),vec2(y.x,y.y)).det()),
		)
	}
	
	pub fn adjoint(self) -> Self
		where T: Copy + Mul<Output=T> + Add<Output=T> + Sub<Output=T> + Div<Output=T> + Neg<Output=T> {
		self.cofactor().transpose()
	}
	
	pub fn inv(self) -> Self
		where T: Copy + Mul<Output=T> + Add<Output=T> + Sub<Output=T> + Div<Output=T> + Neg<Output=T> {
		let Mat3{ x,y,z } = self.adjoint();
		mat3(x/self.det(),y/self.det(),z/self.det())
	}
	
	pub fn ident() -> Self
		where T: Zero + One {
		mat3(
			vec3(T::one() , T::zero(), T::zero()),
			vec3(T::zero(), T::one() , T::zero()),
			vec3(T::zero(), T::zero(), T::one() ),
		)
	}
	
	pub fn apply_to(self, v: Vec3<T>) -> Vec3<T>
		where Vec3<T>: Vector<T> + Copy {
		vec3(
			dot(self.x, v),
			dot(self.y, v),
			dot(self.z, v),
		)
	}

	pub fn extend(self, right: Vec3<T>, bottom: Vec3<T>, corner: T) -> Mat4<T> {
		mat4(
			self.x.extend(right.x),
			self.y.extend(right.y),
			self.z.extend(right.z),
			bottom.extend(corner),
		)
	}

	pub fn transpose(self) -> Self {
		mat3(
			vec3(self.x.x, self.y.x, self.z.x),
			vec3(self.x.y, self.y.y, self.z.y),
			vec3(self.x.z, self.y.z, self.z.z),
		)
	}
	
	pub fn rotate_x(angle: T) -> Self
		where T: Copy + Trig + Neg<Output=T> + Zero + One {
		mat3(
			vec3(T::one(), T::zero(), T::zero()),
			vec3(T::zero(), angle.cos(), -angle.sin()),
			vec3(T::zero(), angle.sin(), angle.cos()),
		)
	}

	pub fn rotate_y(angle: T) -> Self
		where T: Copy + Trig + Neg<Output=T> + Zero + One {
		mat3(
			vec3(angle.cos(), T::zero(), angle.sin()),
			vec3(T::zero(), T::one(), T::zero()),
			vec3(-angle.sin(), T::zero(), angle.cos()),
		)
	}

	pub fn rotate_z(angle: T) -> Self
		where T: Copy + Trig + Neg<Output=T> + Zero + One {
		mat3(
			vec3(angle.cos(), -angle.sin(), T::zero()),
			vec3(angle.sin(),  angle.cos(), T::zero()),
			vec3(T::zero()  , T::zero()   , T::one() ),
		)
	}
}

pub fn mat3<T>(x: Vec3<T>, y: Vec3<T>, z: Vec3<T>) -> Mat3<T> {
	Mat3 { x: x, y: y, z: z }
}

impl<T> Default for Mat3<T>
	where T: Zero + One {
	fn default() -> Self {
		Mat3::ident()
	}
}

impl<T> Add<Self> for Mat3<T>
	where T: Add<Output=T> {
	type Output = Self;
	
	fn add(self, other: Self) -> Self {
		mat3(self.x + other.x, self.y + other.y, self.z + other.z)
	}
}

impl<T> AddAssign<Self> for Mat3<T>
	where T: Copy + Add<Output=T> {
	fn add_assign(&mut self, other: Self) {
		*self = *self + other;
	}
}

impl<T> Sub<Self> for Mat3<T>
	where T: Sub<Output=T> {
	type Output = Self;
	
	fn sub(self, other: Self) -> Self {
		mat3(self.x - other.x, self.y - other.y, self.z - other.z)
	}
}

impl<T> SubAssign<Self> for Mat3<T>
	where T: Copy + Sub<Output=T> {
	fn sub_assign(&mut self, other: Self) {
		*self = *self - other;
	}
}

impl<T> Mul<Mat3<T>> for Mat3<T>
	where Vec3<T>: Vector<T> + Copy {
	type Output = Self;
	
	fn mul(self, other: Self) -> Self {
		let t = other.transpose();
		mat3(
			vec3(dot(self.x,t.x), dot(self.x,t.y), dot(self.x,t.z)),
			vec3(dot(self.y,t.x), dot(self.y,t.y), dot(self.y,t.z)),
			vec3(dot(self.z,t.x), dot(self.z,t.y), dot(self.z,t.z)),
		)
	}
}

impl<T> Mul<Vec3<T>> for Mat3<T>
	where Vec3<T>: Vector<T> + Copy {
	type Output = Vec3<T>;
	
	fn mul(self, v: Vec3<T>) -> Vec3<T> {
		self.apply_to(v)
	}
}

impl<T: Neg> Neg for Mat3<T> {
	type Output = Mat3<<T as Neg>::Output>;
	fn neg(self) -> Mat3<<T as Neg>::Output> { mat3(-self.x,-self.y,-self.z) }
}

impl<T> ArrayTuple for Mat3<T> {
	type Array = [[T; 3]; 3];
	type Tuple = ((T,T,T),(T,T,T),(T,T,T));
	fn into_array(self) -> Self::Array {	let Mat3{x,y,z} = self; [x.into_array(),y.into_array(),z.into_array()] }
	fn into_tuple(self) -> Self::Tuple { let Mat3{x,y,z} = self; (x.into_tuple(),y.into_tuple(),z.into_tuple()) }
}

macro convert($T: ty, $($U: ident),*) {
	$(
		impl Mat3<$T> {
			pub fn $U(self) -> Mat3<$U> {
				mat3(self.x.$U(), self.y.$U(), self.z.$U())
			}
		}
	)*
}

convert!(u8,u8,u16,u32,u64,usize,i8,i16,i32,i64,isize,f32,f64);
convert!(u16,u8,u16,u32,u64,usize,i8,i16,i32,i64,isize,f32,f64);
convert!(u32,u8,u16,u32,u64,usize,i8,i16,i32,i64,isize,f32,f64);
convert!(u64,u8,u16,u32,u64,usize,i8,i16,i32,i64,isize,f32,f64);
convert!(usize,u8,u16,u32,u64,usize,i8,i16,i32,i64,isize,f32,f64);
convert!(i8,u8,u16,u32,u64,usize,i8,i16,i32,i64,isize,f32,f64);
convert!(i16,u8,u16,u32,u64,usize,i8,i16,i32,i64,isize,f32,f64);
convert!(i32,u8,u16,u32,u64,usize,i8,i16,i32,i64,isize,f32,f64);
convert!(i64,u8,u16,u32,u64,usize,i8,i16,i32,i64,isize,f32,f64);
convert!(isize,u8,u16,u32,u64,usize,i8,i16,i32,i64,isize,f32,f64);
convert!(f32,u8,u16,u32,u64,usize,i8,i16,i32,i64,isize,f32,f64);
convert!(f64,u8,u16,u32,u64,usize,i8,i16,i32,i64,isize,f32,f64);
convert!(bool,u8,u16,u32,u64,usize,i8,i16,i32,i64,isize);
