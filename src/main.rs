pub mod cpu;
pub mod memory_bus;

use dmg_01::cpu::CPU;
fn main() {
    let mut cpu = CPU::new();

    dmg_01::run(&mut cpu);
}
