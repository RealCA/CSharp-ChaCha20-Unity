#![allow(non_snake_case)]

use std::ptr;
use std::slice;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use std::arch::is_x86_feature_detected;

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
use std::arch::aarch64::*;

// -------------------- Hardware acceleration --------------------

#[no_mangle]
pub extern "C" fn Vector128_IsHardwareAccelerated() -> bool {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    { is_x86_feature_detected!("sse2") }
    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    { true } // NEON always available on ARMv8
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm", target_arch = "aarch64")))]
    { false }
}

#[no_mangle]
pub extern "C" fn Vector256_IsHardwareAccelerated() -> bool {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    { is_x86_feature_detected!("avx2") }
    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    { true } // NEON can handle 256-bit using 2x128-bit registers
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm", target_arch = "aarch64")))]
    { false }
}

#[no_mangle]
pub extern "C" fn Vector512_IsHardwareAccelerated() -> bool {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    { is_x86_feature_detected!("avx512f") }
    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    { false } // ARM NEON does not have 512-bit registers
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm", target_arch = "aarch64")))]
    { false }
}

// -------------------- Vector128 --------------------

#[no_mangle]
pub extern "C" fn Vector128_Create(input: *const u8, length: usize, offset: usize, out: *mut u8) {
    unsafe {
        let slice = slice::from_raw_parts(input, length);
        let mut data = [0u8; 16];
        let len = slice.len().saturating_sub(offset).min(16);
        data[..len].copy_from_slice(&slice[offset..offset + len]);
        ptr::copy_nonoverlapping(data.as_ptr(), out, 16);
    }
}

#[no_mangle]
pub extern "C" fn Vector128_Xor(a: *const u8, b: *const u8, out: *mut u8) {
    unsafe {
        let va = slice::from_raw_parts(a, 16);
        let vb = slice::from_raw_parts(b, 16);
        let mut result = [0u8; 16];

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        if is_x86_feature_detected!("sse2") {
            let a_ = _mm_loadu_si128(va.as_ptr() as *const _);
            let b_ = _mm_loadu_si128(vb.as_ptr() as *const _);
            let r = _mm_xor_si128(a_, b_);
            _mm_storeu_si128(result.as_mut_ptr() as *mut _, r);
        }
        #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
        {
            let va_neon = vld1q_u8(va.as_ptr());
            let vb_neon = vld1q_u8(vb.as_ptr());
            let r = veorq_u8(va_neon, vb_neon);
            vst1q_u8(result.as_mut_ptr(), r);
        }
        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm", target_arch = "aarch64")))]
        {
            for i in 0..16 { result[i] = va[i] ^ vb[i]; }
        }

        ptr::copy_nonoverlapping(result.as_ptr(), out, 16);
    }
}

// -------------------- Vector256 --------------------

#[no_mangle]
pub extern "C" fn Vector256_Create(input: *const u8, length: usize, offset: usize, out: *mut u8) {
    unsafe {
        let slice = slice::from_raw_parts(input, length);
        let mut data = [0u8; 32];
        let len = slice.len().saturating_sub(offset).min(32);
        data[..len].copy_from_slice(&slice[offset..offset + len]);
        ptr::copy_nonoverlapping(data.as_ptr(), out, 32);
    }
}

#[no_mangle]
pub extern "C" fn Vector256_Xor(a: *const u8, b: *const u8, out: *mut u8) {
    unsafe {
        let va = slice::from_raw_parts(a, 32);
        let vb = slice::from_raw_parts(b, 32);
        let mut result = [0u8; 32];

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        if is_x86_feature_detected!("avx2") {
            let a_ = _mm256_loadu_si256(va.as_ptr() as *const _);
            let b_ = _mm256_loadu_si256(vb.as_ptr() as *const _);
            let r = _mm256_xor_si256(a_, b_);
            _mm256_storeu_si256(result.as_mut_ptr() as *mut _, r);
        }
        #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
        {
            let r1 = veorq_u8(vld1q_u8(va.as_ptr()), vld1q_u8(vb.as_ptr()));
            let r2 = veorq_u8(vld1q_u8(va.as_ptr().add(16)), vld1q_u8(vb.as_ptr().add(16)));
            vst1q_u8(result.as_mut_ptr(), r1);
            vst1q_u8(result.as_mut_ptr().add(16), r2);
        }
        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm", target_arch = "aarch64")))]
        {
            for i in 0..32 { result[i] = va[i] ^ vb[i]; }
        }

        ptr::copy_nonoverlapping(result.as_ptr(), out, 32);
    }
}

// -------------------- Vector512 --------------------

#[no_mangle]
pub extern "C" fn Vector512_Create(input: *const u8, length: usize, offset: usize, out: *mut u8) {
    unsafe {
        let slice = slice::from_raw_parts(input, length);
        let mut data = [0u8; 64];
        let len = slice.len().saturating_sub(offset).min(64);
        data[..len].copy_from_slice(&slice[offset..offset + len]);
        ptr::copy_nonoverlapping(data.as_ptr(), out, 64);
    }
}

#[no_mangle]
pub extern "C" fn Vector512_Xor(a: *const u8, b: *const u8, out: *mut u8) {
    unsafe {
        let va = slice::from_raw_parts(a, 64);
        let vb = slice::from_raw_parts(b, 64);
        let mut result = [0u8; 64];

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        if is_x86_feature_detected!("avx512f") {
            let a_ = _mm512_loadu_si512(va.as_ptr() as *const _);
            let b_ = _mm512_loadu_si512(vb.as_ptr() as *const _);
            let r = _mm512_xor_si512(a_, b_);
            _mm512_storeu_si512(result.as_mut_ptr() as *mut _, r);
        }
        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        {
            for i in 0..64 { result[i] = va[i] ^ vb[i]; }
        }

        ptr::copy_nonoverlapping(result.as_ptr(), out, 64);
    }
}
