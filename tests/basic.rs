use offsetter::offset;

offset!(
    struct Simple {
        0x0 a: u32,
        0x8 b: u64,
        0x20 c: u8,
    }
);

offset!(
    struct StartNonZero {
        0x10 first: u32,
        0x20 second: u64,
    }
);

offset!(
    pub struct WithPub {
        0x0 pub x: f32,
        0x4 pub y: f32,
        0x8 pub z: f32,
    }
);

offset!(
    struct WithPointers {
        0x0 vtable: usize,
        0x8 ptr: *mut u8,
        0x10 val: u32,
    }
);

offset!(
    struct WithArray {
        0x0 id: u32,
        0x8 data: [u8; 16],
        0x20 name: [u8; 32],
    }
);

#[test]
fn test_simple_offsets() {
    assert_eq!(core::mem::offset_of!(Simple, a), 0x0);
    assert_eq!(core::mem::offset_of!(Simple, b), 0x8);
    assert_eq!(core::mem::offset_of!(Simple, c), 0x20);
}

#[test]
fn test_start_non_zero() {
    assert_eq!(core::mem::offset_of!(StartNonZero, first), 0x10);
    assert_eq!(core::mem::offset_of!(StartNonZero, second), 0x20);
}

#[test]
fn test_pub_fields() {
    assert_eq!(core::mem::offset_of!(WithPub, x), 0x0);
    assert_eq!(core::mem::offset_of!(WithPub, y), 0x4);
    assert_eq!(core::mem::offset_of!(WithPub, z), 0x8);
}

#[test]
fn test_pointers() {
    assert_eq!(core::mem::offset_of!(WithPointers, vtable), 0x0);
    assert_eq!(core::mem::offset_of!(WithPointers, ptr), 0x8);
    assert_eq!(core::mem::offset_of!(WithPointers, val), 0x10);
}

#[test]
fn test_arrays() {
    assert_eq!(core::mem::offset_of!(WithArray, id), 0x0);
    assert_eq!(core::mem::offset_of!(WithArray, data), 0x8);
    assert_eq!(core::mem::offset_of!(WithArray, name), 0x20);
}