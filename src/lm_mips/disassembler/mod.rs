//Author: RR28
//Discord: rrx1c
//Jabber: rrx1c@jabber.fr
//Github profile: https://github.com/RRx1C
//Link to repo: https://github.com/RRx1C/lunettes-mips-rs

mod opcode_handlers;

use super::disassembler::opcode_handlers::*;
use super::operands::registers::*;
use super::instruction::*;
use super::LmAddressSize;
use super::operands::*;
use super::utils::string::*;

#[derive(Debug, Copy, Clone)]
pub struct LmDisassembler{
    pub address_size: LmAddressSize,
    pub _version: LmInstructionVersion
}

impl LmDisassembler{
    pub fn new_disassembler(address_size: LmAddressSize) -> LmDisassembler{
        LmDisassembler{
            address_size,
            _version: LmInstructionVersion::NoVersion,
        }
    }
    
    pub fn disassemble(&self, memory: u32, address: u64) -> Option<LmInstruction>{
        //Une map qui réunit tous les handlers des opcodes, il y a d'autre map dans cette map
        const OPCODE_MAP: [fn (instruction: &mut LmInstruction) -> bool; 64] = [
            special_opcode_map, regimm_opcode_map, j, jal, beq, bne,  blez,  bgtz,
            addi,  addiu,  slti,  sltiu,  andi,  ori,  xori,  lui,
            cop0_opcode_map,  cop1_opcode_map,  cop2_opcode_map,  cop1x_opcode_map,  beql,  bnel,  blezl,  bgtzl,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  special2_opcode_map,  jalx,  no_instructions,  special3_opcode_map,
            lb,  lh,  lwl,  lw,  lbu,  lhu,  lwr,  no_instructions,
            sb,  sh,  swl,  sw,  no_instructions,  no_instructions,  swr,  cache,
            ll,  lwc1,  lwc2,  pref,  no_instructions, ldc1, ldc2,  no_instructions,
            sc,  swc1,  swc2,  no_instructions,  no_instructions,  sdc1,  sdc2,  no_instructions];

        let mut instruction: LmInstruction = LmInstruction{
            category: LmInstructionCategory::NoFunction,
            format: LmInstructionFormat::NoFormat,
            operand_num: 0,
            is_conditional: false,
            coprocessor: match memory >> 26{
                0x20 => LmCoprocessor::Cp0,
                0x21 => LmCoprocessor::Cp1,
                0x22 => LmCoprocessor::Cp2,
                0x23 => LmCoprocessor::Cp1x,
                _ => LmCoprocessor::Cpu,
            },
            machine_code: memory,
            operand: [LmOperand::empty_operand(); 4],
            is_relative: false,
            exception: _LmInstructionException::NoException,
            is_region: false,
            string: LmString::new_lmstring(),
            mnemonic: LM_MNE_NO_MNEMONIC,
            address,
            address_size: self.address_size,
            version: LmInstructionVersion::NoVersion
        };
        
        if !OPCODE_MAP[(memory >> 26) as usize](&mut instruction) ||
            instruction.format == LmInstructionFormat::NoFormat ||
            instruction.category == LmInstructionCategory::NoFunction{
                // println!("[-]Instruction couldn't be created for some reasons");
                return None
        }
        return Some(instruction)
    }
    fn reg_format(instruction: &mut LmInstruction, rs: usize, rt: usize, rd: usize, sa: usize, ignore_field: u8) -> bool{
        let mut hex_num: LmString = LmString::new_lmstring();
        let comma: &str = ", ";

        if (ignore_field & 8) == 0 && rs < 4{
            instruction.operand[rs] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 21 & 0b11111).unwrap(), LmCoprocessor::Cpu);
            instruction.operand_num += 1;
        }
        else if (instruction.machine_code >> 21 & 0b11111) != 0{
            return false
        }
        if (ignore_field & 4) == 0 && rt < 4{
            instruction.operand[rt] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 16 & 0b11111).unwrap(), LmCoprocessor::Cpu);
            instruction.operand_num += 1;
        }
        else if (instruction.machine_code >> 16 & 0b11111) != 0{
            return false
        }
        if (ignore_field & 2) == 0 && rd < 4{
            instruction.operand[rd] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 11 & 0b11111).unwrap(), LmCoprocessor::Cpu);
            instruction.operand_num += 1;
        }
        else if (instruction.machine_code >> 11 & 0b11111) != 0{
            return false
        }
        if (ignore_field & 1) == 0 && sa < 4{
            instruction.operand[sa] = LmOperand::new_imm_opreand((instruction.machine_code >> 6 & 0b11111) as u64);
            hex_num.num_to_str(instruction.operand[sa].value);
            instruction.operand_num += 1;
        }
        else if (instruction.machine_code >> 6 & 0b11111) != 0{
            return false
        }

        //Formatting the string
        instruction.string.append_str(instruction.mnemonic);
        instruction.string.append_char(' ');
        instruction.format = LmInstructionFormat::Reg;
        for i in 0..instruction.operand_num{
            if instruction.operand[i]._get_operand_type() == LmOperandType::Reg{
                instruction.string.append_str(LmOperand::get_reg_str(instruction.operand[i].get_register().unwrap(), instruction.operand[i].get_coprocessor()));
            }
            else{
                instruction.string.append_string(&hex_num);
            }
            if i < instruction.operand_num - 1{
                instruction.string.append_str(comma);
            }
        }

        true
    }
    fn imm_format(instruction: &mut LmInstruction, coprocessor: LmCoprocessor, rs: usize, rt: usize, imm: usize) -> bool{
        let mut hex_num: LmString = LmString::new_lmstring();
        let comma: &str = ", ";
        let mut operand_loop = 0;

        //Some attributes about the instruction and setting the operands
        instruction.format = LmInstructionFormat::Imm;
        if rs < 4{
            instruction.operand[rs] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 21 & 0b11111).unwrap(), LmCoprocessor::Cpu);
            instruction.operand_num += 1;
        }
        else if (instruction.machine_code >> 21 & 0b11111) != 0{
                return false
        }

        if rt < 4{
            if instruction.category != LmInstructionCategory::Priviledge && instruction.category != LmInstructionCategory::MemoryControl{
                instruction.operand[rt] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 16 & 0b11111).unwrap(), coprocessor);
            }
            else{
                instruction.operand[rt] = LmOperand::new_imm_opreand((instruction.machine_code >> 16 & 0b11111) as u64);
            }
            instruction.operand_num += 1;
        }
        else if (instruction.machine_code >> 16 & 0b11111) != 0{
                return false
        }

        if imm < 4{
            instruction.operand[imm] = LmOperand::new_imm_opreand((instruction.machine_code & 0xffff) as u64);
            instruction.operand_num += 1;
        }
        else if (instruction.machine_code & 0xffff) != 0{
                return false
        }

        //Formatting the string
        instruction.string.append_str(instruction.mnemonic);    
        instruction.string.append_char(' ');

        //Adds first two operands to the string
        if instruction.category == LmInstructionCategory::Priviledge || instruction.category == LmInstructionCategory::MemoryControl{
            hex_num.num_to_str(instruction.operand[rt].value);
            instruction.string.append_string(&hex_num);
            operand_loop += 1;
            instruction.string.append_str(comma);
        }
        hex_num.num_to_str(instruction.operand[imm].value);
        for i in operand_loop..2{
            if instruction.operand[i]._get_operand_type() == LmOperandType::Reg{
                instruction.string.append_str(LmOperand::get_reg_str(instruction.operand[i].get_register().unwrap(), instruction.operand[i].get_coprocessor()));
            }
            else{
                instruction.string.append_string(&hex_num);
            }
            if i < 2 - 1{
                instruction.string.append_str(comma);
            }
        }
        if instruction.operand_num < 3{
            return true
        }
        if instruction.category != LmInstructionCategory::Load && instruction.category != LmInstructionCategory::Store
            && instruction.category != LmInstructionCategory::Priviledge && instruction.category != LmInstructionCategory::MemoryControl{
            instruction.string.append_str(comma);
            if instruction.operand[2]._get_operand_type() == LmOperandType::Reg{
                instruction.string.append_str(LmOperand::get_reg_str(instruction.operand[2].get_register().unwrap(), instruction.operand[2].get_coprocessor()));
            }
            else{
                instruction.string.append_string(&hex_num);
            }
            return true
        }

        instruction.string.append_char('(');
        instruction.string.append_str(LmOperand::get_reg_str(instruction.operand[2].get_register().unwrap(), instruction.operand[2].get_coprocessor()));
        instruction.string.append_char(')');
        true
    }
    fn jump_format(instruction: &mut LmInstruction) -> (){
        let mut hex_num: LmString = LmString::new_lmstring();

        //Some attributes about the instruction
        instruction.format = LmInstructionFormat::Jump;
        instruction.operand_num = 1 ;
        instruction.is_region = true;
        instruction.category = LmInstructionCategory::BranchJump;
        instruction.operand[0] = LmOperand::new_imm_opreand((instruction.machine_code & 0x3FFFFFF) as u64);

        //Formatting the string
        //If the branch/jump is relative, the string will show it's destination address instead of the offset
        hex_num.num_to_str(instruction.operand[0].value * 0x4 + instruction.address);
        instruction.string.append_str(instruction.mnemonic);
        instruction.string.append_char(' ');
        instruction.string.append_string(&hex_num);

        return;
    }
    pub fn u32_to_register(register: u32) -> Option<LmRegisters>{
        return match register{
            0 => Some(LmRegisters::Zero), 1 => Some(LmRegisters::At), 2 => Some(LmRegisters::V0), 3 => Some(LmRegisters::V1), 4 => Some(LmRegisters::A0), 5 => Some(LmRegisters::A1), 6 => Some(LmRegisters::A2), 7 => Some(LmRegisters::A3),
            8 => Some(LmRegisters::T0), 9 => Some(LmRegisters::T1), 10 => Some(LmRegisters::T2), 11 => Some(LmRegisters::T3), 12 => Some(LmRegisters::T4), 13 => Some(LmRegisters::T5), 14 => Some(LmRegisters::T6), 15 => Some(LmRegisters::T7),
            16 => Some(LmRegisters::S0), 17 => Some(LmRegisters::S1), 18 => Some(LmRegisters::S2), 19 => Some(LmRegisters::S3), 20 => Some(LmRegisters::S4), 21 => Some(LmRegisters::S5), 22 => Some(LmRegisters::S6), 23 => Some(LmRegisters::S7),
            24 => Some(LmRegisters::T8), 25 => Some(LmRegisters::T9), 26 => Some(LmRegisters::K0), 27 => Some(LmRegisters::K1), 28 => Some(LmRegisters::Gp), 29 => Some(LmRegisters::Sp), 30 => Some(LmRegisters::Fp), 31 => Some(LmRegisters::Ra),
            _ => None,
        }
    }
}

