pub use paste;

/// Creates a `#[repr(C, packed)]` struct with automatic padding between
/// fields based on specified byte offsets.
///
/// Also generates a `Debug` impl that safely reads potentially unaligned
/// fields via `core::ptr::read_unaligned`.
///
/// # Examples
///
/// ```
/// use offsetter::offset;
///
/// offset!(
///     struct Player {
///         0x0 health: f32,
///         0x4 max_health: f32,
///         0x8 team_id: u32,
///     }
/// );
/// ```
///
/// With attributes:
/// ```
/// use offsetter::offset;
///
/// offset!(
///     #[derive(Clone, Copy)]
///     struct Entity {
///         0x0  vtable: u64,
///         0x8  id: u32,
///         0x10 position: [f32; 3],
///     }
/// );
/// ```
#[macro_export]
macro_rules! offset {
    (@guard ($current_offset:expr,) -> {$(#[$attr:meta])* $vis:vis struct $name:ident $(($offset:expr, $amount:expr, $id:ident: $ty:ty))*}) => {
        $crate::paste::paste! {
            #[repr(C)]
            $(#[$attr])*
            $vis struct $name {
                $(
                    [<_pad $id>]: [u8; $amount],
                    pub $id: $ty
                ),*
            }
        }

        // compile-time alignment checks
        $(
            const _: () = assert!(
                $offset % core::mem::align_of::<$ty>() == 0,
                concat!(
                    "field `", stringify!($id), "` at offset ",
                    stringify!($offset),
                    " violates alignment of type `", stringify!($ty), "`"
                )
            );
        )*
    };

    (@guard ($current_offset:expr, $offset:literal $fvis:vis $id:ident: $ty:ty, $($next:tt)*) -> {$($output:tt)*}) => {
        $crate::offset!(@guard ($offset + core::mem::size_of::<$ty>(), $($next)*) -> {$($output)* ($offset, $offset - ($current_offset), $id: $ty)});
    };

    ($(#[$attr:meta])* $vis:vis struct $name:ident { $($input:tt)* }) => {
        $crate::offset!(@guard (0, $($input)*) -> {$(#[$attr])* $vis struct $name});
    };
}