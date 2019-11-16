use alloc::vec::Vec;
use core::hint::unreachable_unchecked;

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[repr(u16)]
#[derive(Clone, Copy, PartialEq)]
pub enum Opcode {
    // Two-operand opcodes (2OP)
    OP2_1 = 1,
    OP2_2 = 2,
    OP2_3 = 3,
    OP2_4 = 4,
    OP2_5 = 5,
    OP2_6 = 6,
    OP2_7 = 7,
    OP2_8 = 8,
    OP2_9 = 9,
    OP2_10 = 10,
    OP2_11 = 11,
    OP2_12 = 12,
    OP2_13 = 13,
    OP2_14 = 14,
    OP2_15 = 15,
    OP2_16 = 16,
    OP2_17 = 17,
    OP2_18 = 18,
    OP2_19 = 19,
    OP2_20 = 20,
    OP2_21 = 21,
    OP2_22 = 22,
    OP2_23 = 23,
    OP2_24 = 24,
    OP2_25 = 25,
    OP2_26 = 26,
    OP2_27 = 27,
    OP2_28 = 28,
    // One-operand opcodes (1OP)
    OP1_128 = 128,
    OP1_129 = 129,
    OP1_130 = 130,
    OP1_131 = 131,
    OP1_132 = 132,
    OP1_133 = 133,
    OP1_134 = 134,
    OP1_135 = 135,
    OP1_136 = 136,
    OP1_137 = 137,
    OP1_138 = 138,
    OP1_139 = 139,
    OP1_140 = 140,
    OP1_141 = 141,
    OP1_142 = 142,
    OP1_143 = 143,
    // Zero-operand opcodes (0OP)
    OP0_176 = 176,
    OP0_177 = 177,
    OP0_178 = 178,
    OP0_179 = 179,
    OP0_180 = 180,
    OP0_181 = 181,
    OP0_182 = 182,
    OP0_183 = 183,
    OP0_184 = 184,
    OP0_185 = 185,
    OP0_186 = 186,
    OP0_187 = 187,
    OP0_188 = 188,
    OP0_189 = 189,
    OP0_191 = 191,
    // Variable-operand opcodes (VAR)
    VAR_224 = 224,
    VAR_225 = 225,
    VAR_226 = 226,
    VAR_227 = 227,
    VAR_228 = 228,
    VAR_229 = 229,
    VAR_230 = 230,
    VAR_231 = 231,
    VAR_232 = 232,
    VAR_233 = 233,
    VAR_234 = 234,
    VAR_235 = 235,
    VAR_236 = 236,
    VAR_237 = 237,
    VAR_238 = 238,
    VAR_239 = 239,
    VAR_240 = 240,
    VAR_241 = 241,
    VAR_242 = 242,
    VAR_243 = 243,
    VAR_244 = 244,
    VAR_245 = 245,
    VAR_246 = 246,
    VAR_247 = 247,
    VAR_248 = 248,
    VAR_249 = 249,
    VAR_250 = 250,
    VAR_251 = 251,
    VAR_252 = 252,
    VAR_253 = 253,
    VAR_254 = 254,
    VAR_255 = 255,
    // Extended opcodes (EXT)
    EXT_1000 = 1000,
    EXT_1001 = 1001,
    EXT_1002 = 1002,
    EXT_1003 = 1003,
    EXT_1004 = 1004,
    EXT_1005 = 1005,
    EXT_1006 = 1006,
    EXT_1007 = 1007,
    EXT_1008 = 1008,
    EXT_1009 = 1009,
    EXT_1010 = 1010,
    EXT_1011 = 1011,
    EXT_1012 = 1012,
    EXT_1013 = 1013,
    EXT_1016 = 1016,
    EXT_1017 = 1017,
    EXT_1018 = 1018,
    EXT_1019 = 1019,
    EXT_1020 = 1020,
    EXT_1021 = 1021,
    EXT_1022 = 1022,
    EXT_1023 = 1023,
    EXT_1024 = 1024,
    EXT_1025 = 1025,
    EXT_1026 = 1026,
    EXT_1027 = 1027,
    EXT_1028 = 1028,
    EXT_1029 = 1029,
}

impl Opcode {
    pub fn from_u16(v: u16) -> Option<Opcode> {
        match v {
            1..=28 | 128..=143 | 176..=191 | 224..=255 | 1000..=1029 => {
                // safe, because range is validated
                Some(unsafe { core::mem::transmute(v) })
            }
            _ => None,
        }
    }
}

