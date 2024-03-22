use super::{CPUData, ReadWrite, CPU};
use std::collections::HashMap;

pub type CaseFunction = fn(&mut CPU, &mut CPUData);
pub type CaseHashMap = HashMap<i8, CaseFunction>;

pub(crate) fn implied() -> CaseHashMap {
    HashMap::new()
}

pub(crate) fn immediate() -> CaseHashMap {
    let mut map: CaseHashMap = HashMap::new();

    map.insert(0, methods::get_current_byte);

    map
}

pub(crate) fn zero_page() -> CaseHashMap {
    let mut map: CaseHashMap = HashMap::new();

    map.insert(0, methods::get_current_byte); // Byte after opcode
    map.insert(1, methods::get_byte_from_data); // Content at opcode

    map
}

fn zero_page_func(func: CaseFunction) -> CaseHashMap {
    let mut map: CaseHashMap = HashMap::new();

    map.insert(0, methods::get_current_byte);
    map.insert(1, func);
    map.insert(2, methods::get_byte_from_temp8);

    map
}

pub(crate) fn zero_page_x() -> CaseHashMap {
    zero_page_func(methods::add_x_to_data)
}

pub(crate) fn zero_page_y() -> CaseHashMap {
    zero_page_func(methods::add_y_to_data)
}

pub(crate) fn absolute() -> CaseHashMap {
    let mut map: CaseHashMap = HashMap::new();

    map.insert(0, methods::get_current_byte);
    map.insert(1, methods::get_second_current_byte);

    map
}

fn absolute_page(func: CaseFunction) -> CaseHashMap {
    let mut map: CaseHashMap = HashMap::new();

    map.insert(0, methods::get_current_byte_tempb);
    map.insert(1, func);
    map.insert(2, methods::boundary_check_get_addr);

    map
}

pub(crate) fn absolute_x_page() -> CaseHashMap {
    absolute_page(|cpu, data| {
        methods::get_second_current_byte_add_reg_tempb(cpu, data, cpu.x);
    })
}

pub(crate) fn absolute_y_page() -> CaseHashMap {
    absolute_page(|cpu, data| {
        methods::get_second_current_byte_add_reg_tempb(cpu, data, cpu.y);
    })
}

fn absolute_reg() -> CaseHashMap {
    let mut map: CaseHashMap = HashMap::new();

    map.insert(0, methods::get_current_byte);
    map.insert(1, methods::get_second_current_byte);

    map
}

pub(crate) fn absolute_x() -> CaseHashMap {
    let mut map: CaseHashMap = absolute_reg();

    map.insert(2, methods::create_address_add_x);
    map.insert(3, methods::store_x);

    map
}

pub(crate) fn absolute_y() -> CaseHashMap {
    let mut map: CaseHashMap = absolute_reg();

    map.insert(2, methods::create_address_add_y);
    map.insert(3, methods::store_y);

    map
}

pub(crate) fn indirect() -> CaseHashMap {
    let mut map: CaseHashMap = HashMap::new();

    map.insert(0, methods::get_current_byte);
    map.insert(1, methods::get_second_current_byte);
    map.insert(2, methods::get_byte_from_temp16);
    map.insert(3, methods::get_second_byte_from_temp16);

    map
}

pub(crate) fn indirect_x() -> CaseHashMap {
    let mut map: CaseHashMap = HashMap::new();

    map.insert(0, methods::get_current_byte);
    map.insert(1, methods::get_second_current_byte_add_x);
    map.insert(2, methods::get_next_byte_add_1_temp8);
    map.insert(3, methods::set_temp16_msb_from_data);
    map.insert(4, methods::get_byte_from_temp16_addr);

    map
}

pub(crate) fn indirect_y_page() -> CaseHashMap {
    let mut map: CaseHashMap = HashMap::new();

    map.insert(0, methods::get_current_byte_tempb);
    map.insert(1, methods::get_byte_from_data_store);
    map.insert(2, methods::get_next_byte_add_1_var);
    map.insert(3, methods::add_y_to_addr_check_of);

    map
}

