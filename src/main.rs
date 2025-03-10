//Author: RR28
//Discord: rrx1c
//Jabber: rrx1c@jabber.fr
//Github profile: https://github.com/RRx1C
//Link to repo: https://github.com/RRx1C/lunettes-mips-rs

mod lm_mips;
use lm_mips::disassembler::LmDisassembler;

fn main() {
    let mut str: String;
    let machine_codes =
    [   0x08100008, 0x00000000, 0x8F990000, 0x27390000, 0x0320F809, 0x00000000, 0x1109003F, 0x00000000,
        0x1611007F, 0x00000000, 0x188000BF, 0x00000000, 0x1CA000FF, 0x00000000, 0x216A0010, 0x26720020,
        0x29AC0005, 0x2EB40006, 0x31EE00FF, 0x36F6ABCD, 0x3B381234, 0x3C065678, 0x5109013F, 0x5611017F,
        0x588001BF, 0x5CA001FF, 0x74100400, 0x00000000, 0x82080010, 0x86290020, 0x8A4A0030, 0x8E6B0040,
        0x928C0050, 0x96AD0060, 0x9ACE0070, 0xA2EF0080, 0xA5180090, 0xA93900A0, 0xACA400B0, 0xB8E600C0,
        0xBD0100D0, 0xC12800E0, 0xC6000100, 0xCA220110, 0xCE430120, 0xD6600130, 0xDA840140, 0xE1490150,
        0xE6A20160, 0xEAC50170, 0xF6E20180, 0xF9060190, 0x000A4900, 0x000C5943, 0x1EE68040, 0x99C00703,
        0xE0000800, 0x00000000, 0x1CF680B0, 0x0000000C, 0x0000000D, 0x0000000F];

    let disassembler: LmDisassembler = LmDisassembler::new_disassembler(lm_mips::LmAddressSize::Lm32bits);

    for i in 0..machine_codes.len(){
        match disassembler.disassemble(machine_codes[i], (0x00400000 + i * 4) as u64){
            Some(instruction) => {
                str = String::from_iter(instruction.string.data());
                let instruction_machine_code = machine_codes[i].to_le_bytes();
                println!("0x{:08x} {:02x} {:02x} {:02x} {:02x} {}", instruction.address, 
                    instruction_machine_code[0], instruction_machine_code[1], instruction_machine_code[2], instruction_machine_code[3],
                    str);
            },
            None => eprintln!("Instruction is probably not implemented yet or something went wrong"),
        };
    }
}