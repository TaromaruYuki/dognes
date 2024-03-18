fn main() {
    let mut data = cpu::CPUData::default();
    let mut cpu = cpu::CPU::default();
    cpu.reset(&mut data);

    data.mem.data[0xFFFC] = 0x4C;
    data.mem.data[0xFFFD] = 0x00;
    data.mem.data[0xFFFE] = 0x00;
    data.mem.data[0x0000] = 0xA9;
    data.mem.data[0x0001] = 69;

    for _ in 0..=17 {
        cpu.tick(&mut data);

        if data.clock.state {
            println!(
                "Addr: {:#06x}; Data: {:#04x}; RW: {}; Clock: {}; State: {}",
                data.pins.address,
                data.pins.data,
                data.pins.rw.to_string(),
                data.clock.state,
                data.state.to_string()
            );
        }

        data.clock.tick();

        match data.pins.rw {
            cpu::ReadWrite::R => data.pins.data = data.mem.data[data.pins.address as usize],
            cpu::ReadWrite::W => data.mem.data[data.pins.address as usize] = data.pins.data,
        }
    }

    println!("Done!");
}
