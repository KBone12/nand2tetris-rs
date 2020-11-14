use crate::{
    chip::{
        arith::alu,
        basic::{and, mux, not, or},
        mem::{Pc, Register},
    },
    signal::{Signal, Word},
};

pub struct Cpu {
    address: [bool; 15],
    write_to_memory: bool,
    result: Word,
    a: Register<Word>,
    d: Register<Word>,
    pc: Pc,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            address: [false; 15],
            write_to_memory: false,
            result: Word::zero(),
            a: Register::new(),
            d: Register::new(),
            pc: Pc::new(),
        }
    }

    pub fn get_output(&self) -> (([bool; 15], bool, Word), Word) {
        (
            (self.address, self.write_to_memory, self.result),
            self.pc.get_output(),
        )
    }

    pub fn tick(&mut self, reset: bool, m: Word, instruction: Word) {
        let instruction = instruction.split();
        let write_to_a = or(not(instruction[0]), instruction[10]);
        let use_m = and(instruction[0], instruction[3]);
        let zero_d = and(instruction[0], instruction[4]);
        let negate_d = and(instruction[0], instruction[5]);
        let zero_a = and(instruction[0], instruction[6]);
        let negate_a = and(instruction[0], instruction[7]);
        let f = and(instruction[0], instruction[8]);
        let negate_alu = and(instruction[0], instruction[9]);
        let write_to_d = and(instruction[0], instruction[11]);
        self.write_to_memory = and(instruction[0], instruction[12]);
        let (result, zero, negate) = alu(
            self.d.get_output(),
            mux(self.a.get_output(), m, use_m),
            zero_d,
            negate_d,
            zero_a,
            negate_a,
            f,
            negate_alu,
        );
        self.result = result;
        let a = self.a.get_output();
        self.pc.tick(
            reset,
            or(
                and(instruction[0], and(instruction[13], negate)),
                or(
                    and(instruction[0], and(instruction[14], zero)),
                    and(
                        instruction[0],
                        and(instruction[15], and(not(zero), not(negate))),
                    ),
                ),
            ),
            true,
            a,
        );
        let mut a_data = instruction.clone();
        a_data[0] = false;
        let a_data = Word::from(a_data);
        self.a
            .tick(write_to_a, mux(a_data, self.result, instruction[0]));
        self.d.tick(write_to_d, self.result);
        let a = self.a.get_output().split();
        self.address = [
            a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11], a[12], a[13],
            a[14], a[15],
        ];
    }

    pub fn a(&self) -> Word {
        self.a.get_output()
    }

    pub fn d(&self) -> Word {
        self.d.get_output()
    }

    pub fn pc(&self) -> Word {
        self.pc.get_output()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tick_increments_pc_if_not_reset_and_not_jump() {
        let mut cpu = Cpu::new();
        cpu.tick(true, Word::zero(), Word::zero());
        assert_eq!(cpu.get_output().1, Word::zero());
        cpu.tick(false, Word::zero(), Word::zero());
        assert_eq!(cpu.get_output().1, Word::from(1));
    }

    #[test]
    fn two_plus_three_equals_five() {
        let mut cpu = Cpu::new();
        cpu.tick(true, Word::zero(), Word::zero()); // Reset
        cpu.tick(false, Word::zero(), Word::from(2)); // A = 2
        cpu.tick(
            false,
            Word::zero(),
            Word::from([
                true, true, true, false, true, true, false, false, false, false, false, true,
                false, false, false, false,
            ]),
        ); // D = A & 1
        cpu.tick(false, Word::zero(), Word::from(3)); // A = 3
        cpu.tick(
            false,
            Word::zero(),
            Word::from([
                true, true, true, false, false, false, false, false, true, false, false, true,
                false, false, false, false,
            ]),
        ); // D = A + D

        assert_eq!(cpu.get_output().0.2, Word::from(5));
    }

    #[test]
    fn max_one_two_equals_two() {
        let mut cpu = Cpu::new();
        cpu.tick(true, Word::zero(), Word::zero()); // Reset
        cpu.tick(false, Word::zero(), Word::zero()); // A = 0
        cpu.tick(
            false,
            Word::from(1),
            Word::from([
                true, true, true, true, true, true, false, false, false, false, false, true, false,
                false, false, false,
            ]),
        ); // D = M[A] = 1
        cpu.tick(false, Word::zero(), Word::from(1)); // A = 1
        cpu.tick(
            false,
            Word::from(2),
            Word::from([
                true, true, true, true, false, true, false, false, true, true, false, true, false,
                false, false, false,
            ]),
        ); // D = -(-(D + 1) + M[A] + 1) = D - M[A] = D - 2
        cpu.tick(false, Word::zero(), Word::from(10)); // A = 10
        cpu.tick(
            false,
            Word::zero(),
            Word::from([
                true, true, true, false, false, false, true, true, false, false, false, false,
                false, false, false, true,
            ]),
        ); // JMP to A if D > 0
        assert_ne!(cpu.get_output().1, cpu.a.get_output());
        cpu.tick(false, Word::zero(), Word::from(1)); // A = 1
        cpu.tick(
            false,
            Word::from(2),
            Word::from([
                true, true, true, true, true, true, false, false, false, false, false, true, false,
                false, false, false,
            ]),
        ); // D = M[A] = 2
        cpu.tick(false, Word::zero(), Word::from(12)); // A = 12
        cpu.tick(
            false,
            Word::zero(),
            Word::from([
                true, true, true, false, true, false, true, false, true, false, false, false,
                false, true, true, true,
            ]),
        ); // JMP to A
        assert_eq!(cpu.get_output().1, cpu.a.get_output());
        assert_eq!(
            cpu.d.get_output(),
            Word::from([
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, true, false
            ])
        );
    }
}
