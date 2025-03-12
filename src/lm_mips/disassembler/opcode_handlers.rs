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
    instruction.mnemonic_id = LmMnemonicId::J;
    LmDisassembler::jump_format(instruction);
    true
}
pub fn jal(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Jal;
    LmDisassembler::jump_format(instruction);
    true
}
pub fn beq(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Beq;
    instruction.function = LmInstructionFunction::BranchJump;
    instruction.is_relative = true;
    instruction.is_conditional = true;

    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 0, 1, 2);
}
pub fn bne(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Bne;
    instruction.is_conditional = true;
    instruction.is_relative = true;
    instruction.function = LmInstructionFunction::BranchJump;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 0, 1, 2);
}
pub fn blez(instruction: &mut LmInstruction) -> bool{
    if (instruction.machine_code >> 16 & 5) != 0{
        return false
    }
    instruction.is_relative = true;
    instruction.mnemonic_id = LmMnemonicId::Blez;
    instruction.is_conditional = true;
    instruction.function = LmInstructionFunction::BranchJump;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 0, 4, 1);
}
pub fn bgtz(instruction: &mut LmInstruction) -> bool{
    if (instruction.machine_code >> 16 & 5) != 0{
        return false
    }
    instruction.is_relative = true;
    instruction.mnemonic_id = LmMnemonicId::Bgtz;
    instruction.function = LmInstructionFunction::BranchJump;
    instruction.is_conditional = true;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 0, 4, 1);
}
pub fn addi(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Addi;
    instruction.function = LmInstructionFunction::Computational;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
}
pub fn addiu(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Addiu;
    instruction.function = LmInstructionFunction::Computational;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
}
pub fn slti(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Slti;
    instruction.function = LmInstructionFunction::Computational;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
}
pub fn sltiu(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Sltiu;
    instruction.is_conditional = true;
    instruction.function = LmInstructionFunction::Computational;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
}
pub fn andi(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Andi;
    instruction.function = LmInstructionFunction::Computational;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
}
pub fn ori(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::Computational;
    instruction.mnemonic_id = LmMnemonicId::Ori;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
}
pub fn xori(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::Computational;
    instruction.mnemonic_id = LmMnemonicId::Xori;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
}
pub fn lui(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Lui;
    instruction.function = LmInstructionFunction::Computational;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 4, 0, 1);
}
pub fn beql(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Beql;
    instruction.is_relative = true;
    instruction.function = LmInstructionFunction::BranchJump;
    instruction.is_conditional = true;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 0, 1, 2);
}
pub fn bnel(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Bnel;
    instruction.function = LmInstructionFunction::BranchJump;
    instruction.is_conditional = true;
    instruction.is_relative = true;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 0, 1, 2);
}
pub fn blezl(instruction: &mut LmInstruction) -> bool{
    if (instruction.machine_code >> 16 & 5) != 0{
        return false
    }
    instruction.is_relative = true;
    instruction.mnemonic_id = LmMnemonicId::Blezl;
    instruction.function = LmInstructionFunction::BranchJump;
    instruction.is_conditional = true;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 0, 4, 1);
}
pub fn bgtzl(instruction: &mut LmInstruction) -> bool{
    if (instruction.machine_code >> 16 & 5) != 0{
        return false
    }
    instruction.is_relative = true;
    instruction.mnemonic_id = LmMnemonicId::Bgtzl;
    instruction.function = LmInstructionFunction::BranchJump;
    instruction.is_conditional = true;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 0, 4, 1);
}
pub fn jalx(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Jalx;
    LmDisassembler::jump_format(instruction);
    true
}
pub fn lb(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Lb;
    instruction.function = LmInstructionFunction::LoadStore;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn lh(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::LoadStore;
    instruction.mnemonic_id = LmMnemonicId::Lh;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn lwl(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Lwl;
    instruction.function = LmInstructionFunction::LoadStore;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn lw(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Lw;
    instruction.function = LmInstructionFunction::LoadStore;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn lbu(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Lbu;
    instruction.function = LmInstructionFunction::LoadStore;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn lhu(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Lhu;
    instruction.function = LmInstructionFunction::LoadStore;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn lwr(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Lwr;
    instruction.function = LmInstructionFunction::LoadStore;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn sb(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::LoadStore;
    instruction.mnemonic_id = LmMnemonicId::Sb;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn sh(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Sh;
    instruction.function = LmInstructionFunction::LoadStore;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn swl(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Swl;
    instruction.function = LmInstructionFunction::LoadStore;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn sw(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::LoadStore;
    instruction.mnemonic_id = LmMnemonicId::Sw;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn swr(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::LoadStore;
    instruction.mnemonic_id = LmMnemonicId::Swr;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn cache(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Cache;
    instruction.function = LmInstructionFunction::Miscellaneous;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn ll(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Ll;
    instruction.function = LmInstructionFunction::LoadStore;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn lwc1(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Lwc1;
    instruction.function = LmInstructionFunction::LoadStore;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cp1, 2, 0, 1);
}
pub fn lwc2(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Lwc2;
    instruction.function = LmInstructionFunction::LoadStore;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cp2, 2, 0, 1);
}
pub fn pref(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Pref;
    instruction.function = LmInstructionFunction::Miscellaneous;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn ldc1(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Ldc1;
    instruction.function = LmInstructionFunction::LoadStore;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cp1, 2, 0, 1);
}
pub fn ldc2(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Ldc2;
    instruction.function = LmInstructionFunction::LoadStore;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cp2, 2, 0, 1);
}
pub fn sc(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Sc;
    instruction.function = LmInstructionFunction::LoadStore;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn swc1(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Swc1;
    instruction.function = LmInstructionFunction::LoadStore;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cp1, 2, 0, 1);
}
pub fn swc2(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Swc2;
    instruction.function = LmInstructionFunction::LoadStore;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cp2, 2, 0, 1);
}
pub fn sdc1(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Sdc1;
    instruction.function = LmInstructionFunction::LoadStore;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cp1, 2, 0, 1);
}
pub fn sdc2(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Sdc2;
    instruction.function = LmInstructionFunction::LoadStore;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cp2, 2, 0, 1)
}

//Special
pub fn sll(instruction: &mut LmInstruction) -> bool{
        if instruction.machine_code == 0{
            instruction.format = LmInstructionFormat::Reg;
            instruction.mnemonic_id = LmMnemonicId::Nop;
            instruction.function = LmInstructionFunction::Miscellaneous;
            instruction.string.append_str(LmInstruction::get_memonic(instruction.mnemonic_id));
            return true
        }

    instruction.mnemonic_id = LmMnemonicId::Sll;
    instruction.function = LmInstructionFunction::Computational;
    return LmDisassembler::reg_format(instruction, 4, 1, 0, 2, 0)
}
pub fn sra(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Sra;
    instruction.function = LmInstructionFunction::Computational;
    return LmDisassembler::reg_format(instruction, 4, 1, 0, 2, 0)
}
pub fn sllv(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Sllv;
    instruction.function = LmInstructionFunction::Computational;
    return LmDisassembler::reg_format(instruction, 2, 1, 0, 4, 0)
}
pub fn srav(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Srav;
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
        instruction.mnemonic_id = LmMnemonicId::Jrhb;
    }
    else{
        instruction.mnemonic_id = LmMnemonicId::Jr;
    }
    instruction.operand[0] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 21 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand_num = 1;

    instruction.string.append_str(LmInstruction::get_memonic(instruction.mnemonic_id));
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
        instruction.mnemonic_id = LmMnemonicId::Jalrhb;
    }
    else{
        instruction.mnemonic_id = LmMnemonicId::Jalr;
    }

    instruction.function = LmInstructionFunction::BranchJump;
    instruction.operand[1] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 21 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand[0] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 11 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand_num = 2;

    instruction.string.append_str(LmInstruction::get_memonic(instruction.mnemonic_id));
    instruction.string.append_char(' ');
    instruction.string.append_str(LmOperand::get_reg_str(instruction.operand[0].get_register().unwrap(), instruction.operand[0].get_coprocessor()));
    instruction.string.append_str(", ");
    instruction.string.append_str(LmOperand::get_reg_str(instruction.operand[1].get_register().unwrap(), instruction.operand[1].get_coprocessor()));
    true
}
pub fn movz(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::Miscellaneous;
    instruction.is_conditional = true;
    instruction.mnemonic_id = LmMnemonicId::Movz;
    return LmDisassembler::reg_format(instruction, 1, 2, 0, 4, 0)
}
pub fn movn(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::Miscellaneous;
    instruction.is_conditional = true;
    instruction.mnemonic_id = LmMnemonicId::Movn;
    return LmDisassembler::reg_format(instruction, 1, 2, 0, 4, 0)
}
pub fn syscall(instruction: &mut LmInstruction) -> bool{
    let mut hex_num: LmString = LmString::new_lmstring();

    instruction.operand[0] = LmOperand::new_imm_opreand(((instruction.machine_code >> 6) & 0xFFFFF) as u64);
    hex_num.num_to_str(instruction.operand[0].value);
    instruction.mnemonic_id = LmMnemonicId::Syscall;
    instruction.function = LmInstructionFunction::Miscellaneous;
    instruction.format = LmInstructionFormat::Other;

    instruction.string.append_str(LmInstruction::get_memonic(instruction.mnemonic_id));
    instruction.string.append_char(' ');
    instruction.string.append_string(&hex_num);

    true
}
pub fn break_handler(instruction: &mut LmInstruction) -> bool{
    let mut hex_num: LmString = LmString::new_lmstring();

    instruction.operand[0] = LmOperand::new_imm_opreand(((instruction.machine_code >> 6) & 0xFFFFF) as u64);
    hex_num.num_to_str(instruction.operand[0].value);
    instruction.mnemonic_id = LmMnemonicId::Break;
    instruction.function = LmInstructionFunction::Miscellaneous;
    instruction.format = LmInstructionFormat::Other;

    instruction.string.append_str(LmInstruction::get_memonic(instruction.mnemonic_id));
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
    instruction.mnemonic_id = LmMnemonicId::Sync;
    instruction.operand[0] = LmOperand::new_imm_opreand(((instruction.machine_code >> 6) & 0xFFFFF) as u64);
    return LmDisassembler::reg_format(instruction, 4, 4, 4, 0, 0)
}

//Special2
pub fn madd(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::Computational;
    instruction.mnemonic_id = LmMnemonicId::Madd;
    LmDisassembler::reg_format(instruction, 0, 1, 4, 4, 0)
}
pub fn maddu(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::Computational;
    instruction.mnemonic_id = LmMnemonicId::Maddu;
    LmDisassembler::reg_format(instruction, 0, 1, 4, 4, 0)
}
pub fn mul(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::Computational;
    instruction.mnemonic_id = LmMnemonicId::Mul;
    LmDisassembler::reg_format(instruction, 1, 2, 0, 4, 0)
}
pub fn msub(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::Computational;
    instruction.mnemonic_id = LmMnemonicId::Msub;
    LmDisassembler::reg_format(instruction, 0, 1, 4, 4, 0)
}
pub fn msubu(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::Computational;
    instruction.mnemonic_id = LmMnemonicId::Msubu;
    LmDisassembler::reg_format(instruction, 0, 1, 4, 4, 0)
}
pub fn clz(instruction: &mut LmInstruction) -> bool{
    if (instruction.machine_code >> 6 & 0b11111) != 0{
        return false
    }

    instruction.function = LmInstructionFunction::Computational;
    instruction.mnemonic_id = LmMnemonicId::Clz;

    let success = LmDisassembler::reg_format(instruction, 0, 4, 1, 4, 4);

    instruction.operand[2] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 16 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand_num += 1;
    
    return success
}
pub fn clo(instruction: &mut LmInstruction) -> bool{
    if (instruction.machine_code >> 6 & 0b11111) != 0{
        return false
    }

    instruction.function = LmInstructionFunction::Computational;
    instruction.mnemonic_id = LmMnemonicId::Clo;

    let success = LmDisassembler::reg_format(instruction, 0, 4, 1, 4, 4);

    instruction.operand[2] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 16 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand_num += 1;
    
    return success
}
pub fn sdbbp(instruction: &mut LmInstruction) -> bool{
    let mut hex_num: LmString = LmString::new_lmstring();

    instruction.operand[0] = LmOperand::new_imm_opreand(((instruction.machine_code >> 6) & 0xFFFFF) as u64);
    hex_num.num_to_str(instruction.operand[0].value);
    instruction.mnemonic_id = LmMnemonicId::Sdbbp;
    instruction.function = LmInstructionFunction::Miscellaneous;
    instruction.format = LmInstructionFormat::Other;

    instruction.string.append_str(LmInstruction::get_memonic(instruction.mnemonic_id));
    instruction.string.append_char(' ');
    instruction.string.append_string(&hex_num);
    true
}

//Special3 They need some testing
pub fn ext(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Ext;
    unimplemented!("[-]This handler isn't implemented yet");
}
pub fn ins(instruction: &mut LmInstruction) -> bool{
    let mut hex_num: LmString = LmString::new_lmstring();

    instruction.function = LmInstructionFunction::Miscellaneous;
    instruction.mnemonic_id = LmMnemonicId::Ins;
    instruction.format = LmInstructionFormat::Reg;

    instruction.operand_num = 4;
    instruction.operand[1] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 21 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand[3] = LmOperand::new_imm_opreand((instruction.machine_code >> 11 & 0b11111) as u64);
    instruction.operand[0] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 16 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand[2] = LmOperand::new_imm_opreand((instruction.machine_code >> 6 & 0b11111) as u64);
    
    hex_num.num_to_str(instruction.operand[2].value);

    instruction.string.append_str(LmInstruction::get_memonic(instruction.mnemonic_id));
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
    match instruction.machine_code & 0b11111000000{
        0b00010 => instruction.mnemonic_id = LmMnemonicId::Wsbh,
        0b10000 => instruction.mnemonic_id = LmMnemonicId::Seb,
        0b11000 => instruction.mnemonic_id = LmMnemonicId::Seh,
        _ => return false
    }
    instruction.function = LmInstructionFunction::Computational;
    LmDisassembler::reg_format(instruction, 4, 1, 0, 4, 1)
}
pub fn rdhwr(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::Miscellaneous;
    instruction.mnemonic_id = LmMnemonicId::Rdhwr;
    return LmDisassembler::reg_format(instruction, 4, 0, 1, 4, 0)
}