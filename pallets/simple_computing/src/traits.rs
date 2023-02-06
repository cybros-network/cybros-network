use frame_support::sp_runtime::Saturating;
use crate::macros::impl_auto_increment;

pub trait AutoIncrement {
	fn increment(&self) -> Self;
	fn initial_value() -> Self;
}
impl_auto_increment!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