pub(crate) fn indirect_y() -> CaseHashMap {
    let mut map: CaseHashMap = HashMap::new();

    map.insert(0, methods::get_current_byte);
    map.insert(1, methods::get_byte_from_data_store);
    map.insert(2, methods::get_next_byte_add_1_var);
    map.insert(3, methods::add_y_to_addr_ignore_of);

    map
}

pub(crate) fn branch_clear(value: bool) -> CaseHashMap {
    let mut map: CaseHashMap = HashMap::new();
    map.insert(0, |_, _| {});

    if value {
        map.insert(1, |cpu, _| {
            (cpu.pc, _) = cpu.pc.overflowing_add(1);
            cpu.instruction_finish();
        });
    } else {
        map.insert(1, methods::get_current_byte);
        map.insert(2, |cpu, data| {
            let signed_data = data.pins.data as i8;
            let old_pc = cpu.pc;
            let amount = signed_data.unsigned_abs() as u16;

            if signed_data < 0 {
                // Negative
                (cpu.pc, _) = cpu.pc.overflowing_sub(amount);
            } else {
                // Positive / Zero
                (cpu.pc, _) = cpu.pc.overflowing_add(amount);
            }

            if (old_pc & 0xFF00) == (cpu.pc & 0xFF00) {
                // Don't need extra cycle for page crossing
                cpu.instruction_finish();
            }
        });
        map.insert(3, |cpu, _| {
            cpu.instruction_finish();
        });
    }

    map
}

pub(crate) fn branch_set(value: bool) -> CaseHashMap {
    let mut map: CaseHashMap = HashMap::new();
    map.insert(0, |_, _| {});

    if !value {
        map.insert(1, |cpu, _| {
            (cpu.pc, _) = cpu.pc.overflowing_add(1);
            cpu.instruction_finish();
        });
    } else {
        map.insert(1, methods::get_current_byte);
        map.insert(2, |cpu, data| {
            let signed_data = data.pins.data as i8;
            let old_pc = cpu.pc;
            let amount = signed_data.unsigned_abs() as u16;

            if signed_data < 0 {
                // Negative
                (cpu.pc, _) = cpu.pc.overflowing_sub(amount);
            } else {
                // Positive / Zero
                (cpu.pc, _) = cpu.pc.overflowing_add(amount);
            }

            if (old_pc & 0xFF00) == (cpu.pc & 0xFF00) {
                // Don't need extra cycle for page crossing
                cpu.instruction_finish();
            }
        });
        map.insert(3, |cpu, _| {
            cpu.instruction_finish();
        });
    }

    map
}

pub mod methods {
    use super::{CPUData, CaseFunction, ReadWrite, CPU};

    pub fn get_current_byte(cpu: &mut CPU, data: &mut CPUData) {
        data.pins.address = cpu.pc;
        data.pins.rw = ReadWrite::R;
        cpu.pc += 1;
    }

    pub fn get_current_byte_tempb(cpu: &mut CPU, data: &mut CPUData) {
        cpu.tempb = false;
        get_current_byte(cpu, data);
    }

    pub fn get_second_current_byte(cpu: &mut CPU, data: &mut CPUData) {
        cpu.temp16 = data.pins.data as u16;
        data.pins.address = cpu.pc;
        data.pins.rw = ReadWrite::R;
        cpu.pc += 1;
    }

    pub fn get_second_current_byte_add_reg_tempb(cpu: &mut CPU, data: &mut CPUData, reg: u8) {
        let (addr, of) = data.pins.data.overflowing_add(reg);
        cpu.tempb = of;
        cpu.temp16 = addr as u16;
        data.pins.address = cpu.pc;
        data.pins.rw = ReadWrite::R;
        cpu.pc += 1;
    }

    pub fn get_second_current_byte_add_x(cpu: &mut CPU, data: &mut CPUData) {
        (cpu.temp8, _) = data.pins.data.overflowing_add(cpu.x);
        data.pins.address = cpu.temp8 as u16;
        data.pins.rw = ReadWrite::R;
    }

