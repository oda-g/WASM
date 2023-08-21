// SPDX-License-Identifier: MIT
// Copyright(c) 2023 Itsuro Oda
// https://opensource.org/license/mit/

use crate::bytecode::*;

const REVERVED:&str = "reserved";

const B2M: [&str; 256] = [
/*0x00*/ "unreachale", // none
/*0x01*/ "nop", // none
/*0x02*/ "block", // blocktype
/*0x03*/ "loop", // blocktype
/*0x04*/ "if", // blocktype
/*0x05*/ "else", // none
/*0x06*/ REVERVED,
/*0x07*/ REVERVED,
/*0x08*/ REVERVED,
/*0x09*/ REVERVED,
/*0x0a*/ REVERVED,
/*0x0b*/ "end", // none
/*0x0c*/ "br", // index
/*0x0d*/ "br_if", // index
/*0x0e*/ "br_table", // br_table
/*0x0f*/ "return", // none
/*0x10*/ "call", // index
/*0x11*/ "call_indirect", // index, index
/*0x12*/ REVERVED,
/*0x13*/ REVERVED,
/*0x14*/ REVERVED,
/*0x15*/ REVERVED,
/*0x16*/ REVERVED,
/*0x17*/ REVERVED,
/*0x18*/ REVERVED,
/*0x19*/ REVERVED,
/*0x1a*/ "drop", // none
/*0x1b*/ "select", // none
/*0x1c*/ "select", // vec(valuetype)
/*0x1d*/ REVERVED,
/*0x1e*/ REVERVED,
/*0x1f*/ REVERVED,
/*0x20*/ "local.get", // index
/*0x21*/ "local.set", // index
/*0x22*/ "local.tee", // index
/*0x23*/ "global.get", // index
/*0x24*/ "global.set", // index
/*0x25*/ "table.get", // index
/*0x26*/ "table.set", // index
/*0x27*/ REVERVED,
/*0x28*/ "i32.load", // memarg
/*0x29*/ "i64.load", // memarg
/*0x2a*/ "f32.load", // memarg
/*0x2b*/ "f64.load", // memarg
/*0x2c*/ "i32.load8_s", // memarg
/*0x2d*/ "i32.load8_u", // memarg
/*0x2e*/ "i32.load16_s", // memarg
/*0x2f*/ "i32.load16_u", // memarg
/*0x30*/ "i64.load8_s", // memarg
/*0x31*/ "i64.load8_u", // memarg
/*0x32*/ "i64.load16_s", // memarg
/*0x33*/ "i64.load16_u", // memarg
/*0x34*/ "i64.load32_s", // memarg
/*0x35*/ "i64.load32_u", // memarg
/*0x36*/ "i32.store", // memarg
/*0x37*/ "i64.store", // memarg
/*0x38*/ "f32.store", // memarg
/*0x39*/ "f64.store", // memarg
/*0x3a*/ "i32.store8", // memarg
/*0x3b*/ "i32.store16", // memarg
/*0x3c*/ "i64.store8", // memarg
/*0x3d*/ "i64.store16", // memarg
/*0x3e*/ "i64.store32", // memarg
/*0x3f*/ "memory.size", // 0x00
/*0x40*/ "memory.grow", // 0x00
/*0x41*/ "i32.const", // i32
/*0x42*/ "i64.const", // i64
/*0x43*/ "f32.const", // f32
/*0x44*/ "f64.const", // f64
/*0x45*/ "i32.eqz", // none
/*0x46*/ "i32.eq", // none
/*0x47*/ "i32.ne", // none
/*0x48*/ "i32.lt_s", // none
/*0x49*/ "i32.lt_u", // none
/*0x4a*/ "i32.gt_s", // none
/*0x4b*/ "i32.gt_u", // none
/*0x4c*/ "i32.le_s", // none
/*0x4d*/ "i32.le_u", // none
/*0x4e*/ "i32.ge_s", // none
/*0x4f*/ "i32.ge_u", // none
/*0x50*/ "i64.eqz", // none
/*0x51*/ "i64.eq", // none
/*0x52*/ "i64.ne", // none
/*0x53*/ "i64.lt_s", // none
/*0x54*/ "i64.lt_u", // none
/*0x55*/ "i64.gt_s", // none
/*0x56*/ "i64.gt_u", // none
/*0x57*/ "i64.le_s", // none
/*0x58*/ "i64.le_u", // none
/*0x59*/ "i64.ge_s", // none
/*0x5a*/ "i64.ge_u", // none
/*0x5b*/ "f32.eq", // none
/*0x5c*/ "f32.ne", // none
/*0x5d*/ "f32.lt", // none
/*0x5e*/ "f32.gt", // none
/*0x5f*/ "f32.le", // none
/*0x60*/ "f32.ge", // none
/*0x61*/ "f64.eq", // none
/*0x62*/ "f64.ne", // none
/*0x63*/ "f64.lt", // none
/*0x64*/ "f64.gt", // none
/*0x65*/ "f64.le", // none
/*0x66*/ "f64.ge", // none
/*0x67*/ "i32.clz", // none
/*0x68*/ "i32.ctz", // none
/*0x69*/ "i32.popcnt", // none
/*0x6a*/ "i32.add", // none
/*0x6b*/ "i32.sub", // none
/*0x6c*/ "i32.mul", // none
/*0x6d*/ "i32.div_s", // none
/*0x6e*/ "i32.div_u", // none
/*0x6f*/ "i32.rem_s", // none
/*0x70*/ "i32.rem_u", // none
/*0x71*/ "i32.and", // none
/*0x72*/ "i32.or", // none
/*0x73*/ "i32.xor", // none
/*0x74*/ "i32.shl", // none
/*0x75*/ "i32.shr_s", // none
/*0x76*/ "i32.shr_u", // none
/*0x77*/ "i32.rotl", // none
/*0x78*/ "i32.rotr", // none
/*0x79*/ "i64.clz", // none
/*0x7a*/ "i64.ctz", // none
/*0x7b*/ "i64.popcnt", // none
/*0x7c*/ "i64.add", // none
/*0x7d*/ "i64.sub", // none
/*0x7e*/ "i64.mul", // none
/*0x7f*/ "i64.div_s", // none
/*0x80*/ "i64.div_u", // none
/*0x81*/ "i64.rem_s", // none
/*0x82*/ "i64.rem_u", // none
/*0x83*/ "i64.and", // none
/*0x84*/ "i64.or", // none
/*0x85*/ "i64.xor", // none
/*0x86*/ "i64.shl", // none
/*0x87*/ "i64.shr_s", // none
/*0x88*/ "i64.shr_u", // none
/*0x89*/ "i64.rotl", // none
/*0x8a*/ "i64.rotr", // none
/*0x8b*/ "f32.abs", // none
/*0x8c*/ "f32.neg", // none
/*0x8d*/ "f32.ceil", // none
/*0x8e*/ "f32.floor", // none
/*0x8f*/ "f32.trunc", // none
/*0x90*/ "f32.nearest", // none
/*0x91*/ "f32.sqrt", // none
/*0x92*/ "f32.add", // none
/*0x93*/ "f32.sub", // none
/*0x94*/ "f32.mul", // none
/*0x95*/ "f32.div", // none
/*0x96*/ "f32.min", // none
/*0x97*/ "f32.max", // none
/*0x98*/ "f32.copysign", // none
/*0x99*/ "f64.abs", // none
/*0x9a*/ "f64.neg", // none
/*0x9b*/ "f64.ceil", // none
/*0x9c*/ "f64.floor", // none
/*0x9d*/ "f64.trunc", // none
/*0x9e*/ "f64.nearest", // none
/*0x9f*/ "f64.sqrt", // none
/*0xa0*/ "f64.add", // none
/*0xa1*/ "f64.sub", // none
/*0xa2*/ "f64.mul", // none
/*0xa3*/ "f64.div", // none
/*0xa4*/ "f64.min", // none
/*0xa5*/ "f64.max", // none
/*0xa6*/ "f64.copysign", // none
/*0xa7*/ "i32.wrap_i64", // none
/*0xa8*/ "i32.trunc_f32_s", // none
/*0xa9*/ "i32.trunc_f32_u", // none
/*0xaa*/ "i32.trunc_f64_s", // none
/*0xab*/ "i32.trunc_f64_u", // none
/*0xac*/ "i64.extend_i32_s", // none
/*0xad*/ "i64.extend_i32_u", // none
/*0xae*/ "i64.trunc_f32_s", // none
/*0xaf*/ "i64.trunc_f32_u", // none
/*0xb0*/ "i64.trunc_f64_s", // none
/*0xb1*/ "i64.trunc_f64_u", // none
/*0xb2*/ "f32.convert_i32_s", // none
/*0xb3*/ "f32.convert_i32_u", // none
/*0xb4*/ "f32.convert_i64_s", // none
/*0xb5*/ "f32.convert_i64_u", // none
/*0xb6*/ "f32.demote_f64", // none
/*0xb7*/ "f64.convert_i32_s", // none
/*0xb8*/ "f64.convert_i32_u", // none
/*0xb9*/ "f64.convert_i64_s", // none
/*0xba*/ "f64.convert_i64_u", // none
/*0xbb*/ "f64.promote_f32", // none
/*0xbc*/ "i32.reinterpret_f32", // none
/*0xbd*/ "i64.reinterpret_f64", // none
/*0xbe*/ "f32.reinterpret_i32", // none
/*0xbf*/ "f64.reinterpret_i64", // none
/*0xc0*/ "i32.extend8_s", // none
/*0xc1*/ "i32.extend16_s", // none
/*0xc2*/ "i64.extend8_s", // none
/*0xc3*/ "i64.extend16_s", // none
/*0xc4*/ "i64.extend32_s", // none
/*0xc5*/ REVERVED,
/*0xc6*/ REVERVED,
/*0xc7*/ REVERVED,
/*0xc8*/ REVERVED,
/*0xc9*/ REVERVED,
/*0xca*/ REVERVED,
/*0xcb*/ REVERVED,
/*0xcc*/ REVERVED,
/*0xcd*/ REVERVED,
/*0xce*/ REVERVED,
/*0xcf*/ REVERVED,
/*0xd0*/ "ref.null", // valuetype
/*0xd1*/ "ref.is_null", // none
/*0xd2*/ "ref.func", // index
/*0xd3*/ REVERVED,
/*0xd4*/ REVERVED,
/*0xd5*/ REVERVED,
/*0xd6*/ REVERVED,
/*0xd7*/ REVERVED,
/*0xd8*/ REVERVED,
/*0xd9*/ REVERVED,
/*0xda*/ REVERVED,
/*0xdb*/ REVERVED,
/*0xdc*/ REVERVED,
/*0xdd*/ REVERVED,
/*0xde*/ REVERVED,
/*0xdf*/ REVERVED,
/*0xe0*/ REVERVED,
/*0xe1*/ REVERVED,
/*0xe2*/ REVERVED,
/*0xe3*/ REVERVED,
/*0xe4*/ REVERVED,
/*0xe5*/ REVERVED,
/*0xe6*/ REVERVED,
/*0xe7*/ REVERVED,
/*0xe8*/ REVERVED,
/*0xe9*/ REVERVED,
/*0xea*/ REVERVED,
/*0xeb*/ REVERVED,
/*0xec*/ REVERVED,
/*0xed*/ REVERVED,
/*0xee*/ REVERVED,
/*0xef*/ REVERVED,
/*0xf0*/ REVERVED,
/*0xf1*/ REVERVED,
/*0xf2*/ REVERVED,
/*0xf3*/ REVERVED,
/*0xf4*/ REVERVED,
/*0xf5*/ REVERVED,
/*0xf6*/ REVERVED,
/*0xf7*/ REVERVED,
/*0xf8*/ REVERVED,
/*0xf9*/ REVERVED,
/*0xfa*/ REVERVED,
/*0xfb*/ REVERVED,
/*0xfc*/ "", // see another table
/*0xfd*/ "", // see another table
/*0xfe*/ "", // not defined
/*0xff*/ "", // not defined
];

