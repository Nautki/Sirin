use core::borrow::{Borrow, BorrowMut};

#[repr(C, packed)]
pub struct U8Triplet<
    const SIZE_0: usize,
    const SIZE_1: usize,
    const SIZE_2: usize
>(pub [u8; SIZE_0], pub [u8; SIZE_1], pub [u8; SIZE_2]);

impl <
    const SIZE_0: usize,
    const SIZE_1: usize,
    const SIZE_2: usize
> Borrow<[u8]> for U8Triplet<SIZE_0, SIZE_1, SIZE_2> {
    fn borrow(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(self as *const _ as *const u8, SIZE_0 + SIZE_1 + SIZE_2)
        }
    }
}

impl <
    const SIZE_0: usize,
    const SIZE_1: usize,
    const SIZE_2: usize
> BorrowMut<[u8]> for U8Triplet<SIZE_0, SIZE_1, SIZE_2> {
    fn borrow_mut(&mut self) -> &mut [u8] {
        unsafe {
            core::slice::from_raw_parts_mut(self as *mut _ as *mut u8, SIZE_0 + SIZE_1 + SIZE_2)
        }
    }
}