    pub fn boundary_check_get_addr(cpu: &mut CPU, data: &mut CPUData) {
        if cpu.tempb {
            // We passed a page boundary, need to take a extra cycle
            (cpu.temp8, _) = data.pins.data.overflowing_add(1);
        } else {
            let addr = cpu.temp16 | ((data.pins.data as u16) << 8);
            data.pins.address = addr;
        }

        data.pins.rw = ReadWrite::R;
    }

    pub fn get_byte_from_data_save_addr(cpu: &mut CPU, data: &mut CPUData) {
        cpu.temp16 = data.pins.data as u16;
        get_byte_from_data(cpu, data);
    }

    pub fn get_byte_from_temp16(cpu: &mut CPU, data: &mut CPUData) {
        create_address_from_data_save_addr(cpu, data);
        (cpu.temp16, _) = cpu.temp16.overflowing_add(1);
        data.pins.rw = ReadWrite::R;
    }

    pub fn get_second_byte_from_temp16(cpu: &mut CPU, data: &mut CPUData) {
        data.pins.address = cpu.temp16;
        data.pins.rw = ReadWrite::R;
        cpu.temp16 = data.pins.data as u16;
    }

    pub fn get_byte_from_data(_cpu: &mut CPU, data: &mut CPUData) {
        data.pins.address = data.pins.data as u16;
        data.pins.rw = ReadWrite::R;
    }

    pub fn add_x_to_data(cpu: &mut CPU, data: &mut CPUData) {
        (cpu.temp8, _) = data.pins.data.overflowing_add(cpu.x);
    }

    pub fn add_y_to_data(cpu: &mut CPU, data: &mut CPUData) {
        (cpu.temp8, _) = data.pins.data.overflowing_add(cpu.y);
    }

    pub fn get_byte_from_temp8_save_addr(cpu: &mut CPU, data: &mut CPUData) {
        cpu.temp16 = cpu.temp8 as u16;
        get_byte_from_temp8(cpu, data);
    }

    pub fn get_byte_from_temp8(cpu: &mut CPU, data: &mut CPUData) {
        data.pins.address = cpu.temp8 as u16;
        data.pins.rw = ReadWrite::R;
    }

    pub fn get_data_or_return(cpu: &mut CPU, data: &mut CPUData, func: CaseFunction) {
        if cpu.tempb {
            let addr = cpu.temp16 | ((cpu.temp8 as u16) << 8);
            data.pins.address = addr;
            data.pins.rw = ReadWrite::R;
        } else {
            func(cpu, data);
        }
    }

    pub fn create_address_from_data(cpu: &mut CPU, data: &mut CPUData) {
        let addr: u16 = cpu.temp16 | ((data.pins.data as u16) << 8);
        data.pins.address = addr;
    }

    pub fn create_address_from_data_save_addr(cpu: &mut CPU, data: &mut CPUData) {
        create_address_from_data(cpu, data);
        cpu.temp16 = data.pins.address;
    }

    pub fn get_next_byte_add_1_temp8(cpu: &mut CPU, data: &mut CPUData) {
        cpu.temp16 = data.pins.data as u16;
        (cpu.temp8, _) = cpu.temp8.overflowing_add(1);
        data.pins.address = cpu.temp8 as u16;
        data.pins.rw = ReadWrite::R;
    }

    pub fn get_next_byte_add_1_var(cpu: &mut CPU, data: &mut CPUData) {
        cpu.temp16 = data.pins.data as u16;
        let (addr, _) = cpu.temp8.overflowing_add(1);
        data.pins.address = addr as u16;
        data.pins.rw = ReadWrite::R;
    }

    pub fn set_temp16_msb_from_data(cpu: &mut CPU, data: &mut CPUData) {
        cpu.temp16 |= (data.pins.data as u16) << 8;
    }

    pub fn get_byte_from_temp16_addr(cpu: &mut CPU, data: &mut CPUData) {
        data.pins.address = cpu.temp16;
        data.pins.rw = ReadWrite::R;
    }

