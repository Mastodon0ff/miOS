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
        let rsdp = &*(rdsp_addr as *const Rsdp);
        let xsdt_phys = rsdp.xsdt_addr as u64;
        let xsdt_addr = xsdt_phys + hhdm_offset;
        serial_print("xsdt_addr: ");
        serial_print_num(xsdt_addr as usize);
        serial_print("\n");

        let xsdt = &*(xsdt_addr as *const SdtHeader);
        let length = xsdt.length as usize;
        serial_print("rsdt_length: ");
        serial_print_num(length);
        serial_print("\n");

        let num_entries = (length - size_of::<SdtHeader>()) / 8;
        serial_print("num entries: ");
        serial_print_num(num_entries);
        serial_print("\n");

        let entries = (xsdt_addr + size_of::<SdtHeader>() as u64) as *const u64;
        for i in 0..num_entries {
            let entry_phys = *entries.add(i);
            if entry_phys == 0 {
                continue;
            }
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