//Opcode handlers map
fn no_instructions(_instruction: &mut LmInstruction) -> bool{
    false
}
fn special_opcode_map(instruction: &mut LmInstruction) -> bool{
    static SPECIAL_MAP: [fn(&mut LmInstruction) -> bool; 64] = [
    sll,  no_instructions,  no_instructions,  sra,  sllv,  no_instructions,  no_instructions,  srav,
    jr,  jalr,  movz,  movn,  syscall,  break_handler,  no_instructions,  sync,
    mfhi,  mthi,  mflo,  mtlo,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
    mult,  multu,  div,  divu,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
    add,  addu,  sub,  subu,  and,  or,  xor,  nor,
    no_instructions,  no_instructions,  slt,  sltu,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
    no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
    no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions ];

    SPECIAL_MAP[(instruction.machine_code & 0b111111) as usize](instruction)
}
fn regimm_opcode_map(_instruction: &mut LmInstruction) -> bool{
    static _REGIMM_TABLE: [fn(&mut LmInstruction) -> bool; 64] =
        [   no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions ];
    unimplemented!("[-]Opcode map isn't implemented yet!");
    // REGIMM_TABLE[(instruction.machine_code >> 26) as usize](instruction)
}
fn special2_opcode_map(instruction: &mut LmInstruction) -> bool{
    static SPECIAL2_MAP: [fn(&mut LmInstruction) -> bool; 64] = 
        [   madd,  maddu,  mul,  no_instructions,  msub,  msubu,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            clz,  clo,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  sdbbp ];
    SPECIAL2_MAP[(instruction.machine_code & 0b111111) as usize](instruction)
}
fn special3_opcode_map(instruction: &mut LmInstruction) -> bool{
    static SPECIAL3_MAP: [fn(&mut LmInstruction) -> bool; 64] = 
        [   ext,  no_instructions,  no_instructions,  no_instructions,  ins,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            bshfl,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  rdhwr,  no_instructions,  no_instructions,  no_instructions,  no_instructions ];
    // unimplemented!("[-]Opcode map isn't implemented yet!");
    SPECIAL3_MAP[(instruction.machine_code & 0b111111) as usize](instruction)
}
fn cop0_opcode_map(_instruction: &mut LmInstruction) -> bool{
    static _COP0_MAP: [fn(&mut LmInstruction) -> bool; 64] =
        [   no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions ];
    unimplemented!("[-]Opcode map isn't implemented yet!");
    // instruction.coprocessor = LmCoprocessor::Cp0;
    // COP0_MAP[(instruction.machine_code >> 26) as usize](instruction)
}
fn cop1_opcode_map(_instruction: &mut LmInstruction) -> bool{
    static _COP1_MAP: [fn(&mut LmInstruction) -> bool; 64] =
    [   no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions ];
    unimplemented!("[-]Opcode map isn't implemented yet!");

    // instruction.coprocessor = LmCoprocessor::Cp1;
    // COP1_MAP[(instruction.machine_code >> 26) as usize](instruction)
}
fn cop2_opcode_map(_instruction: &mut LmInstruction) -> bool{
    static _COP2_MAP: [fn(&mut LmInstruction) -> bool; 64] = 
    [   no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions ];
    unimplemented!("[-]Opcode map isn't implemented yet!");

    // instruction.coprocessor = LmCoprocessor::Cp2;
    // COP2_MAP[(instruction.machine_code >> 26) as usize](instruction)
}
fn cop1x_opcode_map(_instruction: &mut LmInstruction) -> bool{
    static _COP1X_MAP: [fn(&mut LmInstruction) -> bool; 64] = 
    [   no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions ];
    unimplemented!("[-]Opcode map isn't implemented yet!");

    // instruction.coprocessor = LmCoprocessor::Cp1x;
    // _COP1X_MAP[(instruction.machine_code >> 26) as usize](instruction)
}