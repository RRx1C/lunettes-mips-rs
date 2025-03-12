//Author: RR28
//Discord: rrx1c
//Jabber: rrx1c@jabber.fr
//Github profile: https://github.com/RRx1C
//Link to repo: https://github.com/RRx1C/lunettes-mips-rs
use super::LmAddressSize;
use super::operands::*;
use super::utils::string::LmString;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LmInstructionFormat{
    NoFormat, Imm, Reg, Jump, Other
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LmInstructionFunction{
    NoFunction, Computational, BranchJump,
    LoadStore, Miscellaneous,
    _Coprocessor
}

// pub struct LmJumpFields{
// }
// pub struct LmRegisterFields{
// }
// pub struct LmImmediateFields{
// }
// pub enum LmInstructionField{
//     LmJumpFields(LmJumpFields),
//     LmRegisterFields(LmRegisterFields),
//     LmImmediateFields(LmImmediateFields)
// }

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LmCoprocessor{
    NoCoprocessor,
    Cpu, Cp0, Cp1, Cp2, Cp1x
}

//On peut s'en servir en tant qu'index dans l'array qui regroupe tous les mnemonics pour trouver le bon mnemonic,
//peut aussi servir pour reconnaître l'instruction sans avoir à comparer le mnemonics 
//avec une chaîne de caractère ce qui peut ralentir la recherche d'une instruction précise.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LmMnemonicId {
    NoMnemonic, J, Jal, Beq, Bne, Blez, Bgtz, Addi, Addiu, Slti, Sltiu, Andi,
    Ori, Xori, Lui, Beql, Bnel, Blezl, Bgtzl, Jalx, Lb, Lh, Lwl, Lw, Lbu, Lhu,
    Lwr, Sb, Sh, Swl, Sw, Swr, Cache, Ll, Lwc1, Lwc2, Pref, Ldc1, Ldc2, Sc,
    Swc1, Swc2, Sdc1, Sdc2, 
    //Special
    Nop, Sll, Sra, Sllv, Srav, Jr, Jrhb, Jalr, Jalrhb, Movz, Movn,
    Syscall, Break, Sync,
    //Special2
    Madd, Maddu, Mul, Msub, Msubu, Clz, Clo, Sdbbp,
    //Special3
    Ext, Ins, Wsbh, Seb, Seh, Rdhwr,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LmInstructionVersion{
    NoVersion, 
    _Mips32, _Mips32R2,
    _Mips64, _MipsR2
}

enum _LmInstructionException{
    LmIntOverflowExcept, LmTrapExcept, 
}

#[derive(Clone, Debug, PartialEq)]
pub struct LmInstruction{
    pub address: u64,
    pub machine_code: u32,
    pub string: LmString,
    pub mnemonic_id: LmMnemonicId,
    pub function: LmInstructionFunction,
    pub format: LmInstructionFormat,
    pub address_size: LmAddressSize,
    pub coprocessor: LmCoprocessor,
    pub is_conditional: bool,
    pub is_relative: bool,
    pub is_region: bool,
    pub operand_num: usize,
    pub operand: [LmOperand; 4],    //L'ordre des opérandes suit celui du format en lettre 
    pub version: LmInstructionVersion,
}

impl LmInstruction{
    pub fn get_memonic(mnemonic_id: LmMnemonicId) -> &'static str{
        static MNEMONIC_TABLE: [&str; 72] = [
            LM_MNE_NO_MNEMONIC, LM_MNE_J, LM_MNE_JAL, LM_MNE_BEQ, LM_MNE_BNE, LM_MNE_BLEZ, LM_MNE_BGTZ, LM_MNE_ADDI, 
            LM_MNE_ADDIU, LM_MNE_SLTI, LM_MNE_SLTIU, LM_MNE_ANDI, LM_MNE_ORI, LM_MNE_XORI, LM_MNE_LUI, LM_MNE_BEQL, 
            LM_MNE_BNEL, LM_MNE_BLEZL, LM_MNE_BGTZL, LM_MNE_JALX, LM_MNE_LB, LM_MNE_LH, LM_MNE_LWL, LM_MNE_LW, 
            LM_MNE_LBU, LM_MNE_LHU, LM_MNE_LWR, LM_MNE_SB, LM_MNE_SH, LM_MNE_SWL, LM_MNE_SW, LM_MNE_SWR, 
            LM_MNE_CACHE, LM_MNE_LL, LM_MNE_LWC1, LM_MNE_LWC2, LM_MNE_PREF, LM_MNE_LDC1, LM_MNE_LDC2, LM_MNE_SC, 
            LM_MNE_SWC1, LM_MNE_SWC2, LM_MNE_SDC1, LM_MNE_SDC2, 
            //Special
            LM_MNE_NOP, LM_MNE_SLL, LM_MNE_SRA, LM_MNE_SLLV,LM_MNE_SRAV,
            LM_MNE_JR, LM_MNE_JRHB, LM_MNE_JALR, LM_MNE_JALRHB, LM_MNE_MOVZ, LM_MNE_MOVN, LM_MNE_SYSCALL, LM_MNE_BREAK,
            LM_MNE_SYNC,
            //Special2
            LM_MNE_MADD, LM_MNE_MADDU, LM_MNE_MUL, LM_MNE_MSUB, LM_MNE_MSUBU, LM_MNE_CLZ, LM_MNE_CLO, LM_MNE_SDBBP,
            //Special3
            LM_MNE_EXT, LM_MNE_INS, LM_MNE_WSBH, LM_MNE_SEB, LM_MNE_SEH, LM_MNE_RDHWR,
        ];
        MNEMONIC_TABLE[mnemonic_id as usize]
    }
}

