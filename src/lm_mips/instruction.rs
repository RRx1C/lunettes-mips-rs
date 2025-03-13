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
pub enum LmInstructionCategory{
    NoFunction, BranchJump, Load,
    Store, Move, Priviledge,
    Logical, Arithmetic, Control,
    Trap, MemoryControl, Ejtag,
    InsertExtract, Shift,
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LmInstructionVersion{
    NoVersion, 
    _Mips32, _Mips32R2,
    _Mips64, _Mips64R2
}

#[derive(Clone, Debug, PartialEq)]
pub enum LmInstructionException{
    NoException = 0, _LmIntOverflowExcept = 1, _LmTrapExcept = 2, 
}
// impl LmInstructionException{
//     pub fn or_assign(&mut self, me: Self) ->(){
//         *self |= me;
//     }
// }

#[derive(Clone, Debug, PartialEq)]
pub struct LmInstruction{
    pub address: u64,
    pub mnemonic: &'static str,
    pub machine_code: u32,
    pub string: LmString,
    pub category: LmInstructionCategory,
    pub format: LmInstructionFormat,
    pub exception: LmInstructionException,
    pub address_size: LmAddressSize,
    pub coprocessor: LmCoprocessor,
    pub is_conditional: bool,
    pub is_relative: bool,
    pub is_region: bool,
    pub operand_num: usize,
    pub operand: [LmOperand; 4],    //L'ordre des opérandes suit celui du format en chaîne de caractères 
    pub version: LmInstructionVersion,
}

pub const LM_MNE_NO_MNEMONIC: &str = "no mnemonic"; pub const LM_MNE_J: &str = "j"; pub const LM_MNE_JAL: &str = "jal";
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
pub const LM_MNE_BREAK: &str = "break"; pub const LM_MNE_SYNC: &str = "syn"; pub const LM_MNE_MFHI: &str = "mfhi";
pub const LM_MNE_MTHI: &str = "mthi"; pub const LM_MNE_MFLO: &str = "mflo"; pub const LM_MNE_MTLO: &str = "mtlo";
pub const LM_MNE_MULT: &str = "mult"; pub const LM_MNE_MULTU: &str = "multu"; pub const LM_MNE_DIV: &str = "div";
pub const LM_MNE_DIVU: &str = "divu"; pub const LM_MNE_ADD: &str = "add"; pub const LM_MNE_ADDU: &str = "addu";
pub const LM_MNE_SUB: &str = "sub"; pub const LM_MNE_SUBU: &str = "subu"; pub const LM_MNE_AND: &str = "and";
pub const LM_MNE_OR: &str = "or"; pub const LM_MNE_XOR: &str = "xor"; pub const LM_MNE_NOR: &str = "nor";
pub const LM_MNE_SLT: &str = "SLT"; pub const LM_MNE_SLTU: &str = "sltu"; pub const LM_MNE_TGE: &str = "tge";
pub const LM_MNE_TGEU: &str = "tgeu"; pub const LM_MNE_TLT: &str = "tlt"; pub const LM_MNE_TLTU: &str = "tltu";
pub const LM_MNE_TEQ: &str = "teq"; pub const LM_MNE_TNE: &str = "tne";

//Special2
pub const LM_MNE_MADD: &str = "madd"; pub const LM_MNE_MADDU: &str = "maddu"; pub const LM_MNE_MUL: &str = "mul";
pub const LM_MNE_MSUB: &str = "msub"; pub const LM_MNE_MSUBU: &str = "msubu"; pub const LM_MNE_CLZ: &str = "clz";
pub const LM_MNE_CLO: &str = "clo"; pub const LM_MNE_SDBBP: &str = "sdbbp";
//Special3
pub const LM_MNE_EXT: &str = "ext"; pub const LM_MNE_INS: &str = "ins"; pub const LM_MNE_WSBH: &str = "wsbh";
pub const LM_MNE_SEB: &str = "seb"; pub const LM_MNE_SEH: &str = "seh"; pub const LM_MNE_RDHWR: &str = "rdhwr";