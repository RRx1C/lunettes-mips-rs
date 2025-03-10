//Opcode handlers
use crate::lm_mips::instruction::*;
use crate::lm_mips::operands::*;
use crate::lm_mips::disassembler::*;

//TODO: Spend more time with JR and JALR
//TODO: Spend more time with Break
//TODO: Pref has the Miscellaneous function and I don't know about cache, I guess it's the same as pref
//TODO: Ya rien qui va

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
    instruction.format = LmInstructionFormat::Imm;
    instruction.function = LmInstructionFunction::BranchJump;
    instruction.is_relative = true;
    instruction.is_conditional = true;

    instruction.operand[0] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 21 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand[1] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 16 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand[2] = LmOperand::new_imm_opreand((instruction.machine_code & 0xffff) as u64);


    true
}
pub fn bne(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Bne;
    instruction.is_conditional = true;
    instruction.is_relative = true;
    instruction.format = LmInstructionFormat::Imm;
    instruction.function = LmInstructionFunction::BranchJump;
    instruction.operand[0] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 21 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand[1] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 16 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand[2] = LmOperand::new_imm_opreand((instruction.machine_code & 0xffff) as u64);
    true
}
pub fn blez(instruction: &mut LmInstruction) -> bool{
    if (instruction.machine_code >> 16 & 5) != 0{
        return false
    }
    instruction.is_relative = true;
    instruction.mnemonic_id = LmMnemonicId::Blez;
    instruction.format = LmInstructionFormat::Imm;
    instruction.is_conditional = true;
    instruction.function = LmInstructionFunction::BranchJump;
    instruction.operand[0] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 21 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand[1] = LmOperand::new_imm_opreand((instruction.machine_code & 0xffff) as u64);
    true
}
pub fn bgtz(instruction: &mut LmInstruction) -> bool{
    if (instruction.machine_code >> 16 & 5) != 0{
        return false
    }
    instruction.is_relative = true;
    instruction.mnemonic_id = LmMnemonicId::Bgtz;
    instruction.function = LmInstructionFunction::BranchJump;
    instruction.is_conditional = true;
    LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 0, 4, 1);
    true
}
pub fn addi(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Addi;
    instruction.function = LmInstructionFunction::Computational;
    LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
    true
}
pub fn addiu(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Addiu;
    instruction.function = LmInstructionFunction::Computational;
    LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
    true
}
pub fn slti(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Slti;
    LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
    instruction.is_conditional = true;
    instruction.function = LmInstructionFunction::Computational;
    true
}
pub fn sltiu(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Sltiu;
    instruction.is_conditional = true;
    instruction.function = LmInstructionFunction::Computational;
    LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
    true
}
pub fn andi(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Andi;
    instruction.function = LmInstructionFunction::Computational;
    LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
    true
}
pub fn ori(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::Computational;
    instruction.mnemonic_id = LmMnemonicId::Ori;
    LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
    true
}
pub fn xori(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::Computational;
    instruction.mnemonic_id = LmMnemonicId::Xori;
    LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
    true
}
pub fn lui(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Lui;
    instruction.format = LmInstructionFormat::Imm;
    instruction.function = LmInstructionFunction::Computational;
    instruction.operand[0] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 16 & 5).unwrap(), LmCoprocessor::Cpu);
    instruction.operand[1] = LmOperand::new_imm_opreand((instruction.machine_code & 0xffff) as u64);
    true
}
pub fn beql(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Beql;
    instruction.format = LmInstructionFormat::Imm;
    instruction.is_relative = true;
    instruction.function = LmInstructionFunction::BranchJump;
    instruction.is_conditional = true;
    instruction.operand[0] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 21 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand[1] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 16 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand[2] = LmOperand::new_imm_opreand((instruction.machine_code & 0xffff) as u64);
    true
}
pub fn bnel(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Bnel;
    instruction.format = LmInstructionFormat::Imm;
    instruction.function = LmInstructionFunction::BranchJump;
    instruction.is_conditional = true;
    instruction.is_relative = true;
    instruction.operand[0] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 21 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand[1] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 16 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand[2] = LmOperand::new_imm_opreand((instruction.machine_code & 0xffff) as u64);
    true
}
pub fn blezl(instruction: &mut LmInstruction) -> bool{
    if (instruction.machine_code >> 16 & 5) != 0{
        return false
    }
    instruction.is_relative = true;
    instruction.mnemonic_id = LmMnemonicId::Blezl;
    instruction.format = LmInstructionFormat::Imm;
    instruction.function = LmInstructionFunction::BranchJump;
    instruction.is_conditional = true;
    instruction.operand[0] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 21 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand[1] = LmOperand::new_imm_opreand((instruction.machine_code & 0xffff) as u64);
    true
}
pub fn bgtzl(instruction: &mut LmInstruction) -> bool{
    if (instruction.machine_code >> 16 & 5) != 0{
        return false
    }
    instruction.is_relative = true;
    instruction.mnemonic_id = LmMnemonicId::Bgtzl;
    instruction.format = LmInstructionFormat::Imm;
    instruction.function = LmInstructionFunction::BranchJump;
    instruction.is_conditional = true;
    instruction.operand[0] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 21 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand[1] = LmOperand::new_imm_opreand((instruction.machine_code & 0xffff) as u64);
    true
}
pub fn jalx(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Jalx;
    LmDisassembler::jump_format(instruction);
    true
}
pub fn lb(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Lb;
    LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
    instruction.function = LmInstructionFunction::LoadStore;
    true
}
pub fn lh(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Lh;
    LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
    instruction.function = LmInstructionFunction::LoadStore;
    true
}
pub fn lwl(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Lwl;
    LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
    instruction.function = LmInstructionFunction::LoadStore;
    true
}
pub fn lw(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Lw;
    LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
    instruction.function = LmInstructionFunction::LoadStore;
    true
}
pub fn lbu(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Lbu;
    LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
    instruction.function = LmInstructionFunction::LoadStore;
    true
}
pub fn lhu(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Lhu;
    LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
    instruction.function = LmInstructionFunction::LoadStore;
    true
}
pub fn lwr(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Lwr;
    LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
    instruction.function = LmInstructionFunction::LoadStore;
    true
}
pub fn sb(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Sb;
    LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
    instruction.function = LmInstructionFunction::LoadStore;
    true
}
pub fn sh(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Sh;
    LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
    instruction.function = LmInstructionFunction::LoadStore;
    true
}
pub fn swl(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Swl;
    LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
    instruction.function = LmInstructionFunction::LoadStore;
    true
}
pub fn sw(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Sw;
    LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
    instruction.function = LmInstructionFunction::LoadStore;
    true
}
pub fn swr(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Swr;
    LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
    instruction.function = LmInstructionFunction::LoadStore;
    true
}
pub fn cache(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Cache;
    instruction.format = LmInstructionFormat::Imm;
    instruction.function = LmInstructionFunction::LoadStore;
    instruction.operand[0] = LmOperand::new_imm_opreand((instruction.machine_code >> 16 & 0b11111) as u64);
    instruction.operand[1] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 21 & 0b11111).unwrap(), LmCoprocessor::NoCoprocessor);
    instruction.operand[2] = LmOperand::new_imm_opreand((instruction.machine_code & 0xffff) as u64);
    true
}
pub fn ll(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Ll;
    instruction.function = LmInstructionFunction::LoadStore;
    LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
    true
}
pub fn lwc1(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Lwc1;
    instruction.function = LmInstructionFunction::LoadStore;
    LmDisassembler::imm_format(instruction, LmCoprocessor::Cp1, 1, 0, 2);
    true
}
pub fn lwc2(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Lwc2;
    instruction.function = LmInstructionFunction::LoadStore;
    LmDisassembler::imm_format(instruction, LmCoprocessor::Cp2, 1, 0, 2);
    true
}
pub fn pref(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Pref;
    instruction.format = LmInstructionFormat::Imm;
    instruction.function = LmInstructionFunction::LoadStore;
    instruction.operand[0] = LmOperand::new_imm_opreand((instruction.machine_code >> 16 & 0b11111) as u64);    
    instruction.operand[1] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 21 & 0b11111).unwrap(), LmCoprocessor::NoCoprocessor);
    instruction.operand[2] = LmOperand::new_imm_opreand((instruction.machine_code & 0xffff) as u64);  
    true
}
pub fn ldc1(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Ldc1;
    instruction.function = LmInstructionFunction::LoadStore;
    LmDisassembler::imm_format(instruction, LmCoprocessor::Cp1, 1, 0, 2);
    true
}
pub fn ldc2(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Ldc2;
    instruction.function = LmInstructionFunction::LoadStore;
    LmDisassembler::imm_format(instruction, LmCoprocessor::Cp2, 1, 0, 2);
    true
}
pub fn sc(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Sc;
    instruction.function = LmInstructionFunction::LoadStore;
    LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
    true
}
pub fn swc1(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Swc1;
    instruction.function = LmInstructionFunction::LoadStore;
    LmDisassembler::imm_format(instruction, LmCoprocessor::Cp1, 1, 0, 2);
    true
}
pub fn swc2(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Swc2;
    instruction.function = LmInstructionFunction::LoadStore;
    LmDisassembler::imm_format(instruction, LmCoprocessor::Cp2, 1, 0, 2);
    true
}
pub fn sdc1(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Sdc1;
    instruction.function = LmInstructionFunction::LoadStore;
    LmDisassembler::imm_format(instruction, LmCoprocessor::Cp1, 1, 0, 2);
    true
}
pub fn sdc2(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic_id = LmMnemonicId::Sdc2;
    instruction.function = LmInstructionFunction::LoadStore;
    LmDisassembler::imm_format(instruction, LmCoprocessor::Cp2, 1, 0, 2);
    true
}
pub fn sll(instruction: &mut LmInstruction) -> bool{
    if instruction.machine_code >> 21 & 0b11111 != 0{
        return false
    }
    instruction.format = LmInstructionFormat::Reg;
    instruction.mnemonic_id = LmMnemonicId::Sll;
    instruction.function = LmInstructionFunction::Computational;
    instruction.operand[0] = LmOperand::new_imm_opreand((instruction.machine_code >> 6 & 0b11111) as u64);
    instruction.operand[1] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 11 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand[2] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 16 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    true
}
pub fn sra(instruction: &mut LmInstruction) -> bool{
    if instruction.machine_code >> 21 & 0b11111 != 0{
        return false
    }
    instruction.format = LmInstructionFormat::Reg;
    instruction.mnemonic_id = LmMnemonicId::Sra;
    instruction.function = LmInstructionFunction::Computational;
    instruction.operand[0] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 11 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand[1] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 16 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand[2] = LmOperand::new_imm_opreand((instruction.machine_code >> 6 & 0b11111) as u64);
    true
}
pub fn sllv(instruction: &mut LmInstruction) -> bool{
    if instruction.machine_code >> 6 & 0b11111 != 0{
        return false
    }
    instruction.format = LmInstructionFormat::Reg;
    instruction.mnemonic_id = LmMnemonicId::Sllv;
    instruction.function = LmInstructionFunction::Computational;
    instruction.operand[0] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 11 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand[1] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 16 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand[2] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 21 & 0b11111) .unwrap(), LmCoprocessor::Cpu);
    true
}
pub fn srav(instruction: &mut LmInstruction) -> bool{
    if instruction.machine_code >> 6 & 0b11111 != 0{
        return false
    }
    instruction.format = LmInstructionFormat::Reg;
    instruction.mnemonic_id = LmMnemonicId::Srav;
    instruction.function = LmInstructionFunction::Computational;
    instruction.operand[0] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 11 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand[1] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 16 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand[2] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 21 & 0b11111) .unwrap(), LmCoprocessor::Cpu);
    true
}
pub fn jr(instruction: &mut LmInstruction) -> bool{
    if instruction.machine_code >> 11 & 0b1111111111 != 0{
        return false
    }
    instruction.function = LmInstructionFunction::BranchJump;
    instruction.format = LmInstructionFormat::Reg;
    if (instruction.machine_code >> 6 & 0b11111) == 0b10000{
        instruction.mnemonic_id = LmMnemonicId::Jrhb;
    }
    else{
        instruction.mnemonic_id = LmMnemonicId::Jr;
    }
    instruction.operand[0] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 21 & 0b11111) .unwrap(), LmCoprocessor::Cpu);
    true
}
pub fn jalr(instruction: &mut LmInstruction) -> bool{
    if instruction.machine_code >> 16 & 0b11111 != 0{
        return false
    }
    instruction.function = LmInstructionFunction::BranchJump;
    instruction.format = LmInstructionFormat::Reg;
    if (instruction.machine_code >> 6 & 0b11111) == 0b10000{
        instruction.mnemonic_id = LmMnemonicId::Jalrhb;
    }
    else{
        instruction.mnemonic_id = LmMnemonicId::Jalr;
    }
    instruction.operand[0] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 11 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand[1] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 21 & 0b11111) .unwrap(), LmCoprocessor::Cpu);
    true
}
pub fn movz(instruction: &mut LmInstruction) -> bool{
    if instruction.machine_code >> 6 & 0b11111 != 0{
        return false
    }
    instruction.function = LmInstructionFunction::Miscellaneous;
    instruction.is_conditional = true;
    instruction.format = LmInstructionFormat::Reg;
    instruction.mnemonic_id = LmMnemonicId::Movz;
    instruction.operand[0] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 11 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand[1] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 21 & 0b11111) .unwrap(), LmCoprocessor::Cpu);
    instruction.operand[2] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 16 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    true
}
pub fn movn(instruction: &mut LmInstruction) -> bool{
    if instruction.machine_code >> 6 & 0b11111 != 0{
        return false
    }
    instruction.function = LmInstructionFunction::Miscellaneous;
    instruction.is_conditional = true;
    instruction.format = LmInstructionFormat::Reg;
    instruction.mnemonic_id = LmMnemonicId::Movn;
    instruction.operand[0] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 11 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand[1] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 21 & 0b11111) .unwrap(), LmCoprocessor::Cpu);
    instruction.operand[2] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 16 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    true
}
pub fn syscall(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::Miscellaneous;
    instruction.format = LmInstructionFormat::Other;
    instruction.mnemonic_id = LmMnemonicId::Syscall;
    instruction.operand[0] = LmOperand::new_imm_opreand(((instruction.machine_code >> 6) & 0xFFFFF) as u64);
    true
}
pub fn break_inst(instruction: &mut LmInstruction) -> bool{
    instruction.function = LmInstructionFunction::Miscellaneous;
    instruction.format = LmInstructionFormat::Other;
    instruction.mnemonic_id = LmMnemonicId::Break;
    // instruction.operands.push(LmOperand::new_imm_opreand(((instruction.machine_code >> 6) & 0xFFFFF) as u64));
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
    true
}