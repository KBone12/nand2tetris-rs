use crate::chip::{
    arith::alu,
    basic::{and, mux16, not, or},
    mem::{Pc, Register},
};

pub struct Cpu {
    address: [bool; 15],
    write_to_memory: bool,
    result: [bool; 16],
    a: Register,
    d: Register,
    pc: Pc,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            address: [false; 15],
            write_to_memory: false,
            result: [false; 16],
            a: Register::new(),
            d: Register::new(),
            pc: Pc::new(),
        }
    }

    pub fn get_output(&self) -> (([bool; 15], bool, [bool; 16]), [bool; 16]) {
        (
            (self.address, self.write_to_memory, self.result),
            self.pc.get_output(),
        )
    }

    pub fn tick(&mut self, reset: bool, m: &[bool; 16], instruction: &[bool; 16]) {
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
            &self.d.get_output(),
            &mux16(&self.a.get_output(), m, use_m),
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
            &a,
        );
        let mut a_data = *instruction;
        a_data[0] = false;
        self.a
            .tick(write_to_a, &mux16(&a_data, &self.result, instruction[0]));
        self.d.tick(write_to_d, &self.result);
        let a = self.a.get_output();
        self.address = [
            a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11], a[12], a[13],
            a[14], a[15],
        ];
    }

    pub fn a(&self) -> [bool; 16] {
        self.a.get_output()
    }

    pub fn d(&self) -> [bool; 16] {
        self.d.get_output()
    }

    pub fn pc(&self) -> [bool; 16] {
        self.pc.get_output()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tick_increments_pc_if_not_reset_and_not_jump() {
        let mut cpu = Cpu::new();
        cpu.tick(true, &[false; 16], &[false; 16]);
        assert_eq!(cpu.get_output().1, [false; 16]);
        cpu.tick(false, &[false; 16], &[false; 16]);
        assert_eq!(
            cpu.get_output().1,
            [
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, true
            ]
        );
    }

    #[test]
    fn two_plus_three_equals_five() {
        let mut cpu = Cpu::new();
        cpu.tick(true, &[false; 16], &[false; 16]); // Reset
        cpu.tick(
            false,
            &[false; 16],
            &[
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, true, false,
            ],
        ); // A = 2
        cpu.tick(
            false,
            &[false; 16],
            &[
                true, true, true, false, true, true, false, false, false, false, false, true,
                false, false, false, false,
            ],
        ); // D = A & 1
        cpu.tick(
            false,
            &[false; 16],
            &[
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, true, true,
            ],
        ); // A = 3
        cpu.tick(
            false,
            &[false; 16],
            &[
                true, true, true, false, false, false, false, false, true, false, false, true,
                false, false, false, false,
            ],
        ); // D = A + D

        assert_eq!(cpu.get_output().0.2, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true]);
    }

    #[test]
    fn max_one_two_equals_two() {
        let mut cpu = Cpu::new();
        cpu.tick(true, &[false; 16], &[false; 16]); // Reset
        cpu.tick(false, &[false; 16], &[false; 16]); // A = 0
        cpu.tick(
            false,
            &[
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, true,
            ],
            &[
                true, true, true, true, true, true, false, false, false, false, false, true, false,
                false, false, false,
            ],
        ); // D = M[A] = 1
        cpu.tick(
            false,
            &[false; 16],
            &[
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, true,
            ],
        ); // A = 1
        cpu.tick(
            false,
            &[
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, true, false,
            ],
            &[
                true, true, true, true, false, true, false, false, true, true, false, true, false,
                false, false, false,
            ],
        ); // D = -(-(D + 1) + M[A] + 1) = D - M[A] = D - 2
        cpu.tick(
            false,
            &[false; 16],
            &[
                false, false, false, false, false, false, false, false, false, false, false, false,
                true, false, true, false,
            ],
        ); // A = 10
        cpu.tick(
            false,
            &[false; 16],
            &[
                true, true, true, false, false, false, true, true, false, false, false, false,
                false, false, false, true,
            ],
        ); // JMP to A if D > 0
        assert_ne!(cpu.get_output().1, cpu.a.get_output());
        cpu.tick(
            false,
            &[false; 16],
            &[
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, true,
            ],
        ); // A = 1
        cpu.tick(
            false,
            &[
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, true, false,
            ],
            &[
                true, true, true, true, true, true, false, false, false, false, false, true, false,
                false, false, false,
            ],
        ); // D = M[A] = 2
        cpu.tick(
            false,
            &[false; 16],
            &[
                false, false, false, false, false, false, false, false, false, false, false, false,
                true, true, false, false,
            ],
        ); // A = 12
        cpu.tick(
            false,
            &[false; 16],
            &[
                true, true, true, false, true, false, true, false, true, false, false, false,
                false, true, true, true,
            ],
        ); // JMP to A
        assert_eq!(cpu.get_output().1, cpu.a.get_output());
        assert_eq!(
            cpu.d.get_output(),
            [
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, true, false
            ]
        );
    }
}
