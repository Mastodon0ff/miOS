use crate::serial_io::*;

#[repr(C, packed)]
struct Rsdp {
    signature: [u8; 8],
    checksum: u8,
    oem_id: [u8; 6],
    revision: u8,
    rsdt_addr: u32,
    length: u32,
    xsdt_addr: u64,
    ext_checksum: u8,
    reserved: [u8; 3],
}

#[repr(C, packed)]
struct SdtHeader {
    signature: [u8; 4],
    length: u32,
    revision: u8,
    checksum: u8,
    oem_id: [u8; 6],
    oem_table_od: [u8; 8],
    oem_revision: u32,
    creator_id: u32,
    creater_revision: u32,
}

pub fn init(rdsp_addr: u64, hhdm_offset: u64) {
    unsafe {
        let rsdt_phys = *((rdsp_addr + 16) as *const u32) as u64;
        let rsdt_addr = rsdt_phys + hhdm_offset;
        serial_print("rsdt_phys: ");
        serial_print_num(rsdt_phys as usize);
        serial_print("\n");

        let length = *((rsdt_addr + 4) as *const u32) as usize;
        serial_print("rsdt_length: ");
        serial_print_num(length);
        serial_print("\n");

        let num_entries = (length - 36) / 4;
        serial_print("num entries: ");
        serial_print_num(num_entries);
        serial_print("\n");

        let entries = (rsdt_addr + 36) as *const u32;
        for i in 0..num_entries {
            let entry_phys = (*entries.add(i)) as u64;
            let entry_addr = entry_phys + hhdm_offset;
            serial_print("table: ");
            for j in 0..4u64 {
                serial_write_byte(*((entry_addr + j) as *const u8));
            }
            serial_print(" at ");
            serial_print_num(entry_addr as usize);
            serial_print("\n");
        }
    }
}
