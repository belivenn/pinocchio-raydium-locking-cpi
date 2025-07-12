#![no_std]

pub mod instructions;
pub mod states;

use core::mem::MaybeUninit;

const UNINIT_BYTE: MaybeUninit<u8> = MaybeUninit::<u8>::uninit();

#[inline(always)]
fn write_bytes(destination: &mut [MaybeUninit<u8>], source: &[u8]) {
    for (d, s) in destination.iter_mut().zip(source.iter()) {
        d.write(*s);
    }
}
pinocchio_pubkey::declare_id!("LockrWmn6K5twhz3y9w1dQERbmgSaRkfnTeTKbpofwE");