    pub fn get_byte_from_data_store(cpu: &mut CPU, data: &mut CPUData) {
        get_byte_from_data(cpu, data);
        cpu.temp8 = data.pins.data;
    }

    pub fn add_y_to_addr_check_of(cpu: &mut CPU, data: &mut CPUData) {
        let old = data.pins.data;
        cpu.temp16 |= (data.pins.data as u16) << 8;
        (cpu.temp16, _) = cpu.temp16.overflowing_add(cpu.y as u16);

        cpu.tempb = ((cpu.temp16 >> 8) as u8) > old;

        data.pins.address = cpu.temp16;
        data.pins.rw = ReadWrite::R;
    }

    pub fn add_y_to_addr_ignore_of(cpu: &mut CPU, data: &mut CPUData) {
        cpu.temp16 |= (data.pins.data as u16) << 8;
        (cpu.temp16, _) = cpu.temp16.overflowing_add(cpu.y as u16);
    }

    pub fn store_if_overflow_or_end(cpu: &mut CPU, data: &mut CPUData, func: CaseFunction) {
        if !cpu.tempb {
            func(cpu, data);
        }

        cpu.temp8 = data.pins.data;
    }

    fn store_zero_page_register(_cpu: &mut CPU, data: &mut CPUData, reg: u8) {
        data.pins.address = data.pins.data as u16;
        data.pins.data = reg;
        data.pins.rw = ReadWrite::W;
    }

    pub fn store_zero_page_a(cpu: &mut CPU, data: &mut CPUData) {
        store_zero_page_register(cpu, data, cpu.a);
    }

    pub fn store_zero_page_x(cpu: &mut CPU, data: &mut CPUData) {
        store_zero_page_register(cpu, data, cpu.x);
    }

    pub fn store_zero_page_y(cpu: &mut CPU, data: &mut CPUData) {
        store_zero_page_register(cpu, data, cpu.y);
    }

    pub fn store_temp_8_in_temp16(cpu: &mut CPU, data: &mut CPUData) {
        data.pins.address = cpu.temp16;
        data.pins.data = cpu.temp8;
        data.pins.rw = ReadWrite::W;
    }

    fn store_register(cpu: &mut CPU, data: &mut CPUData, reg: u8) {
        data.pins.address = cpu.temp16;
        data.pins.data = reg;
        data.pins.rw = ReadWrite::W;
    }

    pub fn store_a(cpu: &mut CPU, data: &mut CPUData) {
        store_register(cpu, data, cpu.a);
    }

    pub fn store_x(cpu: &mut CPU, data: &mut CPUData) {
        store_register(cpu, data, cpu.x);
    }

    pub fn store_y(cpu: &mut CPU, data: &mut CPUData) {
        store_register(cpu, data, cpu.y);
    }

    fn store_zero_page_temp_register(cpu: &mut CPU, data: &mut CPUData, reg: u8) {
        data.pins.address = cpu.temp8 as u16;
        data.pins.data = reg;
        data.pins.rw = ReadWrite::W;
    }

    pub fn store_zero_page_temp_a(cpu: &mut CPU, data: &mut CPUData) {
        store_zero_page_temp_register(cpu, data, cpu.a);
    }

    pub fn store_zero_page_temp_x(cpu: &mut CPU, data: &mut CPUData) {
        store_zero_page_temp_register(cpu, data, cpu.x);
    }

    pub fn store_zero_page_temp_y(cpu: &mut CPU, data: &mut CPUData) {
        store_zero_page_temp_register(cpu, data, cpu.y);
    }

    fn create_address_add_offset(cpu: &mut CPU, data: &mut CPUData, reg: u8) {
        let addr: u16 = cpu.temp16 | ((data.pins.data as u16) << 8);
        (cpu.temp16, _) = addr.overflowing_add(reg as u16);
    }

    pub fn create_address_add_x(cpu: &mut CPU, data: &mut CPUData) {
        create_address_add_offset(cpu, data, cpu.x);
    }

    pub fn create_address_add_y(cpu: &mut CPU, data: &mut CPUData) {
        create_address_add_offset(cpu, data, cpu.y);
    }
}
