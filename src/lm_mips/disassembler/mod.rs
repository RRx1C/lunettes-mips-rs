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
        const OPCODE_TABLE: [fn (instruction: &mut LmInstruction) -> bool; 64] = [
            special_opcode_table, regimm_opcode_table, j, jal, beq, bne,  blez,  bgtz,
            addi,  addiu,  slti,  sltiu,  andi,  ori,  xori,  lui,
            cop0_opcode_table,  cop1_opcode_table,  cop2_opcode_table,  cop1x_opcode_table,  beql,  bnel,  blezl,  bgtzl,
            empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  jalx,  empty_opcode,  special3_opcode_table,
            lb,  lh,  lwl,  lw,  lbu,  lhu,  lwr,  empty_opcode,
            sb,  sh,  swl,  sw,  empty_opcode,  empty_opcode,  swr,  cache,
            ll,  lwc1,  lwc2,  pref,  empty_opcode, ldc1, ldc2,  empty_opcode,
            sc,  swc1,  swc2,  empty_opcode,  empty_opcode,  sdc1,  sdc2,  empty_opcode];

        let mut instruction: LmInstruction = LmInstruction{
            function: LmInstructionFunction::NoFunction,
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
            operand: [LmOperand::empty_operand(); 3],
            is_relative: false,
            is_region: false,
            string: LmString::new_lmstring(),
            mnemonic_id: LmMnemonicId::NoMnemonic,
            address,
            address_size: self.address_size,
            version: LmInstructionVersion::NoVersion
        };
        
        if !OPCODE_TABLE[(memory >> 26) as usize](&mut instruction) ||
            instruction.mnemonic_id == LmMnemonicId::NoMnemonic ||
            instruction.format == LmInstructionFormat::NoFormat ||
            instruction.function == LmInstructionFunction::NoFunction{
                // println!("[-]Instruction couldn't be created for some reasons");
                return None
        }
        return Some(instruction)
    }

    fn imm_format(instruction: &mut LmInstruction, coprocessor: LmCoprocessor, rs: usize, rt: usize, imm: usize) -> (){
        let mut hex_num: LmString = LmString::new_lmstring();
        let comma: &str = ", ";

        //Some attributes about the instruction and setting the operands
        instruction.format = LmInstructionFormat::Imm;
        if rs < 4{
            instruction.operand[rs] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 21 & 0b11111).unwrap(), LmCoprocessor::Cpu);
            instruction.operand_num += 1;
        }
        if rt < 4{
            instruction.operand[rt] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 16 & 0b11111).unwrap(), coprocessor);
            instruction.operand_num += 1;
        }
        if imm < 4{
            instruction.operand[imm] = LmOperand::new_imm_opreand((instruction.machine_code & 0xffff) as u64);
            instruction.operand_num += 1;
        }

        //Formatting the string
        hex_num.num_to_str(instruction.operand[imm].value);
        instruction.string.append_str(LmInstruction::get_memonic(instruction.mnemonic_id));
        instruction.string.append_char(' ');

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
    }
    fn _imm_branch_str_formatting(instruction: &mut LmInstruction){
        let mut hex_num: LmString = LmString::new_lmstring();

        //Formatting the string
        hex_num.num_to_str(instruction.operand[2].value);
        instruction.string.append_str(LmInstruction::get_memonic(instruction.mnemonic_id));
        instruction.string.append_char(' ');
        instruction.string.append_str(LmInstruction::get_memonic(instruction.mnemonic_id));
        instruction.string.append_string(&hex_num);
    }
    fn jump_format(instruction: &mut LmInstruction) -> (){
        let mut hex_num: LmString = LmString::new_lmstring();

        //Some attributes about the instruction
        instruction.format = LmInstructionFormat::Jump;
        instruction.operand_num = 1 ;
        instruction.is_region = true;
        instruction.function = LmInstructionFunction::BranchJump;
        instruction.operand[0] = LmOperand::new_imm_opreand((instruction.machine_code & 0x3FFFFFF) as u64);

        //Formatting the string
        hex_num.num_to_str(instruction.operand[0].value);
        instruction.string.append_str(LmInstruction::get_memonic(instruction.mnemonic_id));
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
fn empty_opcode(_instruction: &mut LmInstruction) -> bool{
    false
}
fn special_opcode_table(instruction: &mut LmInstruction) -> bool{
    static SPECIAL_TABLE: [fn(&mut LmInstruction) -> bool; 64] = [
    sll,  empty_opcode,  empty_opcode,  sra,  sllv,  empty_opcode,  empty_opcode,  srav,
    jr,  jalr,  movz,  movn,  syscall,  break_inst,  empty_opcode,  sync,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode ];

    SPECIAL_TABLE[(instruction.machine_code & 0b11111) as usize](instruction)
}
fn regimm_opcode_table(_instruction: &mut LmInstruction) -> bool{
    static _REGIMM_TABLE: [fn(&mut LmInstruction) -> bool; 64] = [
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode ];
    unimplemented!("[-]Opcode map isn't implemented yet!");
    // REGIMM_TABLE[(instruction.machine_code >> 26) as usize](instruction)
}
fn special3_opcode_table(_instruction: &mut LmInstruction) -> bool{
    static _SPECIAL3_TABLE: [fn(&mut LmInstruction) -> bool; 64] = [
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode ];
    unimplemented!("[-]Opcode map isn't implemented yet!");
    // SPECIAL3_TABLE[(instruction.machine_code >> 26) as usize](instruction)
}
fn cop0_opcode_table(_instruction: &mut LmInstruction) -> bool{
    static _COP0_TABLE: [fn(&mut LmInstruction) -> bool; 64] = [
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode ];
    unimplemented!("[-]Opcode map isn't implemented yet!");
    // instruction.coprocessor = LmCoprocessor::Cp0;
    // COP0_TABLE[(instruction.machine_code >> 26) as usize](instruction)
}
fn cop1_opcode_table(_instruction: &mut LmInstruction) -> bool{
    static _COP1_TABLE: [fn(&mut LmInstruction) -> bool; 64] = [
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode ];
    unimplemented!("[-]Opcode map isn't implemented yet!");

    // instruction.coprocessor = LmCoprocessor::Cp1;
    // COP1_TABLE[(instruction.machine_code >> 26) as usize](instruction)
}
fn cop2_opcode_table(_instruction: &mut LmInstruction) -> bool{
    static _COP2_TABLE: [fn(&mut LmInstruction) -> bool; 64] = [
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode ];
    unimplemented!("[-]Opcode map isn't implemented yet!");

    // instruction.coprocessor = LmCoprocessor::Cp2;
    // COP2_TABLE[(instruction.machine_code >> 26) as usize](instruction)
}
fn cop1x_opcode_table(_instruction: &mut LmInstruction) -> bool{
    static _COP1X_TABLE: [fn(&mut LmInstruction) -> bool; 64] = [
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode ];
    unimplemented!("[-]Opcode map isn't implemented yet!");

    // instruction.coprocessor = LmCoprocessor::Cp1x;
    // COP1X_TABLE[(instruction.machine_code >> 26) as usize](instruction)
}