const B2M_FC: [&str; 18] = [
/*0*/ "i32.trunc_sat_f32_s", //none
/*1*/ "i32.trunc_sat_f32_u", //none
/*2*/ "i32.trunc_sat_f64_s", //none
/*3*/ "i32.trunc_sat_f64_u", //none
/*4*/ "i64.trunc_sat_f32_s", //none
/*5*/ "i64.trunc_sat_f32_u", //none
/*6*/ "i64.trunc_sat_f64_s", //none
/*7*/ "i64.trunc_sat_f64_u", //none
/*8*/ "memory.init", // index, 0x00
/*9*/ "data.drop", // index
/*10*/ "memory.copy", // 0x00, 0x00
/*11*/ "memory.fill", // 0x00
/*12*/ "table.init", // index, index
/*13*/ "elem.drop", // index
/*14*/ "table.copy", // index, index
/*15*/ "table.grow", // index
/*16*/ "table.size", // index
/*17*/ "table.fill", // index
];

pub enum BlockType {
    Empty,
    Valtype(u8),
    TypeIndex(u32),
}

pub struct BrTable {
    labels: Vec<u32>,
    default: u32,
}

pub struct Memarg {
    align: u32,
    offset: u32,
}

pub enum Operand {
    None,
    BlockType(BlockType),
    Index(u32),
    Index2(u32, u32),
    BrTable(BrTable),
    VecValtype(Vec<u8>),
    Memarg(Memarg),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Valtype(u8),
}

