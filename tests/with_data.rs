use offsetter::offset_export;

offset_export!(
    #[derive(Clone, Copy)]
    struct GamePlayer {
        0x0 id: u32,
        0x8 health: f32,
        0xC max_health: f32,
        0x10 position: [f32; 3],
        0x20 team_id: u8,
        0x24 flags: u32,
    }
);

offset_export!(
    #[derive(Clone, Copy)]
    struct Simple {
        0x0 a: u8,
        0x4 b: u32,
        0x8 c: u16,
    }
);

#[test]
fn test_game_player_transmute() {
    let size = core::mem::size_of::<GamePlayer>();
    let mut buf = vec![0u8; size];

    // id = 42 at 0x0
    buf[0x0..0x4].copy_from_slice(&42u32.to_le_bytes());
    // health = 75.5 at 0x8
    buf[0x8..0xC].copy_from_slice(&75.5f32.to_le_bytes());
    // max_health = 100.0 at 0xC
    buf[0xC..0x10].copy_from_slice(&100.0f32.to_le_bytes());
    // position = [1.0, 2.0, 3.0] at 0x10
    buf[0x10..0x14].copy_from_slice(&1.0f32.to_le_bytes());
    buf[0x14..0x18].copy_from_slice(&2.0f32.to_le_bytes());
    buf[0x18..0x1C].copy_from_slice(&3.0f32.to_le_bytes());
    // team_id = 7 at 0x20
    buf[0x20] = 7;
    // flags = 0xDEADBEEF at 0x24
    buf[0x24..0x28].copy_from_slice(&0xDEADBEEFu32.to_le_bytes());

    let player: GamePlayer = unsafe { std::ptr::read(buf.as_ptr() as *const GamePlayer) };

    assert_eq!(player.id, 42);
    assert_eq!(player.health, 75.5);
    assert_eq!(player.max_health, 100.0);
    assert_eq!(player.position, [1.0, 2.0, 3.0]);
    assert_eq!(player.team_id, 7);
    assert_eq!(player.flags, 0xDEADBEEF);
}

#[test]
fn test_simple_transmute() {
    let size = core::mem::size_of::<Simple>();
    let mut buf = vec![0u8; size];

    buf[0x0] = 0xFF;
    buf[0x4..0x8].copy_from_slice(&0x12345678u32.to_le_bytes());
    buf[0x8..0xA].copy_from_slice(&0xABCDu16.to_le_bytes());

    let s: Simple = unsafe { std::ptr::read(buf.as_ptr() as *const Simple) };

    assert_eq!(s.a, 0xFF);
    assert_eq!(s.b, 0x12345678);
    assert_eq!(s.c, 0xABCD);
}