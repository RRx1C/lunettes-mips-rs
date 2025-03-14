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
    instruction.category = LmInstructionCategory::BranchJump;
    instruction.is_relative = true;
    instruction.is_conditional = true;

    instruction.mnemonic = LM_MNE_BEQ;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 0, 1, 2);
}
pub fn bne(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_BNE;
    instruction.is_conditional = true;
    instruction.is_relative = true;
    instruction.category = LmInstructionCategory::BranchJump;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 0, 1, 2);
}
pub fn blez(instruction: &mut LmInstruction) -> bool{
    instruction.is_relative = true;
    instruction.mnemonic = LM_MNE_BLEZ;
    instruction.is_conditional = true;
    instruction.category = LmInstructionCategory::BranchJump;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 0, 4, 1);
}
pub fn bgtz(instruction: &mut LmInstruction) -> bool{
    instruction.is_relative = true;
    instruction.mnemonic = LM_MNE_BGTZ;
    instruction.category = LmInstructionCategory::BranchJump;
    instruction.is_conditional = true;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 0, 4, 1);
}
pub fn addi(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_ADDI;
    instruction.exception |= LmInstructionException::LmIntOverflowExcept;
    instruction.category = LmInstructionCategory::Arithmetic;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
}
pub fn addiu(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_ADDIU;
    instruction.category = LmInstructionCategory::Arithmetic;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
}
pub fn slti(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_SLTI;
    instruction.category = LmInstructionCategory::Arithmetic;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
}
pub fn sltiu(instruction: &mut LmInstruction) -> bool{
    instruction.is_conditional = true;
    instruction.category = LmInstructionCategory::Arithmetic;
    instruction.mnemonic = LM_MNE_SLTIU;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
}
pub fn andi(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_ANDI;
    instruction.category = LmInstructionCategory::Logical;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
}
pub fn ori(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Logical;
    instruction.mnemonic = LM_MNE_ORI;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
}
pub fn xori(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Logical;
    instruction.mnemonic = LM_MNE_XORI;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 1, 0, 2);
}
pub fn lui(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_LUI;
    instruction.category = LmInstructionCategory::Logical;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 4, 0, 1);
}
pub fn beql(instruction: &mut LmInstruction) -> bool{
    instruction.is_relative = true;
    instruction.category = LmInstructionCategory::BranchJump;
    instruction.mnemonic = LM_MNE_BEQL;
    instruction.is_conditional = true;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 0, 1, 2);
}
pub fn bnel(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_BNEL;
    instruction.category = LmInstructionCategory::BranchJump;
    instruction.is_conditional = true;
    instruction.is_relative = true;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 0, 1, 2);
}
pub fn blezl(instruction: &mut LmInstruction) -> bool{
    instruction.is_relative = true;
    instruction.mnemonic = LM_MNE_BLEZL;
    instruction.category = LmInstructionCategory::BranchJump;
    instruction.is_conditional = true;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 0, 4, 1);
}
pub fn bgtzl(instruction: &mut LmInstruction) -> bool{
    instruction.is_relative = true;
    instruction.mnemonic = LM_MNE_BGTZL;
    instruction.category = LmInstructionCategory::BranchJump;
    instruction.is_conditional = true;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 0, 4, 1);
}
pub fn jalx(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_JALX;
    LmDisassembler::jump_format(instruction);
    true
}
pub fn lb(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Load;
    instruction.mnemonic = LM_MNE_LB;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn lh(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Load;
    instruction.mnemonic = LM_MNE_LH;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn lwl(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Load;
    instruction.mnemonic = LM_MNE_LWL;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn lw(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Load;
    instruction.mnemonic = LM_MNE_LW;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn lbu(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Load;
    instruction.mnemonic = LM_MNE_LBU;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn lhu(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Load;
    instruction.mnemonic = LM_MNE_LHU;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn lwr(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Load;
    instruction.mnemonic = LM_MNE_LWR;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn sb(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Store;
    instruction.mnemonic = LM_MNE_SB;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn sh(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Store;
    instruction.mnemonic = LM_MNE_SH;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn swl(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_SWL;
    instruction.category = LmInstructionCategory::Store;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn sw(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Store;
    instruction.mnemonic = LM_MNE_SW;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn swr(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Store;
    instruction.mnemonic = LM_MNE_SWR;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn cache(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_CACHE;
    instruction.category = LmInstructionCategory::Priviledge;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn ll(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_LL;
    instruction.category = LmInstructionCategory::Load;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn lwc1(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_LWC1;
    instruction.category = LmInstructionCategory::Load;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cp1, 2, 0, 1);
}
pub fn lwc2(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_LWC2;
    instruction.category = LmInstructionCategory::Load;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cp2, 2, 0, 1);
}
pub fn pref(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_PREF;
    instruction.category = LmInstructionCategory::MemoryControl;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn ldc1(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_LDC1;
    instruction.category = LmInstructionCategory::Load;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cp1, 2, 0, 1);
}
pub fn ldc2(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_LDC2;
    instruction.category = LmInstructionCategory::Load;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cp2, 2, 0, 1);
}
pub fn sc(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_SC;
    instruction.category = LmInstructionCategory::Store;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cpu, 2, 0, 1);
}
pub fn swc1(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_SWC1;
    instruction.category = LmInstructionCategory::Store;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cp1, 2, 0, 1);
}
pub fn swc2(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_SWC2;
    instruction.category = LmInstructionCategory::Store;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cp2, 2, 0, 1);
}
pub fn sdc1(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_SDC1;
    instruction.category = LmInstructionCategory::Store;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cp1, 2, 0, 1);
}
pub fn sdc2(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_SDC2;
    instruction.category = LmInstructionCategory::Store;
    return LmDisassembler::imm_format(instruction, LmCoprocessor::Cp2, 2, 0, 1)
}

//Special
pub fn sll(instruction: &mut LmInstruction) -> bool{
    if instruction.machine_code == 0{
        instruction.format = LmInstructionFormat::Reg;
        instruction.mnemonic = LM_MNE_NOP;
        instruction.category = LmInstructionCategory::Control;
        instruction.string.append_str(instruction.mnemonic);
        return true
    }

    instruction.mnemonic = LM_MNE_SLL;
    instruction.category = LmInstructionCategory::Shift;
    return LmDisassembler::reg_format(instruction, 4, 1, 0, 2, 0)
}
pub fn sra(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_SRA;
    instruction.category = LmInstructionCategory::Shift;
    return LmDisassembler::reg_format(instruction, 4, 1, 0, 2, 0)
}
pub fn sllv(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_SLLV;
    instruction.category = LmInstructionCategory::Shift;
    return LmDisassembler::reg_format(instruction, 2, 1, 0, 4, 0)
}
pub fn srav(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_SRAV;
    instruction.category = LmInstructionCategory::Shift;
    return LmDisassembler::reg_format(instruction, 2, 1, 0, 4, 0)
}
pub fn jr(instruction: &mut LmInstruction) -> bool{
    instruction.format = LmInstructionFormat::Reg;
    
    instruction.category = LmInstructionCategory::BranchJump;
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

    instruction.category = LmInstructionCategory::BranchJump;
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
    instruction.category = LmInstructionCategory::Move;
    instruction.is_conditional = true;
    instruction.mnemonic = LM_MNE_MOVZ;
    return LmDisassembler::reg_format(instruction, 1, 2, 0, 4, 0)
}
pub fn movn(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Move;
    instruction.is_conditional = true;
    instruction.mnemonic = LM_MNE_MOVN;
    return LmDisassembler::reg_format(instruction, 1, 2, 0, 4, 0)
}
pub fn syscall(instruction: &mut LmInstruction) -> bool{
    let mut hex_num: LmString = LmString::new_lmstring();

    instruction.operand[0] = LmOpImmediate::new_imm_opreand(((instruction.machine_code >> 6) & 0xFFFFF) as u64);

    if let LmOperand::LmOpImmediate(imm) = instruction.operand[0]{
        hex_num.num_to_str(imm.value);
    };

    instruction.mnemonic = LM_MNE_SYSCALL;
    instruction.category = LmInstructionCategory::Trap;
    instruction.format = LmInstructionFormat::Other;

    instruction.string.append_str(instruction.mnemonic);
    instruction.string.append_char(' ');
    instruction.string.append_string(&hex_num);

    true
}
pub fn break_handler(instruction: &mut LmInstruction) -> bool{
    let mut hex_num: LmString = LmString::new_lmstring();

    instruction.operand[0] = LmOpImmediate::new_imm_opreand(((instruction.machine_code >> 6) & 0xFFFFF) as u64);

    if let LmOperand::LmOpImmediate(imm) = instruction.operand[0]{
        hex_num.num_to_str(imm.value);
    };
    
    instruction.mnemonic = LM_MNE_BREAK;
    instruction.category = LmInstructionCategory::Trap;
    instruction.format = LmInstructionFormat::Other;

    instruction.string.append_str(instruction.mnemonic);
    instruction.string.append_char(' ');
    instruction.string.append_string(&hex_num);
    true
}
pub fn sync(instruction: &mut LmInstruction) -> bool{
    if (instruction.machine_code >> 11 & 0x7fff) != 0{
        return false
    }

    let mut hex_num: LmString = LmString::new_lmstring();

    //Setting the attributes
    instruction.category = LmInstructionCategory::MemoryControl;
    instruction.format = LmInstructionFormat::Other;
    instruction.mnemonic = LM_MNE_SYNC;
    instruction.operand_num = 1;
    instruction.operand[0] = LmOpImmediate::new_imm_opreand(((instruction.machine_code >> 6) & 0b11111) as u64);

    //Formatting the string
    if let LmOperand::LmOpImmediate(imm) = instruction.operand[0]{
        hex_num.num_to_str(imm.value);
    };
    instruction.string.append_str(instruction.mnemonic);
    instruction.string.append_char(' ');
    instruction.string.append_string(&hex_num);
    true
}
pub fn mfhi(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_MFHI;
    instruction.category = LmInstructionCategory::Move;
    LmDisassembler::reg_format(instruction, 4, 4, 0, 4, 0)
}
pub fn mthi(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_MTHI;
    instruction.category = LmInstructionCategory::Move;
    LmDisassembler::reg_format(instruction, 0, 4, 4, 4, 0)
}
pub fn mflo(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_MFLO;
    instruction.category = LmInstructionCategory::Move;
    LmDisassembler::reg_format(instruction, 4, 4, 0, 4, 0)
}
pub fn mtlo(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_MTLO;
    instruction.category = LmInstructionCategory::Move;
    LmDisassembler::reg_format(instruction, 0, 4, 4, 4, 0)
}
pub fn add(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Arithmetic;
    instruction.mnemonic = LM_MNE_ADD;
    LmDisassembler::reg_format(instruction, 1, 2, 0, 4, 0)
}
pub fn addu(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Arithmetic;
    instruction.mnemonic = LM_MNE_ADDU;
    LmDisassembler::reg_format(instruction, 1, 2, 0, 4, 0)
}
pub fn sub(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Arithmetic;
    instruction.mnemonic = LM_MNE_SUB;
    LmDisassembler::reg_format(instruction, 1, 2, 0, 4, 0)
}
pub fn subu(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Arithmetic;
    instruction.mnemonic = LM_MNE_SUBU;
    LmDisassembler::reg_format(instruction, 1, 2, 0, 4, 0)
}
pub fn and(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Logical;
    instruction.mnemonic = LM_MNE_AND;
    LmDisassembler::reg_format(instruction, 1, 2, 0, 4, 0)
}
pub fn or(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Logical;
    instruction.mnemonic = LM_MNE_OR;
    LmDisassembler::reg_format(instruction, 1, 2, 0, 4, 0)
}
pub fn xor(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Logical;
    instruction.mnemonic = LM_MNE_XOR;
    LmDisassembler::reg_format(instruction, 1, 2, 0, 4, 0)
}
pub fn nor(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Logical;
    instruction.mnemonic = LM_MNE_NOR;
    LmDisassembler::reg_format(instruction, 1, 2, 0, 4, 0)
}
pub fn slt(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Arithmetic;
    instruction.is_conditional = true;
    instruction.mnemonic = LM_MNE_SLT;
    LmDisassembler::reg_format(instruction, 1, 2, 0, 4, 0)
}
pub fn sltu(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Arithmetic;
    instruction.is_conditional = true;
    instruction.mnemonic = LM_MNE_SLTU;
    LmDisassembler::reg_format(instruction, 1, 2, 0, 4, 0)
}
pub fn mult(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Arithmetic;
    instruction.mnemonic = LM_MNE_MULT;
    LmDisassembler::reg_format(instruction, 0, 1, 4, 4, 0)
}
pub fn multu(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Arithmetic;
    instruction.mnemonic = LM_MNE_MULTU;
    LmDisassembler::reg_format(instruction, 0, 1, 4, 4, 0)
}
pub fn div(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Arithmetic;
    instruction.mnemonic = LM_MNE_DIV;
    LmDisassembler::reg_format(instruction, 0, 1, 4, 4, 0)
}
pub fn divu(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Arithmetic;
    instruction.mnemonic = LM_MNE_DIVU;
    LmDisassembler::reg_format(instruction, 0, 1, 4, 4, 0)
}
pub fn tge(instruction: &mut LmInstruction) -> bool{
    let mut hex_num: LmString = LmString::new_lmstring();
    
    instruction.mnemonic = LM_MNE_TGE;
    instruction.format = LmInstructionFormat::Other;
    instruction.category = LmInstructionCategory::Trap;
    instruction.operand_num = 3;
    instruction.operand[0] = LmOpRegister::new_reg_opreand((instruction.machine_code >> 21 & 0b11111) as u8, LmCoprocessor::Cpu);
    instruction.operand[1] = LmOpRegister::new_reg_opreand((instruction.machine_code >> 16 & 0b11111) as u8, LmCoprocessor::Cpu);
    instruction.operand[2] = LmOpImmediate::new_imm_opreand((instruction.machine_code >> 6 & 0b1111111111) as u64);

    let reg1 = match instruction.operand[0]{
        LmOperand::LmOpRegister(reg1) => reg1,
        _ => return false,
    };
    let reg2 = match instruction.operand[1]{
        LmOperand::LmOpRegister(reg1) => reg1,
        _ => return false,
    };
    if let LmOperand::LmOpImmediate(imm) = instruction.operand[2]{
        hex_num.num_to_str(imm.value);
    };

    //Formatting
    instruction.string.append_str(instruction.mnemonic);
    instruction.string.append_str(" ");
    instruction.string.append_str(reg1.register);
    instruction.string.append_str(", ");
    instruction.string.append_str(reg2.register);
    instruction.string.append_str(", ");
    instruction.string.append_string(&hex_num);
    true
}
pub fn tgeu(instruction: &mut LmInstruction) -> bool{
    let mut hex_num: LmString = LmString::new_lmstring();
    
    instruction.mnemonic = LM_MNE_TGEU;
    instruction.format = LmInstructionFormat::Other;
    instruction.category = LmInstructionCategory::Trap;
    instruction.operand_num = 3;
    instruction.operand[0] = LmOpRegister::new_reg_opreand((instruction.machine_code >> 21 & 0b11111) as u8, LmCoprocessor::Cpu);
    instruction.operand[1] = LmOpRegister::new_reg_opreand((instruction.machine_code >> 16 & 0b11111) as u8, LmCoprocessor::Cpu);
    instruction.operand[2] = LmOpImmediate::new_imm_opreand((instruction.machine_code >> 6 & 0b1111111111) as u64);

    let reg1 = match instruction.operand[0]{
        LmOperand::LmOpRegister(reg1) => reg1,
        _ => return false,
    };
    let reg2 = match instruction.operand[1]{
        LmOperand::LmOpRegister(reg1) => reg1,
        _ => return false,
    };
    if let LmOperand::LmOpImmediate(imm) = instruction.operand[2]{
        hex_num.num_to_str(imm.value);
    };

    //Formatting
    instruction.string.append_str(instruction.mnemonic);
    instruction.string.append_str(" ");
    instruction.string.append_str(reg1.register);
    instruction.string.append_str(", ");
    instruction.string.append_str(reg2.register);
    instruction.string.append_str(", ");
    instruction.string.append_string(&hex_num);
    true
}
pub fn tlt(instruction: &mut LmInstruction) -> bool{
    let mut hex_num: LmString = LmString::new_lmstring();
    
    instruction.mnemonic = LM_MNE_TLT;
    instruction.format = LmInstructionFormat::Other;
    instruction.category = LmInstructionCategory::Trap;
    instruction.operand_num = 3;
    instruction.operand[0] = LmOpRegister::new_reg_opreand((instruction.machine_code >> 21 & 0b11111) as u8, LmCoprocessor::Cpu);
    instruction.operand[1] = LmOpRegister::new_reg_opreand((instruction.machine_code >> 16 & 0b11111) as u8, LmCoprocessor::Cpu);
    instruction.operand[2] = LmOpImmediate::new_imm_opreand((instruction.machine_code >> 6 & 0b1111111111) as u64);

    let reg1 = match instruction.operand[0]{
        LmOperand::LmOpRegister(reg1) => reg1,
        _ => return false,
    };
    let reg2 = match instruction.operand[1]{
        LmOperand::LmOpRegister(reg1) => reg1,
        _ => return false,
    };
    if let LmOperand::LmOpImmediate(imm) = instruction.operand[2]{
        hex_num.num_to_str(imm.value);
    };

    //Formatting
    instruction.string.append_str(instruction.mnemonic);
    instruction.string.append_str(" ");
    instruction.string.append_str(reg1.register);
    instruction.string.append_str(", ");
    instruction.string.append_str(reg2.register);
    instruction.string.append_str(", ");
    instruction.string.append_string(&hex_num);
    true
}
pub fn tltu(instruction: &mut LmInstruction) -> bool{
    let mut hex_num: LmString = LmString::new_lmstring();
    
    instruction.mnemonic = LM_MNE_TLTU;
    instruction.format = LmInstructionFormat::Other;
    instruction.category = LmInstructionCategory::Trap;
    instruction.operand_num = 3;
    instruction.operand[0] = LmOpRegister::new_reg_opreand((instruction.machine_code >> 21 & 0b11111) as u8, LmCoprocessor::Cpu);
    instruction.operand[1] = LmOpRegister::new_reg_opreand((instruction.machine_code >> 16 & 0b11111) as u8, LmCoprocessor::Cpu);
    instruction.operand[2] = LmOpImmediate::new_imm_opreand((instruction.machine_code >> 6 & 0b1111111111) as u64);

    let reg1 = match instruction.operand[0]{
        LmOperand::LmOpRegister(reg1) => reg1,
        _ => return false,
    };
    let reg2 = match instruction.operand[1]{
        LmOperand::LmOpRegister(reg1) => reg1,
        _ => return false,
    };
    if let LmOperand::LmOpImmediate(imm) = instruction.operand[2]{
        hex_num.num_to_str(imm.value);
    };

    //Formatting
    instruction.string.append_str(instruction.mnemonic);
    instruction.string.append_str(" ");
    instruction.string.append_str(reg1.register);
    instruction.string.append_str(", ");
    instruction.string.append_str(reg2.register);
    instruction.string.append_str(", ");
    instruction.string.append_string(&hex_num);
    true
}
pub fn teq(instruction: &mut LmInstruction) -> bool{
    let mut hex_num: LmString = LmString::new_lmstring();
    
    instruction.mnemonic = LM_MNE_TEQ;
    instruction.format = LmInstructionFormat::Other;
    instruction.category = LmInstructionCategory::Trap;
    instruction.operand_num = 3;
    instruction.operand[0] = LmOpRegister::new_reg_opreand((instruction.machine_code >> 21 & 0b11111) as u8, LmCoprocessor::Cpu);
    instruction.operand[1] = LmOpRegister::new_reg_opreand((instruction.machine_code >> 16 & 0b11111) as u8, LmCoprocessor::Cpu);
    instruction.operand[2] = LmOpImmediate::new_imm_opreand((instruction.machine_code >> 6 & 0b1111111111) as u64);

    let reg1 = match instruction.operand[0]{
        LmOperand::LmOpRegister(reg1) => reg1,
        _ => return false,
    };
    let reg2 = match instruction.operand[1]{
        LmOperand::LmOpRegister(reg1) => reg1,
        _ => return false,
    };
    if let LmOperand::LmOpImmediate(imm) = instruction.operand[2]{
        hex_num.num_to_str(imm.value);
    };

    //Formatting
    instruction.string.append_str(instruction.mnemonic);
    instruction.string.append_str(" ");
    instruction.string.append_str(reg1.register);
    instruction.string.append_str(", ");
    instruction.string.append_str(reg2.register);
    instruction.string.append_str(", ");
    instruction.string.append_string(&hex_num);
    true
}
pub fn tne(instruction: &mut LmInstruction) -> bool{
    //Function variable
    let mut hex_num: LmString = LmString::new_lmstring();

    //Attributes
    instruction.mnemonic = LM_MNE_TNE;
    instruction.format = LmInstructionFormat::Other;
    instruction.category = LmInstructionCategory::Trap;
    instruction.operand_num = 3;
    instruction.operand[0] = LmOpRegister::new_reg_opreand((instruction.machine_code >> 21 & 0b11111) as u8, LmCoprocessor::Cpu);
    instruction.operand[1] = LmOpRegister::new_reg_opreand((instruction.machine_code >> 16 & 0b11111) as u8, LmCoprocessor::Cpu);
    instruction.operand[2] = LmOpImmediate::new_imm_opreand((instruction.machine_code >> 6 & 0b1111111111) as u64);

    let reg1 = match instruction.operand[0]{
        LmOperand::LmOpRegister(reg1) => reg1,
        _ => return false,
    };
    let reg2 = match instruction.operand[1]{
        LmOperand::LmOpRegister(reg1) => reg1,
        _ => return false,
    };
    if let LmOperand::LmOpImmediate(imm) = instruction.operand[2]{
        hex_num.num_to_str(imm.value);
    };

    //Formatting
    instruction.string.append_str(instruction.mnemonic);
    instruction.string.append_str(" ");
    instruction.string.append_str(reg1.register);
    instruction.string.append_str(", ");
    instruction.string.append_str(reg2.register);
    instruction.string.append_str(", ");
    instruction.string.append_string(&hex_num);
    true
}

//Special2
pub fn madd(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Arithmetic;
    instruction.exception |= LmInstructionException::LmIntOverflowExcept;
    instruction.mnemonic = LM_MNE_MADD;
    LmDisassembler::reg_format(instruction, 0, 1, 4, 4, 0)
}
pub fn maddu(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Arithmetic;
    instruction.mnemonic = LM_MNE_MADDU;
    LmDisassembler::reg_format(instruction, 0, 1, 4, 4, 0)
}
pub fn mul(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Arithmetic;
    instruction.mnemonic = LM_MNE_MUL;
    LmDisassembler::reg_format(instruction, 1, 2, 0, 4, 0)
}
pub fn msub(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Arithmetic;
    instruction.mnemonic = LM_MNE_MSUB;
    LmDisassembler::reg_format(instruction, 0, 1, 4, 4, 0)
}
pub fn msubu(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Arithmetic;
    instruction.mnemonic = LM_MNE_MSUBU;
    LmDisassembler::reg_format(instruction, 0, 1, 4, 4, 0)
}
pub fn clz(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Arithmetic;
    instruction.mnemonic = LM_MNE_CLZ;

    let success = LmDisassembler::reg_format(instruction, 0, 4, 1, 4, 4);
    instruction.operand[2] = LmOperand::new_reg_opreand(LmDisassembler::u32_to_register(instruction.machine_code >> 16 & 0b11111).unwrap(), LmCoprocessor::Cpu);
    instruction.operand_num += 1;
    
    return success
}
pub fn clo(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Arithmetic;
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
    instruction.category = LmInstructionCategory::Ejtag;
    instruction.format = LmInstructionFormat::Other;

    instruction.string.append_str(instruction.mnemonic);
    instruction.string.append_char(' ');
    instruction.string.append_string(&hex_num);
    true
}

//Special3 They need some testing
pub fn ext(instruction: &mut LmInstruction) -> bool{
    instruction.mnemonic = LM_MNE_EXT;
    instruction.category = LmInstructionCategory::InsertExtract;
    unimplemented!("[-]This handler isn't implemented yet");
}
pub fn ins(instruction: &mut LmInstruction) -> bool{
    let mut hex_num: LmString = LmString::new_lmstring();

    instruction.category = LmInstructionCategory::InsertExtract;
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
    match instruction.machine_code & 0b11111000000{
        0b00010 => {
            instruction.category = LmInstructionCategory::InsertExtract;
            instruction.mnemonic = LM_MNE_WSBH;},
        0b10000 => {
            instruction.category = LmInstructionCategory::Arithmetic;
            instruction.mnemonic = LM_MNE_SEB;},
        0b11000 => {
            instruction.category = LmInstructionCategory::Arithmetic;
            instruction.mnemonic = LM_MNE_SEH;},
        _ => return false
    };
    
    
    LmDisassembler::reg_format(instruction, 4, 1, 0, 4, 1)
}
pub fn rdhwr(instruction: &mut LmInstruction) -> bool{
    instruction.category = LmInstructionCategory::Move;
    instruction.mnemonic = LM_MNE_RDHWR;
    return LmDisassembler::reg_format(instruction, 4, 0, 1, 4, 0)
}