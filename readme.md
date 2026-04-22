# offsetter

A lightweight Rust macro for defining structs with explicit field offsets and automatic padding generation.

Useful for game hacking, reverse engineering, process memory manipulation, and FFI work where struct layouts are known only partially.

## Disclaimer

This is not my original idea and I didn't come up with the name. All credit goes to the author of [this post](https://www.unknowncheats.me/forum/rust-language-/596798-rust-automatic-offset-padding.html), who for some reason deleted the crate from crates.io.

## Usage

```rust
use offsetter::offset;

offset!(
    pub struct Entity {
        0x0  id: u32,
        0x8  health: f32,
        0xC  max_health: f32,
        0x10 position: [f32; 3],
        0x20 team_id: u32,
    }
);
```

This expands into a `#[repr(C)]` struct with hidden `_padN: [u8; ...]` fields between your fields, ensuring each field sits at the exact byte offset you specified.

## Alignment checking

Offsets are validated at compile time. Misaligned offsets produce a clear error:

```rust
// compile error: "field `b` at offset 0x5 violates alignment of type `u32`"
offset!(
    struct Bad {
        0x0 a: u8,
        0x5 b: u32,
    }
);
```

## Syntax

```rust
offset!(
    [attributes]
    [visibility] struct Name {
        offset [visibility] field_name: Type,
        ...
    }
);
```

- `offset` — hex or decimal byte offset (e.g. `0x10`, `16`)
- Field visibility is optional — fields are always generated as `pub`
- The struct is always `#[repr(C)]`

## Example

```rust
use offsetter::offset;

offset!(
    struct UNICODE_STRING {
        0x0 length: u16,
        0x2 maximum_length: u16,
        0x8 buffer: u64,
    }
);

offset!(
    pub struct Player {
        0x0   vtable: usize,
        0x8   id: u32,
        0xDC  position: [f32; 3],
        0x168 name: [u16; 32],
        0x1BC max_health: u32,
        0x21C health: i32,
        0x250 world_position: [f32; 3],
        0x2D4 target: usize,
        0x2F4 speed: f32,
    }
);
```

## License

MIT