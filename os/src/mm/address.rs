use crate::config::PAGE_SIZE_BITS;

const PA_WIDTH_SV39: usize = 56;
const VA_WIDTH_SV39:usize = 39;
const PPN_WIDTH_SV39:usize = PA_WIDTH_SV39 - PAGE_SIZE_BITS;
const VPN_WIDTH_SV39: usize = VA_WIDTH_SV39 - PAGE_SIZE_BITS;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysAddr(pub usize);