//Author: RR28
//Discord: rrx1c
//Jabber: rrx1c@jabber.fr
//Github profile: https://github.com/RRx1C
//Link to repo: https://github.com/RRx1C/lunettes-mips-rs

use crate::lm_mips::instruction::*;
use crate::lm_mips::operands::*;
use crate::lm_mips::disassembler::*;

//TODO special3 instructions need to be worked on

//Opcode handlers

//Default opcode field handlers
pub fn j(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_J;
    LmDisassembler::jump_format(instruction);
    true
}
pub fn jal(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_JAL;
    LmDisassembler::jump_format(instruction);
    true
}
pub fn beq(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::BranchJump;
    instruction.is_relative = true;
    instruction.is_conditional = true;

    instruction.mnemonic = LM_MNE_BEQ;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 0, 1, 2);
}
pub fn bne(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_BNE;
    instruction.is_conditional = true;
    instruction.is_relative = true;
    instruction.function = LmInstructionFunction::BranchJump;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 0, 1, 2);
}
pub fn blez(instruction: &mut LmInstruction) -> bool{
    instruction.is_relative = true;
    instruction.mnemonic = LM_MNE_BLEZ;
    instruction.is_conditional = true;
    instruction.function = LmInstructionFunction::BranchJump;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 0, 4, 1);
}
pub fn bgtz(instruction: &mut LmInstruction) -> bool{
    instruction.is_relative = true;
    instruction.mnemonic = LM_MNE_BGTZ;
    instruction.function = LmInstructionFunction::BranchJump;
    instruction.is_conditional = true;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 0, 4, 1);
}
pub fn addi(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_ADDI;
    instruction.function = LmInstructionFunction::Computational;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
}
pub fn addiu(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_ADDIU;
    instruction.function = LmInstructionFunction::Computational;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
}
pub fn slti(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_SLTI;
    instruction.function = LmInstructionFunction::Computational;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
}
pub fn sltiu(instruction: &mut LmInstruction) -> bool{
    instruction.is_conditional = true;
    instruction.function = LmInstructionFunction::Computational;
    instruction.mnemonic = LM_MNE_SLTIU;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
}
pub fn andi(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_ANDI;
    instruction.function = LmInstructionFunction::Computational;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
}
pub fn ori(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::Computational;
    instruction.mnemonic = LM_MNE_ORI;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
}
pub fn xori(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::Computational;
    instruction.mnemonic = LM_MNE_XORI;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
}
pub fn lui(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_LUI;
    instruction.function = LmInstructionFunction::Computational;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 4, 0, 1);
}
pub fn beql(instruction: &mut LmInstruction) -> bool{
    instruction.is_relative = true;
    instruction.function = LmInstructionFunction::BranchJump;
    instruction.mnemonic = LM_MNE_BEQL;
    instruction.is_conditional = true;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 0, 1, 2);
}
pub fn bnel(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_BNEL;
    instruction.function = LmInstructionFunction::BranchJump;
    instruction.is_conditional = true;
    instruction.is_relative = true;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 0, 1, 2);
}
pub fn blezl(instruction: &mut LmInstruction) -> bool{
    instruction.is_relative = true;
    instruction.mnemonic = LM_MNE_BLEZL;
    instruction.function = LmInstructionFunction::BranchJump;
    instruction.is_conditional = true;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 0, 4, 1);
}
pub fn bgtzl(instruction: &mut LmInstruction) -> bool{
    instruction.is_relative = true;
    instruction.mnemonic = LM_MNE_BGTZL;
    instruction.function = LmInstructionFunction::BranchJump;
    instruction.is_conditional = true;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 0, 4, 1);
}
pub fn jalx(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_JALX;
    LmDisassembler::jump_format(instruction);
    true
}
pub fn lb(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::LoadStore;
    instruction.mnemonic = LM_MNE_LB;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn lh(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::LoadStore;
    instruction.mnemonic = LM_MNE_LH;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn lwl(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::LoadStore;
    instruction.mnemonic = LM_MNE_LWL;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn lw(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::LoadStore;
    instruction.mnemonic = LM_MNE_LW;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn lbu(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::LoadStore;
    instruction.mnemonic = LM_MNE_LBU;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn lhu(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::LoadStore;
    instruction.mnemonic = LM_MNE_LHU;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn lwr(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::LoadStore;
    instruction.mnemonic = LM_MNE_LWR;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn sb(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::LoadStore;
    instruction.mnemonic = LM_MNE_SB;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn sh(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::LoadStore;
    instruction.mnemonic = LM_MNE_SH;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn swl(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_SWL;
    instruction.function = LmInstructionFunction::LoadStore;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn sw(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::LoadStore;
    instruction.mnemonic = LM_MNE_SW;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn swr(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::LoadStore;
    instruction.mnemonic = LM_MNE_SWR;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn cache(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_CACHE;
    instruction.function = LmInstructionFunction::Miscellaneous;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn ll(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_LL;
    instruction.function = LmInstructionFunction::LoadStore;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn lwc1(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_LWC1;
    instruction.function = LmInstructionFunction::LoadStore;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cp1, 2, 0, 1);
}
pub fn lwc2(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_LWC2;
    instruction.function = LmInstructionFunction::LoadStore;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cp2, 2, 0, 1);
}
pub fn pref(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_PREF;
    instruction.function = LmInstructionFunction::Miscellaneous;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn ldc1(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_LDC1;
    instruction.function = LmInstructionFunction::LoadStore;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cp1, 2, 0, 1);
}
pub fn ldc2(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_LDC2;
    instruction.function = LmInstructionFunction::LoadStore;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cp2, 2, 0, 1);
}
pub fn sc(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_SC;
    instruction.function = LmInstructionFunction::LoadStore;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn swc1(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_SWC1;
    instruction.function = LmInstructionFunction::LoadStore;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cp1, 2, 0, 1);
}
pub fn swc2(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_SWC2;
    instruction.function = LmInstructionFunction::LoadStore;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cp2, 2, 0, 1);
}
pub fn sdc1(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_SDC1;
    instruction.function = LmInstructionFunction::LoadStore;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cp1, 2, 0, 1);
}
pub fn sdc2(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_SDC2;
    instruction.function = LmInstructionFunction::LoadStore;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cp2, 2, 0, 1)
}

//Special
pub fn sll(instruction: &mut LmInstruction) -> bool{
    if instruction.machine_code == 0{
        instruction.format = LmInstructionFormat::Reg;
        instruction.mnemonic = LM_MNE_NOP;
        instruction.function = LmInstructionFunction::Miscellaneous;
        instruction.string.append_str(instruction.mnemonic);
        return true
    }

    instruction.mnemonic = LM_MNE_SLL;
    instruction.function = LmInstructionFunction::Computational;
    return LmDisassembler::reg_format(instruction, 4, 1, 0, 2, 0)
}
pub fn sra(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_SRA;
    instruction.function = LmInstructionFunction::Computational;
    return LmDisassembler::reg_format(instruction, 4, 1, 0, 2, 0)
}
pub fn sllv(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_SLLV;
    instruction.function = LmInstructionFunction::Computational;
    return LmDisassembler::reg_format(instruction, 2, 1, 0, 4, 0)
}
pub fn srav(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_SRAV;
    instruction.function = LmInstructionFunction::Computational;
    return LmDisassembler::reg_format(instruction, 2, 1, 0, 4, 0)
}
pub fn jr(instruction: &mut LmInstruction) -> bool{
    instruction.format = LmInstructionFormat::Reg;
    
    instruction.function = LmInstructionFunction::BranchJump;
    if (instruction.machine_code >> 16 & 0b11111) != 0
        || (instruction.machine_code >> 11 & 0b11111) != 0{
        return false
    }
    if (instruction.machine_code >> 6 & 0b11111) == 0b10000{
        instruction.mnemonic = LM_MNE_JRHB;
    }
    else{
        instruction.mnemonic = LM_MNE_JR;
    }
    instruction.operand[0] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 21 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand_num = 1;

    instruction.string.append_str(instruction.mnemonic);
    instruction.string.append_char(' ');
    instruction.string.append_str(LmOperand::get_reg_str(instruction.operand[0].get_register().unwrap(), instruction.operand[0].get_coprocessor()));
    true
}
pub fn jalr(instruction: &mut LmInstruction) -> bool{
    if (instruction.machine_code >> 16 & 0b11111) != 0{
        return false
    }

    instruction.format = LmInstructionFormat::Reg;
    if (instruction.machine_code >> 6 & 0b11111) == 0b10000{
        instruction.mnemonic = LM_MNE_JALRHB
    }
    else{
        instruction.mnemonic = LM_MNE_JALR
    }

    instruction.function = LmInstructionFunction::BranchJump;
    instruction.operand[1] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 21 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand[0] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 11 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand_num = 2;

    instruction.string.append_str(instruction.mnemonic);

    instruction.string.append_str(LmOperand::get_reg_str(instruction.operand[0].get_register().unwrap(), instruction.operand[0].get_coprocessor()));
    instruction.string.append_str(", ");
    instruction.string.append_str(LmOperand::get_reg_str(instruction.operand[1].get_register().unwrap(), instruction.operand[1].get_coprocessor()));
    true
}
pub fn movz(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::Miscellaneous;
    instruction.is_conditional = true;
        instruction.mnemonic = LM_MNE_MOVZ;
    return LmDisassembler::reg_format(instruction, 1, 2, 0, 4, 0)
}
pub fn movn(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::Miscellaneous;
    instruction.is_conditional = true;
    instruction.mnemonic = LM_MNE_MOVN;
    return LmDisassembler::reg_format(instruction, 1, 2, 0, 4, 0)
}
pub fn syscall(instruction: &mut LmInstruction) -> bool{
    let mut hex_num: LmString = LmString::new_lmstring();

    instruction.operand[0] = LmOperand::new_imm_opreand(((instruction.machine_code >> 6) & 0xFFFFF) as u64);
    hex_num.num_to_str(instruction.operand[0].value);
    instruction.mnemonic = LM_MNE_SYSCALL;
    instruction.function = LmInstructionFunction::Miscellaneous;
    instruction.format = LmInstructionFormat::Other;

    instruction.string.append_str(instruction.mnemonic);
    instruction.string.append_char(' ');
    instruction.string.append_string(&hex_num);

    true
}
pub fn break_handler(instruction: &mut LmInstruction) -> bool{
    let mut hex_num: LmString = LmString::new_lmstring();

    instruction.operand[0] = LmOperand::new_imm_opreand(((instruction.machine_code >> 6) & 0xFFFFF) as u64);
    hex_num.num_to_str(instruction.operand[0].value);
    instruction.mnemonic = LM_MNE_BREAK;
    instruction.function = LmInstructionFunction::Miscellaneous;
    instruction.format = LmInstructionFormat::Other;

    instruction.string.append_str(instruction.mnemonic);
    instruction.string.append_char(' ');
    instruction.string.append_string(&hex_num);
    true
}
pub fn sync(instruction: &mut LmInstruction) -> bool{
    if (instruction.machine_code >> 11 & 0xffff) != 0{
        return false
    }
    instruction.function = LmInstructionFunction::Miscellaneous;
    instruction.format = LmInstructionFormat::Other;
    instruction.mnemonic = LM_MNE_SYNC;
    instruction.operand[0] = LmOperand::new_imm_opreand(((instruction.machine_code >> 6) & 0xFFFFF) as u64);
    return LmDisassembler::reg_format(instruction, 4, 4, 4, 0, 0)
}

//Special2
pub fn madd(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::Computational;
    instruction.mnemonic = LM_MNE_MADD;
    LmDisassembler::reg_format(instruction, 0, 1, 4, 4, 0)
}
pub fn maddu(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::Computational;
    instruction.mnemonic = LM_MNE_MADDU;
    LmDisassembler::reg_format(instruction, 0, 1, 4, 4, 0)
}
pub fn mul(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::Computational;
    instruction.mnemonic = LM_MNE_MUL;
    LmDisassembler::reg_format(instruction, 1, 2, 0, 4, 0)
}
pub fn msub(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::Computational;
    instruction.mnemonic = LM_MNE_MSUB;
    LmDisassembler::reg_format(instruction, 0, 1, 4, 4, 0)
}
pub fn msubu(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::Computational;
    instruction.mnemonic = LM_MNE_MSUBU;
    LmDisassembler::reg_format(instruction, 0, 1, 4, 4, 0)
}
pub fn clz(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::Computational;
    instruction.mnemonic = LM_MNE_CLZ;

    let success = LmDisassembler::reg_format(instruction, 0, 4, 1, 4, 4);
    instruction.operand[2] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 16 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand_num += 1;
    
    return success
}
pub fn clo(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::Computational;
    instruction.mnemonic = LM_MNE_CLO;

    let success = LmDisassembler::reg_format(instruction, 0, 4, 1, 4, 4);
    instruction.operand[2] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 16 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand_num += 1;
    
    return success
}
pub fn sdbbp(instruction: &mut LmInstruction) -> bool{
    let mut hex_num: LmString = LmString::new_lmstring();

    instruction.operand[0] = LmOperand::new_imm_opreand(((instruction.machine_code >> 6) & 0xFFFFF) as u64);
    hex_num.num_to_str(instruction.operand[0].value);
    instruction.mnemonic = LM_MNE_SDBBP;
    instruction.function = LmInstructionFunction::Miscellaneous;
    instruction.format = LmInstructionFormat::Other;

    instruction.string.append_str(instruction.mnemonic);
    instruction.string.append_char(' ');
    instruction.string.append_string(&hex_num);
    true
}

//Special3 They need some testing
pub fn ext(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_EXT;
    unimplemented!("[-]This handler isn't implemented yet");
}
pub fn ins(instruction: &mut LmInstruction) -> bool{
    let mut hex_num: LmString = LmString::new_lmstring();

    instruction.function = LmInstructionFunction::Miscellaneous;
    instruction.mnemonic = LM_MNE_INS;
    instruction.format = LmInstructionFormat::Reg;

    instruction.operand_num = 4;
    instruction.operand[1] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 21 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand[3] = LmOperand::new_imm_opreand((instruction.machine_code >> 11 & 0b11111) as u64);
    instruction.operand[0] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 16 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand[2] = LmOperand::new_imm_opreand((instruction.machine_code >> 6 & 0b11111) as u64);
    
    hex_num.num_to_str(instruction.operand[2].value);

    instruction.string.append_str(instruction.mnemonic);
    instruction.string.append_char(' ');
    instruction.string.append_str(LmOperand::get_reg_str(instruction.operand[0].get_register().unwrap(), instruction.operand[0].get_coprocessor()));
    instruction.string.append_str(", ");
    instruction.string.append_str(LmOperand::get_reg_str(instruction.operand[1].get_register().unwrap(), instruction.operand[1].get_coprocessor()));
    instruction.string.append_str(", ");
    instruction.string.append_string(&hex_num);
    instruction.string.append_str(", ");
    hex_num.num_to_str(instruction.operand[3].value);
    instruction.string.append_string(&hex_num);
    false
}
pub fn bshfl(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = match instruction.machine_code & 0b11111000000{
        0b00010 => LM_MNE_WSBH,
        0b10000 => LM_MNE_SEB,
        0b11000 => LM_MNE_SEH,
        _ => return false
    };
    instruction.function = LmInstructionFunction::Computational;
    LmDisassembler::reg_format(instruction, 4, 1, 0, 4, 1)
}
pub fn rdhwr(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::Miscellaneous;
    instruction.mnemonic = LM_MNE_RDHWR;
    return LmDisassembler::reg_format(instruction, 4, 0, 1, 4, 0)
}