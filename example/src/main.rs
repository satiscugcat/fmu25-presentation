#[cfg(target_arch="aarch64")]
use core::arch::aarch64::*;

#[cfg(target_arch="aarch64")]
fn main() {
    unsafe {
	let a = vld1q_s8(&(127 as i8) as *const i8);
	let b = vld1q_s8(&(-2 as i8) as *const i8);

	let result = vabdq_s8(a, b);
	let result = *(&result as *const int8x16_t as *const [i8; 16]);
	
	println!("{}", result[0])
    }
}