pub struct Inst {
    pub op_code: u8,
    pub sub_op: u32,
    pub operand: Operand,
    pub level: i32,
}

fn fmt_valuetype(t: &u8) -> String {
    match t {
        0x7f => "i32".to_string(),
        0x7e => "i64".to_string(),
        0x7d => "f32".to_string(),
        0x7c => "f64".to_string(),
        0x7b => "v128".to_string(),
        0x70 => "funcref".to_string(),
        0x6f => "externref".to_string(),
        _ => "".to_string()
    }
}

impl Inst {
    fn get_mnemonic(&self) -> String {
        if self.op_code == 0xfc {
            B2M_FC[self.sub_op as usize].to_string()
        } else {
            B2M[self.op_code as usize].to_string()
        }
    }

    pub fn print(&self) {
        let l = if self.level < 0 {0} else {self.level};
        for _ in 0..l {
            print!("  ");
        }
        print!("{}", self.get_mnemonic());
        match &self.operand {
            Operand::BlockType(br_type) => {
                match br_type {
                    BlockType::Empty => (),
                    BlockType::Valtype(v) => print!(" {}", fmt_valuetype(v)),
                    BlockType::TypeIndex(idx) => print!(" {}", idx),
                }
            },
            Operand::Index(idx) => print!(" {}", idx),
            Operand::Index2(idx1, idx2) => print!(" {} {}", idx1, idx2),
            Operand::BrTable(br_table) => {
                print!(" [");
                for l in &br_table.labels {
                    print!("{} ", l);
                }
                print!("] {}", br_table.default);
            },
            Operand::VecValtype(values) => {
                print!(" [");
                for v in values {
                    print!("{} ", fmt_valuetype(v));
                }
                print!("]");
            },
            Operand::Memarg(memarg) => print!(" {} {}", memarg.align, memarg.offset),
            Operand::I32(num) => print!(" {}", num),
            Operand::I64(num) => print!(" {}", num),
            Operand::F32(num) => print!(" {}", num),
            Operand::F64(num) => print!(" {}", num),
            Operand::Valtype(v) => print!(" {}", fmt_valuetype(v)),
            _ => ()
        }
        println!("");
    }
}

