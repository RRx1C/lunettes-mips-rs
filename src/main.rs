//Author: RR28
//Discord: rrx1c
//Jabber: rrx1c@jabber.fr
//Github profile: https://github.com/RRx1C
//Link to repo: https://github.com/RRx1C/lunettes-mips-rs

mod lm_mips;
use lm_mips::disassembler::LmDisassembler;
use lm_mips::instruction::LmCoprocessor;

fn main() {
    let mut str: String;
    let machine_codes =
    [   0x00000000, 0x73000000, 0x00800408, 0x0000000c, 0x000001cf,
        0x08010008, 0x0C104000, 0x11090010, 0x16110020, 0x18A00030, 0x1CA80040, 0x214A0010, 0x25520020,
        0x29940005, 0x2DD60006, 0x31EF00FF, 0x35F6ABCD, 0x39981234, 0x3C065678, 0x51090050, 0x56110060, 
        0x5cc00070, 0x74A10080, 0x81080010, 0x85090020, 0x892A0030, 0x8D2B0040, 0x910C0050, 0x950D0060,
        0x992E0070, 0xA1CF0080, 0xA9D00090, 0xacF100A0, 0xB8A200B0, 0xB9E300C0, 0xBD0400D0, 0xC11000E0,
        0xC4820100, 0xC8C30200, 0xCC840300, 0xD4840130, 0xD8C50400, 0xE0820160, 0xE4C50700, 0xE8860180,
        0xECC70800, 0x00095004, 0x000A5005, 0x000C5006, 0x000E5007, 0x03E00008, 0x03200008, 0x0120F809,
        0x0120F809, 0x016A482A, 0x01AE482B, 0x0000000C, 0x0000000D, 0x0000000F];

    let disassembler: LmDisassembler = LmDisassembler::new_disassembler(lm_mips::LmAddressSize::Lm32bits);

    for i in 0..machine_codes.len(){
        match disassembler.disassemble(machine_codes[i], (0x00400000 + i * 4) as u64){
            Some(instruction) => {
                str = String::from_iter(instruction.string.data());
                let instruction_machine_code = machine_codes[i].to_le_bytes();
                print!("0x{:08x} {:02x} {:02x} {:02x} {:02x} {}", instruction.address, 
                    instruction_machine_code[0], instruction_machine_code[1], instruction_machine_code[2], instruction_machine_code[3],
                    str);
                if instruction.coprocessor == LmCoprocessor::NoCoprocessor{
                    println!(" no coprocessor specified");
                }
                else {
                    println!("");
                }
            },
            None => eprintln!("Instruction is probably not implemented yet or something went wrong"),
        };
    }
}