use crate::{cpu::Cpu, memory::Memory, rom::Rom};

pub struct Computer {
    rom: Rom,
    cpu: Cpu,
    memory: Memory,
}

impl Computer {
    pub fn new() -> Self {
        Self {
            rom: Rom::new(),
            cpu: Cpu::new(),
            memory: Memory::new(),
        }
    }

    pub fn tick(&mut self, reset: bool) {
        let ((address, write_to_memory, cpu_output), pc) = self.cpu.get_output();
        self.memory.tick(&address, write_to_memory, &cpu_output);
        let memory_data = self.memory.get_output();
        let pc = [
            pc[1], pc[2], pc[3], pc[4], pc[5], pc[6], pc[7], pc[8], pc[9], pc[10], pc[11], pc[12],
            pc[13], pc[14], pc[15],
        ];
        self.rom.set_address(&pc);
        let instruction = self.rom.get_output();
        self.cpu.tick(reset, &memory_data, &instruction);
    }
}
