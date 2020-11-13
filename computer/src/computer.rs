use crate::{cpu::Cpu, keyboard::Keyboard, memory::Memory, rom::Rom, screen::Screen};

pub struct Computer<S: Screen, K: Keyboard> {
    rom: Rom,
    cpu: Cpu,
    memory: Memory<S, K>,
}

impl<S: Screen, K: Keyboard> Computer<S, K> {
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

    pub fn set_keystate(&mut self, state: K::State) {
        self.memory.set_keystate(state);
    }

    pub fn set_rom(&mut self, rom: Rom) {
        self.rom = rom;
    }

    pub fn screen(&self) -> &S {
        self.memory.screen()
    }
}