pub const LM_MNE_NO_MNEMONIC: &str = "error"; pub const LM_MNE_J: &str = "j"; pub const LM_MNE_JAL: &str = "jal";
pub const LM_MNE_BEQ: &str = "beq"; pub const LM_MNE_BNE: &str = "bne"; pub const LM_MNE_BLEZ: &str = "blez";
pub const LM_MNE_BGTZ: &str = "bgtz"; pub const LM_MNE_ADDI: &str = "addi"; pub const LM_MNE_ADDIU: &str = "addiu";
pub const LM_MNE_SLTI: &str = "slti"; pub const LM_MNE_SLTIU: &str = "sltiu"; pub const LM_MNE_ANDI: &str = "andi";
pub const LM_MNE_ORI: &str = "ori"; pub const LM_MNE_XORI: &str = "xori"; pub const LM_MNE_LUI: &str = "lui";
pub const LM_MNE_BEQL: &str = "beql"; pub const LM_MNE_BNEL: &str = "bnel"; pub const LM_MNE_BLEZL: &str = "blezl";
pub const LM_MNE_BGTZL: &str = "bgtzl"; pub const LM_MNE_JALX: &str = "jalx"; pub const LM_MNE_LB: &str = "lb";
pub const LM_MNE_LH: &str = "lh"; pub const LM_MNE_LWL: &str = "lwl"; pub const LM_MNE_LW: &str = "lw";
pub const LM_MNE_LBU: &str = "lbu"; pub const LM_MNE_LHU: &str = "lhu"; pub const LM_MNE_LWR: &str = "lwr";
pub const LM_MNE_SB: &str = "sb"; pub const LM_MNE_SH: &str = "sh"; pub const LM_MNE_SWL: &str = "swl";
pub const LM_MNE_SW: &str = "sw"; pub const LM_MNE_SWR: &str = "swr"; pub const LM_MNE_CACHE: &str = "cache";
pub const LM_MNE_LL: &str = "ll"; pub const LM_MNE_LWC1: &str = "lwc1"; pub const LM_MNE_LWC2: &str = "lwc2";
pub const LM_MNE_PREF: &str = "pref"; pub const LM_MNE_LDC1: &str = "ldc1"; pub const LM_MNE_LDC2: &str = "ldc2";
pub const LM_MNE_SC: &str = "sc"; pub const LM_MNE_SWC1: &str = "swc1"; pub const LM_MNE_SWC2: &str = "swc2";
pub const LM_MNE_SDC1: &str = "sdc1"; pub const LM_MNE_SDC2: &str = "sdc2";
//Special
pub const LM_MNE_NOP: &str = "nop"; pub const LM_MNE_SLL: &str = "sll"; pub const LM_MNE_SRA: &str = "sra";
pub const LM_MNE_SLLV: &str = "sllv"; pub const LM_MNE_SRAV: &str = "srav"; pub const LM_MNE_JR: &str = "jr";
pub const LM_MNE_JRHB: &str = "jr.hb"; pub const LM_MNE_JALR: &str = "jalr"; pub const LM_MNE_JALRHB: &str = "jalr.hb";
pub const LM_MNE_MOVZ: &str = "movz"; pub const LM_MNE_MOVN: &str = "movn"; pub const LM_MNE_SYSCALL: &str = "syscall";
pub const LM_MNE_BREAK: &str = "break"; pub const LM_MNE_SYNC: &str = "syn";
//Special2
pub const LM_MNE_MADD: &str = "madd"; pub const LM_MNE_MADDU: &str = "maddu"; pub const LM_MNE_MUL: &str = "mul";
pub const LM_MNE_MSUB: &str = "msub"; pub const LM_MNE_MSUBU: &str = "msubu"; pub const LM_MNE_CLZ: &str = "clz";
pub const LM_MNE_CLO: &str = "clo"; pub const LM_MNE_SDBBP: &str = "sdbbp";
//Special3
pub const LM_MNE_EXT: &str = "ext"; pub const LM_MNE_INS: &str = "ins"; pub const LM_MNE_WSBH: &str = "wsbh";
pub const LM_MNE_SEB: &str = "seb"; pub const LM_MNE_SEH: &str = "seh"; pub const LM_MNE_RDHWR: &str = "rdhwr";