#[derive(PartialEq)]
pub enum OperandType {
    Small,
    Large,
    Variable,
    Omitted,
}

impl OperandType {
    pub fn from(bytes: &[u8]) -> Vec<OperandType> {
        bytes
            .iter()
            .fold(Vec::new(), |mut acc, n| {
                acc.push((n & 0b1100_0000) >> 6);
                acc.push((n & 0b0011_0000) >> 4);
                acc.push((n & 0b0000_1100) >> 2);
                acc.push(n & 0b0000_0011);
                acc
            })
            .into_iter()
            .map(|b| match b {
                0b00 => OperandType::Large,
                0b01 => OperandType::Small,
                0b10 => OperandType::Variable,
                0b11 => OperandType::Omitted,
                _ => unsafe { unreachable_unchecked() },
            })
            .take_while(|t| *t != OperandType::Omitted)
            .collect()
    }
}

pub enum Operand {
    Small(u8),
    Large(u16),
    Variable(u8),
}

pub struct Branch {
    pub condition: u16,
    pub address: Option<usize>,
    pub returns: Option<u16>,
}

pub struct Instruction {
    pub addr: usize,
    pub opcode: Opcode,
    pub operands: Vec<Operand>,
    pub store: Option<u8>,
    pub branch: Option<Branch>,
    pub text_position: Option<usize>,
    pub next: usize,
}

impl Instruction {
    pub fn does_store(opcode: Opcode, version: u8) -> bool {
        use self::Opcode::*;

        match opcode {
            // does a store in any version
            OP2_8 | OP2_9 | OP2_15 | OP2_16 | OP2_17 | OP2_18 | OP2_19 | OP2_20 | OP2_21
            | OP2_22 | OP2_23 | OP2_24 | OP2_25 | OP1_129 | OP1_130 | OP1_131 | OP1_132
            | OP1_136 | OP1_142 | VAR_224 | VAR_231 | VAR_236 | VAR_246 | VAR_247 | VAR_248
            | EXT_1000 | EXT_1001 | EXT_1002 | EXT_1003 | EXT_1004 | EXT_1009 | EXT_1010
            | EXT_1019 | EXT_1029 => true,
            // only stores in certain versions
            OP1_143 => version < 5,
            OP0_181 => version == 4, // missing * in spec?
            OP0_182 => version == 4, // missing * in spec?
            OP0_185 => version >= 5,
            VAR_228 => version >= 5,
            VAR_233 => version == 6,
            _ => false,
        }
    }

    pub fn does_branch(opcode: Opcode, version: u8) -> bool {
        use self::Opcode::*;

        match opcode {
            // does a branch in any version
            OP2_1 | OP2_2 | OP2_3 | OP2_4 | OP2_5 | OP2_6 | OP2_7 | OP2_10 | OP1_128 | OP1_129
            | OP1_130 | OP0_189 | OP0_191 | VAR_247 | VAR_255 | EXT_1006 | EXT_1024 | EXT_1027 => {
                true
            }
            // only branches in certain versions
            OP0_181 => version < 4,
            OP0_182 => version < 4,
            _ => false,
        }
    }

    pub fn does_text(opcode: Opcode) -> bool {
        use self::Opcode::*;

        match opcode {
            OP0_178 | OP0_179 => true,
            _ => false,
        }
    }
}

impl Instruction {
    pub fn advances(&self) -> bool {
        use self::Opcode::*;

        // Some instructions never advance to the next instruction:
        // throw, ret, jump, rtrue, rfalse, print_ret, restart, and ret_popped
        match self.opcode {
            OP2_28 | OP1_139 | OP1_140 | OP0_176 | OP0_177 | OP0_179 | OP0_183 | OP0_184
            | OP0_186 => false,
            _ => true,
        }
    }

    pub fn does_call(&self, version: u8) -> bool {
        use self::Opcode::*;

        match self.opcode {
            OP2_25 | OP2_26 | OP1_136 | VAR_224 | VAR_236 | VAR_249 | VAR_250 => true,
            OP1_143 => version >= 4,
            _ => false,
        }
    }

    pub fn should_advance(&self, version: u8) -> bool {
        !self.does_call(version) && self.opcode != Opcode::OP0_181 && self.opcode != Opcode::OP0_182
    }
}
