//! Tests

use super::*;

/// The full 12-round permutation. Test vector from XKCP, rev 2a8d2311.
#[test]
fn xoodoo_12_zero() {
    let mut state = XoodooState::default();
    let xoodoo = XoodooP::<12>::default();
    xoodoo.apply(&mut state);
    assert_eq!(
        state.get_state(),
        &[
            0x89D5D88D, 0xA963FCBF, 0x1B232D19, 0xFFA5A014, 0x36B18106, 0xAFC7C1FE, 0xAEE57CBE,
            0xA77540BD, 0x2E86E870, 0xFEF5B7C9, 0x8B4FADF2, 0x5E4F4062,
        ]
    );
}

/// The 6-round permutation. Test vector from XKCP, rev 2a8d2311.
#[test]
fn xoodoo_6_zero() {
    let mut state = XoodooState::default();
    let xoodoo = XoodooP::<6>::default();
    xoodoo.apply(&mut state);
    assert_eq!(
        state.get_state(),
        &[
            0x28C9CEA3, 0xAD204F60, 0x2EC3D0D6, 0xF050C7C5, 0x08DC1225, 0x61992304, 0x9E0D402D,
            0x42D59B9B, 0x1E6114FC, 0x186EB697, 0x35DBBC7F, 0xA1F9104E,
        ]
    );
}
