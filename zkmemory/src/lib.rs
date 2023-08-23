/// This crate provides a simple RAM machine for use in the zkVM
#[deny(warnings, unused, nonstandard_style, missing_docs, unsafe_code)]

/// A state machine with two instructions [Write](crate::machine::Instruction::Write) and [Read](crate::machine::Instruction::Read).
mod ram_machine;
pub use ram_machine::*;

#[cfg(test)]
mod tests {
    use crate::base::{Base, U256};
    use crate::machine::{RAMMachine, StateMachine256, StateMachine32};

    #[test]
    #[should_panic]
    fn sm256_invalid_init() {
        StateMachine256::new(255);
    }

    #[test]
    #[should_panic]
    fn sm32_invalid_init() {
        StateMachine32::new(28);
    }

    #[test]
    fn sm256_write_read_one_cell() {
        let mut sm = StateMachine256::new(256);
        let chunk = U256::from_bytes([5u8; 32]);
        sm.write(U256::from_usize(0), chunk);

        assert_eq!(sm.read(U256::from_usize(0)), chunk);
    }

    #[test]
    fn sm256_read_empty_cell() {
        let mut sm = StateMachine256::new(256);
        let chunk = U256::from_bytes([0u8; 32]);

        assert_eq!(sm.read(U256::from_usize(32)), chunk);
    }

    #[test]
    fn sm256_write_one_cell_read_two_cell() {
        let mut sm = StateMachine256::new(256);
        let chunk_1 = U256::from_bytes([5u8; 32]);
        let chunk_2 = U256::from_bytes([10u8; 32]);

        let expected: [u8; 32] = [[5u8; 17].as_slice(), [10u8; 15].as_slice()]
            .concat()
            .try_into()
            .unwrap();

        sm.write(U256::from_usize(0), chunk_1);
        sm.write(U256::from_usize(32), chunk_2);
        assert_eq!(sm.read(U256::from_usize(15)), U256::from_bytes(expected));
    }

    #[test]
    fn sm256_write_two_cell_read_one_cell() {
        let mut sm = StateMachine256::new(256);

        let chunk = U256::from_bytes([1u8; 32]);
        sm.write(U256::from_usize(23), chunk);
        let expected_lo: [u8; 32] = [[0u8; 23].as_slice(), [1u8; 9].as_slice()]
            .concat()
            .try_into()
            .unwrap();
        let expected_hi: [u8; 32] = [[1u8; 23].as_slice(), [0u8; 9].as_slice()]
            .concat()
            .try_into()
            .unwrap();
        assert_eq!(sm.read(U256::from_usize(0)), U256::from_bytes(expected_lo));
        assert_eq!(sm.read(U256::from_usize(32)), U256::from_bytes(expected_hi));
    }

    #[test]
    fn sm32_read_empty_cell() {
        let mut sm = StateMachine32::new(32);
        assert_eq!(sm.read(64), 0u32);
    }

    #[test]
    fn sm32_write_read_one_cell() {
        let mut sm = StateMachine32::new(32);
        let chunk = 12u32;
        sm.write(0u32, chunk);
        assert_eq!(sm.read(0u32), 12u32);
    }

    #[test]
    fn sm32_write_one_cell_read_two_cells() {
        let mut sm = StateMachine32::new(32);
        let chunk_1 = u32::from_bytes([7u8; 4]);
        let chunk_2 = u32::from_bytes([10u8; 4]);
        sm.write(0u32, chunk_1);
        sm.write(4u32, chunk_2);
        assert_eq!(sm.read(3u32), u32::from_be_bytes([7u8, 10, 10, 10]));
    }

    #[test]
    fn sm32_write_two_cells_read_one_cells() {
        let mut sm = StateMachine32::new(32);
        let chunk = u32::from_bytes([3u8; 4]);
        sm.write(2u32, chunk);
        assert_eq!(sm.read(0u32), u32::from_be_bytes([0u8, 0, 3, 3]));
        assert_eq!(sm.read(4u32), u32::from_be_bytes([3u8, 3, 0, 0]));
    }

    #[test]
    fn u256_test() {
        let chunk_1 = U256::from_bytes([9u8; 32]);
        let chunk_2 = U256::from_usize(10);
        assert_eq!(chunk_1.to_bytes(), [9u8; 32]);
        assert_eq!(U256::zero(), U256::from_bytes([0u8; 32]));
        assert_eq!(chunk_2.to_usize(), 10 as usize);
        assert!(!chunk_1.is_zero());
    }

    #[test]
    /// The testcases above already covered Add, Sub and Rem. This test case covers Div
    fn u256_arithmetic_test() {
        let chunk_1 = U256::from_bytes([34u8; 32]);
        let chunk_2 = U256::from_bytes([17u8; 32]);
        assert_eq!(chunk_1 / chunk_2, U256::from_usize(2));
    }

    #[test]
    fn u32_test() {
        let chunk_1 = u32::from_bytes([73u8; 4]);
        let chunk_2 = u32::from_usize(103);
        assert_eq!(chunk_1.to_bytes(), [73u8; 4]);
        assert_eq!(u32::zero(), u32::from_bytes([0u8; 4]));
        assert_eq!(chunk_2.to_usize(), 103 as usize);
        assert!(!chunk_1.is_zero());
    }

    #[test]
    fn u64_test() {
        let chunk_1 = u64::from_bytes([15u8; 8]);
        let chunk_2 = u64::from_usize(235);
        assert_eq!(chunk_1.to_bytes(), [15u8; 8]);
        assert_eq!(u64::zero(), u64::from_bytes([0u8; 8]));
        assert_eq!(chunk_2.to_usize(), 235 as usize);
        assert!(!chunk_1.is_zero());
    }
}