pub fn get_insts(buf: &mut ByteCodeBuff) -> Vec<Inst> {
    _get_insts(buf, 0)
}

fn _get_insts(buf: &mut ByteCodeBuff, level: i32) -> Vec<Inst> {
    let mut insts: Vec<Inst> = Vec::new();

    loop {
        let code = buf.get_byte();

        match code {
            // blocktype
            0x02 | 0x03 | 0x04 => {
                let cur = buf.get_cur();
                let block_type = buf.get_byte();
                match block_type {
                    0x40 => {
                        let inst = Inst {
                            op_code: code,
                            sub_op: 0,
                            operand: Operand::BlockType(BlockType::Empty),
                            level: level,
                        };
                        insts.push(inst);
                    },
                    0x6f | 0x70 | 0x7b..=0x7f => {
                        let inst = Inst {
                            op_code: code,
                            sub_op: 0,
                            operand: Operand::BlockType(BlockType::Valtype(block_type)),
                            level: level,
                        };
                        insts.push(inst);
                    },
                    _ => {
                        // s33 but same as i32
                        buf.set_cur(cur); // put back 1 byte
                        let idx = buf.get_i32();
                        let inst = Inst {
                            op_code: code,
                            sub_op: 0,
                            operand: Operand::BlockType(BlockType::TypeIndex(idx as u32)),
                            level: level,
                        };
                        insts.push(inst);
                    }
                }
                let block_insts = _get_insts(buf, level + 1);
                insts.extend(block_insts);
            },
            // else
            0x05 => {
                let inst = Inst {
                    op_code: code,
                    sub_op: 0,
                    operand: Operand::None,
                    level: level - 1,
                };
                insts.push(inst);
            },
            // end
            0x0b => {
                let inst = Inst {
                    op_code: code,
                    sub_op: 0,
                    operand: Operand::None,
                    level: level - 1,
                };
                insts.push(inst);
                break;
            },
            // br_table
            0x0e => {
                let n = buf.get_u32();
                let mut labels: Vec<u32> = Vec::new();
                for _ in 0..n {
                    labels.push(buf.get_u32());
                }
                let default = buf.get_u32();
                let br_table = BrTable {
                    labels: labels,
                    default: default
                };
                let inst = Inst {
                    op_code: code,
                    sub_op: 0,
                    operand: Operand::BrTable(br_table),
                    level: level,
                };
                insts.push(inst);
            },
            // operand: index
            0x0c | 0x0d | 0x10 | 0x20..=0x26 | 0x3f | 0x40 | 0xd2 => {
                let inst = Inst {
                    op_code: code,
                    sub_op: 0,
                    operand: Operand::Index(buf.get_u32()),
                    level: level,
                };
                insts.push(inst);
                // 0x3f | 0x40 0x00
            },
            // operand: index, index
            0x11 => {
                let idx2 = buf.get_u32(); // typeidx
                let idx1 = buf.get_u32(); // tableidx
                let inst = Inst {
                    op_code: code,
                    sub_op: 0,
                    operand: Operand::Index2(idx1, idx2),
                    level: level,
                };
                insts.push(inst);
            },
            // operand: i32
            0x41 => {
                let inst = Inst {
                    op_code: code,
                    sub_op: 0,
                    operand: Operand::I32(buf.get_i32()),
                    level: level,
                };
                insts.push(inst);
            },
            // operand: i64
            0x42 => {
                let inst = Inst {
                    op_code: code,
                    sub_op: 0,
                    operand: Operand::I64(buf.get_i64()),
                    level: level,
                };
                insts.push(inst);
            },
            // operand: f32
            0x43 => {
                let inst = Inst {
                    op_code: code,
                    sub_op: 0,
                    operand: Operand::F32(buf.get_f32()),
                    level: level,
                };
                insts.push(inst);
            },
            // operand: f64
            0x44 => {
                let inst = Inst {
                    op_code: code,
                    sub_op: 0,
                    operand: Operand::F64(buf.get_f64()),
                    level: level,
                };
                insts.push(inst);
            },
            // operand: vec(valtype)
            0x1c => {
                let n = buf.get_u32();
                let mut values = Vec::new();
                for _ in 0..n {
                    values.push(buf.get_byte());
                }
                let inst = Inst {
                    op_code: code,
                    sub_op: 0,
                    operand: Operand::VecValtype(values),
                    level: level,
                };
                insts.push(inst);
            },
            // operand: memarg
            0x28..=0x3e => {
                let align = buf.get_u32();
                let offset = buf.get_u32();
                let memarg = Memarg {
                    align,
                    offset,
                };
                let inst = Inst {
                    op_code: code,
                    sub_op: 0,
                    operand: Operand::Memarg(memarg),
                    level: level,
                };
                insts.push(inst);
            },
            // operand: valtype
            0xd0 => {
                let valuetype = buf.get_byte();
                let inst = Inst {
                    op_code: code,
                    sub_op: 0,
                    operand: Operand::Valtype(valuetype),
                    level: level,
                };
                insts.push(inst);
            },
            // no operand
            0x00 | 0x01 | 0x0f | 0x1a | 0x1b | 0x45..=0xc4 | 0xd1 => {
                let inst = Inst {
                    op_code: code,
                    sub_op: 0,
                    operand: Operand::None,
                    level: level,
                };
                insts.push(inst);
            },
            // FC
            0xfc => {
                let sub_op = buf.get_u32();
                match sub_op {
                    0..=7 => {
                        let inst = Inst {
                            op_code: code,
                            sub_op: sub_op,
                            operand: Operand::None,
                            level: level,
                        };
                        insts.push(inst);
                    },
                    8 | 10 | 12 | 14 => {
                        let idx1 = buf.get_u32();
                        let idx2 = buf.get_u32();
                        let inst = Inst {
                            op_code: code,
                            sub_op: sub_op,
                            operand: Operand::Index2(idx1, idx2),
                            level: level,
                        };
                        insts.push(inst);
                        // 8: index,0x00
                        // 10: 0x00, 0x00
                    },
                    9 | 11 | 13 | 15 | 16 | 17 => {
                        let idx = buf.get_u32();
                        let inst = Inst {
                            op_code: code,
                            sub_op: sub_op,
                            operand: Operand::Index(idx),
                            level: level,
                        };
                        insts.push(inst);
                        // 11: 0x00
                    },
                    _ => {
                        panic!("illegal sub_op: {}", sub_op);
                    }
                }
            },
            // FD
            0xfd => {
                panic!("0xFD not suported at the moment");
            },
            _ => {
                panic!("illegal op: {}", code);
            }
        }
    }

    insts
}
