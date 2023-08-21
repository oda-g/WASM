// SPDX-License-Identifier: MIT
// Copyright(c) 2023 Itsuro Oda
// https://opensource.org/license/mit/

#![allow(dead_code)]

use std::fmt;

use crate::bytecode::*;
use crate::module::*;
use crate::inst::*;

pub struct Store {
    memory: Vec<u8>,
    // table,
    // global,
}

pub fn make_store(module: &Module) -> Store {
    let mut memory = Vec::new();
    //TODO: allocate memory and initialize from data segment
    Store {memory,}
}

#[derive(Debug, Clone)]
pub enum Value {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    //V128(v128), not supported yet
    //Index(u32), funcref
    //???, externref
    // stack only
    Label(Label),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::I32(n) => write!(f, "{}", n),
            Value::I64(n) => write!(f, "{}", n),
            Value::F32(n) => write!(f, "{}", n),
            Value::F64(n) => write!(f, "{}", n),
            _ => write!(f, ""),
        }
    }
}

pub struct Stack {
    stack: Vec<Value>,
}

impl Stack {
    fn new() -> Stack {
        Stack {stack: Vec::new(),}
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    fn dup_top(&mut self) {
        let item = self.stack.pop().unwrap();
        self.stack.push(item.clone());
        self.stack.push(item);
    }

    fn peek_top(&self) -> Value {
        let item = &self.stack[self.stack.len() - 1];
        item.clone()
    }

    fn pop_i32(&mut self) -> Result<i32, String> {
        let item = self.stack.pop().unwrap();
        if let Value::I32(n) = item {
            return Ok(n);
        }
        Err("stack value expect i32".to_string())
    }

    fn push_i32(&mut self, n: i32) {
        self.stack.push(Value::I32(n));
    }

    fn pop_i64(&mut self) -> Result<i64, String> {
        let item = self.stack.pop().unwrap();
        if let Value::I64(n) = item {
            return Ok(n);
        }
        Err("stack value expect i64".to_string())
    }

    fn push_i64(&mut self, n: i64) {
        self.stack.push(Value::I64(n));
    }

    fn pop_f32(&mut self) -> Result<f32, String> {
        let item = self.stack.pop().unwrap();
        if let Value::F32(n) = item {
            return Ok(n);
        }
        Err("stack value expect f32".to_string())
    }

    fn push_f32(&mut self, n: f32) {
        self.stack.push(Value::F32(n));
    }

    fn pop_f64(&mut self) -> Result<f64, String> {
        let item = self.stack.pop().unwrap();
        if let Value::F64(n) = item {
            return Ok(n);
        }
        Err("stack value expect f64".to_string())
    }

    fn push_f64(&mut self, n: f64) {
        self.stack.push(Value::F64(n));
    }

    fn pop_label(&mut self) -> Result<Label, String> {
        let item = self.stack.pop().unwrap();
        if let Value::Label(label) = item {
            return Ok(label);
        }
        Err("stack value expect label".to_string())
    }

    fn pop_nth_label(&mut self, n: u32) -> Label {
        let mut num = n + 1;
        loop {
            let item = self.stack.pop().unwrap();
            match item {
                Value::Label(label) => {
                    num -= 1;
                    if num == 0 {
                        return label;
                    }
                },
                _ => (),
            }
        }
    }

    fn push_label(&mut self, label: Label) {
        self.stack.push(Value::Label(label));
    }
}

#[derive(Clone)]
pub struct Frame {
    func_idx: usize,
    locals: Vec<Value>,
    ip: usize,
}

impl Frame {
    fn next(&mut self) {
        self.ip += 1;
    }

    fn set_ip(&mut self, i: usize) {
        self.ip = i;
    }
}

#[derive(Debug, Clone)]
pub struct Label {
    arity: usize,
    next_ip: usize,
}

pub enum Function {
    Import(ImportFunc),
    Local(LocalFunc),
}

pub struct ImportFunc {
    pub name: String,
    pub ft: Functype,
}

pub struct LocalFunc {
    pub ft: Functype,
    pub locals: Vec<Valtype>,
    pub insts: Vec<Inst>,
}

impl LocalFunc {
    pub fn show_insts(&self) {
        for inst in &self.insts {
            inst.print();
        }
    }
}

pub fn exec_func(args: &[&str], module: &Module, store: &mut Store) -> Result<(), String> {
    if args.len() == 0 {
        return Err("need funcidx".to_string());
    }
    let idx: usize = match args[0].parse() {
        Ok(n) => n,
        Err(err) => return Err(format!("{}", err)),
    };
    if idx >= module.num_funcs() {
        return Err(format!("funcidx {} out of range. must be less than {}", idx,
                           module.num_funcs()));
    }
    if module.is_import_func(idx) {
        return Err("import function is not supported".to_string());
    }

    let func = module.get_local_func(idx);
    if func.ft.input.0.len() != args.len() - 1 {
        return Err(format!("need {} args", func.ft.input.0.len()));
    }

    let mut stack = Stack::new();

    for (i, v) in func.ft.input.0.iter().enumerate() {
        match &v.0 {
            0x7f => {
                let a: i32 = match args[i + 1].parse() {
                    Ok(n) => n,
                    Err(err) => return Err(format!("{}", err)),
                };
                stack.push_i32(a);
            },
            0x7e => {
                let a: i64 = match args[i + 1].parse() {
                    Ok(n) => n,
                    Err(err) => return Err(format!("{}", err)),
                };
                stack.push_i64(a);
            },
            0x7d => {
                let a: f32 = match args[i + 1].parse() {
                    Ok(n) => n,
                    Err(err) => return Err(format!("{}", err)),
                };
                stack.push_f32(a);
            }
            0x7c => {
                let a: f64 = match args[i + 1].parse() {
                    Ok(n) => n,
                    Err(err) => return Err(format!("{}", err)),
                };
                stack.push_f64(a);
            },
            _ => {
                return Err("not supported".to_string());
            },
        }
    }

    call_func(idx, module, store, &mut stack)?;

    if func.ft.output.0.len() == 1 {
        println!("result: {}", stack.pop());
    }
    Ok(())
}

fn call_func(idx: usize, module: &Module, store: &Store, stack: &mut Stack) -> Result<(), String> {
    if module.is_import_func(idx) {
        return Err("import function is not supported".to_string());
    }
    let func = module.get_local_func(idx);

    let mut locals = Vec::new(); // Frame locals
    // set default
    for v in func.ft.input.0.iter() {
       match &v.0 {
           0x7f => locals.push(Value::I32(0)),
           0x7e => locals.push(Value::I64(0)),
           0x7d => locals.push(Value::F32(0.0)),
           0x7c => locals.push(Value::F64(0.0)),
           _ => {
               return Err("not supported".to_string());
           },
        }
    }
    for v in &func.locals {
       match &v.0 {
           0x7f => locals.push(Value::I32(0)),
           0x7e => locals.push(Value::I64(0)),
           0x7d => locals.push(Value::F32(0.0)),
           0x7c => locals.push(Value::F64(0.0)),
           _ => {
               return Err("not supported".to_string());
           },
       }
    }

    let num_input = func.ft.input.0.len();
    for i in 0..num_input {
        let j = num_input - i - 1;
        let v = &func.ft.input.0[j];
        match &v.0 {
            0x7f => {
                let a = stack.pop_i32()?;
                locals[j] = Value::I32(a);
            },
            0x7e => {
                let a = stack.pop_i64()?;
                locals[j] = Value::I64(a);
            },
            0x7d => {
                let a = stack.pop_f32()?;
                locals[j] = Value::F32(a);
            }
            0x7c => {
                let a = stack.pop_f64()?;
                locals[j] = Value::F64(a);
            },
            _ => {
                return Err("not supported".to_string());
            },
        }
    }

    let mut frame = Frame {
        func_idx: idx,
        locals: locals,
        ip: 0,
    };

    // execute function
    loop {
        let inst = &func.insts[frame.ip];
        //println!("{}: {:?} {:?}", frame.ip, &frame.locals, &self.stack);
        //inst.print();
        match inst.op_code {
            0x00 => { // unreachable
                return Err("trap: unreachable".to_string());
            },
            0x02 => { // block
                exec_02(func, &inst, &mut frame, stack)?;
            },
            0x0b => { // end
                if inst.level < 0 {
                    //TODO: suport only one return value
                    let item = stack.peek_top(); // must be return value
                    println!("function[{}] done: {:?}", frame.func_idx, item);
                    break;
                }
                exec_0b(&inst, &mut frame, stack)?;
            },
            0x0d => { // br_if
                let t = stack.pop_i32()?;
                if t == 0 {
                    frame.next();
                } else {
                    exec_0c(&inst, &mut frame, stack)?;
                }
            },
            0x0f => { // return
                let item = stack.peek_top(); // must be return value
                println!("function[{}] done: {:?}", frame.func_idx, item);
                break;
            },
            0x10 => { // call
                if let Operand::Index(idx) = &inst.operand {
                    call_func(*idx as usize, module, store, stack)?;
                    frame.next();
                }
            },
            _ => {
                let e_fn = EXEC_TABLE[inst.op_code as usize];
                e_fn(&inst, &mut frame, stack)?;
            },
        }
    }
    Ok(())
}

// block
fn exec_02(func: &LocalFunc, inst: &Inst, frame: &mut Frame, stack: &mut Stack) -> Result<(), String> {
    //TODO: suport EMPTY only now
    let mut next_ip = frame.ip + 1;
    loop {
        let it = &func.insts[next_ip];
        if it.op_code == 0x0b && it.level == inst.level {
             break;
        }
        next_ip += 1;
    }
    next_ip += 1;
    stack.push_label(Label {
         arity: 0,
         next_ip,
    });
    frame.next();
    Ok(())
}

// loop
fn exec_03(_inst: &Inst, frame: &mut Frame, stack: &mut Stack) -> Result<(), String> {
    //TODO: suport EMPTY only now
    stack.push_label(Label {
        arity: 0,
        next_ip: frame.ip,
    });
    frame.next();
    Ok(())
}

// end
fn exec_0b(inst: &Inst, frame: &mut Frame, stack: &mut Stack) -> Result<(), String> {
    //NOTE: this is called when loop end
    assert!(inst.level >= 0);
    //TODO: return value
    stack.pop_label()?;
    frame.next();
    Ok(())
}

// br
fn exec_0c(inst: &Inst, frame: &mut Frame, stack: &mut Stack) -> Result<(), String> {
    if let Operand::Index(n) = inst.operand {
        let label = stack.pop_nth_label(n);
        frame.set_ip(label.next_ip);
    }
    Ok(())
}

// local.get
fn exec_20(inst: &Inst, frame: &mut Frame, stack: &mut Stack) -> Result<(), String> {
    if let Operand::Index(idx) = inst.operand {
        let local_value = &frame.locals[idx as usize];
        match local_value {
            Value::I32(n) => {
                stack.push_i32(*n);
            },
            Value::I64(n) => {
                stack.push_i64(*n);
            },
            Value::F32(n) => {
                stack.push_f32(*n);
            },
            Value::F64(n) => {
                stack.push_f64(*n);
            },
            _ => return Err("trap: not supported yet".to_string()),
        }
    }
    frame.next();
    Ok(())
}

// local.set
fn exec_21(inst: &Inst, frame: &mut Frame, stack: &mut Stack) -> Result<(), String> {
    if let Operand::Index(idx) = inst.operand {
        let local_value = &frame.locals[idx as usize];
        match local_value {
            Value::I32(_) => {
                let a = stack.pop_i32()?;
                frame.locals[idx as usize] = Value::I32(a);
            },
            Value::I64(_) => {
                let a = stack.pop_i64()?;
                frame.locals[idx as usize] = Value::I64(a);
            },
            Value::F32(_) => {
                let a = stack.pop_f32()?;
                frame.locals[idx as usize] = Value::F32(a);
            },
            Value::F64(_) => {
                let a = stack.pop_f64()?;
                frame.locals[idx as usize] = Value::F64(a);
            },
            _ => return Err("trap: not supported yet".to_string()),
        }
    }
    frame.next();
    Ok(())
}

// local.tee
fn exec_22(inst: &Inst, frame: &mut Frame, stack: &mut Stack) -> Result<(), String> {
    stack.dup_top();
    exec_21(inst, frame, stack)
}

// i32.const
fn exec_41(inst: &Inst, frame: &mut Frame, stack: &mut Stack) -> Result<(), String> {
    if let Operand::I32(n) = inst.operand {
        stack.push_i32(n);
    }
    frame.next();
    Ok(())
}

// i32.eqz
fn exec_45(_inst: &Inst, frame: &mut Frame, stack: &mut Stack) -> Result<(), String> {
    let n = stack.pop_i32()?;
    stack.push_i32((n == 0) as i32);
    frame.next();
    Ok(())
}

// i32 binop bool
fn i32_binop_bool(inst: &Inst, frame: &mut Frame, stack: &mut Stack) -> Result<(), String> {
    let n2 = stack.pop_i32()?;
    let n1 = stack.pop_i32()?;
    let r: bool = match inst.op_code {
        0x46 => n1 == n2, // eq
        0x47 => n1 != n2, // ne
        0x48 => n1 < n2, // lt_s
        0x49 => (n1 as u32) < (n2 as u32), // lt_u
        0x4a => n1 > n2, // gt_s
        0x4b => (n1 as u32) > (n2 as u32), // gt_u
        0x4c => n1 <= n2, // le_s
        0x4d => (n1 as u32) <= (n1 as u32), // le_u
        0x4e => n1 >= n2, // ge_s
        0x4f => (n1 as u32) >= (n2 as u32), // ge_u
        _ => false,
    };
    stack.push_i32(r as i32);
    frame.next();
    Ok(())
}

// i32 binop
fn i32_binop(inst: &Inst, frame: &mut Frame, stack: &mut Stack) -> Result<(), String> {
    let n2 = stack.pop_i32()?;
    let n1 = stack.pop_i32()?;
    let r: i32 = match inst.op_code {
        0x6a => n1 + n2, // add
        0x6b => n1 - n2, // sub
        0x6c => n1 * n2, // mul
        0x6d => n1 / n2, // div_s
        0x6e => ((n1 as u32) / (n2 as u32)) as i32, // div_u
        0x6f => n1 % n2, // rem_s
        0x70 => ((n1 as u32) % (n2 as u32)) as i32, // rem_u
        0x71 => n1 & n2, // and
        0x72 => n1 | n2, // or
        0x73 => n1 ^ n2, // xor
        _ => 0,
    };
    stack.push_i32(r);
    frame.next();
    Ok(())
}

fn not_supported(inst: &Inst, _frame: &mut Frame, _stack: &mut Stack) -> Result<(), String> {
    Err(format!("trap: op({:#02x}) not supported yet", inst.op_code))
}

type ExecInst = fn(&Inst, &mut Frame, &mut Stack) -> Result<(), String>;
const EXEC_TABLE: [ExecInst; 256] = [
/*0x00*/ not_supported, // "unreachale"
/*0x01*/ not_supported, // "nop"
/*0x02*/ not_supported, // "block"
/*0x03*/ exec_03, // "loop
/*0x04*/ not_supported, // "if"
/*0x05*/ not_supported, // "else"
/*0x06*/ not_supported, // REVERVED
/*0x07*/ not_supported, // REVERVED
/*0x08*/ not_supported, // REVERVED
/*0x09*/ not_supported, // REVERVED
/*0x0a*/ not_supported, // REVERVED
/*0x0b*/ exec_0b, // "end"
/*0x0c*/ exec_0c, // "br"
/*0x0d*/ not_supported, // "br_if"
/*0x0e*/ not_supported, // "br_table"
/*0x0f*/ not_supported, // "return"
/*0x10*/ not_supported, // "call"
/*0x11*/ not_supported, // "call_indirect"
/*0x12*/ not_supported, // REVERVED
/*0x13*/ not_supported, // REVERVED
/*0x14*/ not_supported, // REVERVED
/*0x15*/ not_supported, // REVERVED
/*0x16*/ not_supported, // REVERVED
/*0x17*/ not_supported, // REVERVED
/*0x18*/ not_supported, // REVERVED
/*0x19*/ not_supported, // REVERVED
/*0x1a*/ not_supported, // "drop"
/*0x1b*/ not_supported, // "select"
/*0x1c*/ not_supported, // "select"
/*0x1d*/ not_supported, // REVERVED
/*0x1e*/ not_supported, // REVERVED
/*0x1f*/ not_supported, // REVERVED
/*0x20*/ exec_20, // "local.get"
/*0x21*/ exec_21, // "local.set"
/*0x22*/ exec_22, // "local.tee"
/*0x23*/ not_supported, // "global.get"
/*0x24*/ not_supported, // "global.set"
/*0x25*/ not_supported, // "table.get"
/*0x26*/ not_supported, // "table.set"
/*0x27*/ not_supported, // REVERVED
/*0x28*/ not_supported, // "i32.load"
/*0x29*/ not_supported, // "i64.load"
/*0x2a*/ not_supported, // "f32.load"
/*0x2b*/ not_supported, // "f64.load"
/*0x2c*/ not_supported, // "i32.load8_s"
/*0x2d*/ not_supported, // "i32.load8_u"
/*0x2e*/ not_supported, // "i32.load16_s"
/*0x2f*/ not_supported, // "i32.load16_u"
/*0x30*/ not_supported, // "i64.load8_s"
/*0x31*/ not_supported, // "i64.load8_u"
/*0x32*/ not_supported, // "i64.load16_s"
/*0x33*/ not_supported, // "i64.load16_u"
/*0x34*/ not_supported, // "i64.load32_s"
/*0x35*/ not_supported, // "i64.load32_u"
/*0x36*/ not_supported, // "i32.store"
/*0x37*/ not_supported, // "i64.store"
/*0x38*/ not_supported, // "f32.store"
/*0x39*/ not_supported, // "f64.store"
/*0x3a*/ not_supported, // "i32.store8"
/*0x3b*/ not_supported, // "i32.store16"
/*0x3c*/ not_supported, // "i64.store8"
/*0x3d*/ not_supported, // "i64.store16"
/*0x3e*/ not_supported, // "i64.store32"
/*0x3f*/ not_supported, // "memory.size"
/*0x40*/ not_supported, // "memory.grow"
/*0x41*/ exec_41, // "i32.const"
/*0x42*/ not_supported, // "i64.const"
/*0x43*/ not_supported, // "f32.const"
/*0x44*/ not_supported, // "f64.const"
/*0x45*/ exec_45, // "i32.eqz"
/*0x46*/ i32_binop_bool, // "i32.eq"
/*0x47*/ i32_binop_bool, // "i32.ne"
/*0x48*/ i32_binop_bool, // "i32.lt_s"
/*0x49*/ i32_binop_bool, // "i32.lt_u"
/*0x4a*/ i32_binop_bool, // "i32.gt_s"
/*0x4b*/ i32_binop_bool, // "i32.gt_u"
/*0x4c*/ i32_binop_bool, // "i32.le_s"
/*0x4d*/ i32_binop_bool, // "i32.le_u"
/*0x4e*/ i32_binop_bool, // "i32.ge_s"
/*0x4f*/ i32_binop_bool, // "i32.ge_u"
/*0x50*/ not_supported, // "i64.eqz"
/*0x51*/ not_supported, // "i64.eq"
/*0x52*/ not_supported, // "i64.ne"
/*0x53*/ not_supported, // "i64.lt_s"
/*0x54*/ not_supported, // "i64.lt_u"
/*0x55*/ not_supported, // "i64.gt_s"
/*0x56*/ not_supported, // "i64.gt_u"
/*0x57*/ not_supported, // "i64.le_s"
/*0x58*/ not_supported, // "i64.le_u"
/*0x59*/ not_supported, // "i64.ge_s"
/*0x5a*/ not_supported, // "i64.ge_u"
/*0x5b*/ not_supported, // "f32.eq"
/*0x5c*/ not_supported, // "f32.ne"
/*0x5d*/ not_supported, // "f32.lt"
/*0x5e*/ not_supported, // "f32.gt"
/*0x5f*/ not_supported, // "f32.le"
/*0x60*/ not_supported, // "f32.ge"
/*0x61*/ not_supported, // "f64.eq"
/*0x62*/ not_supported, // "f64.ne"
/*0x63*/ not_supported, // "f64.lt"
/*0x64*/ not_supported, // "f64.gt"
/*0x65*/ not_supported, // "f64.le"
/*0x66*/ not_supported, // "f64.ge"
/*0x67*/ not_supported, // "i32.clz"
/*0x68*/ not_supported, // "i32.ctz"
/*0x69*/ not_supported, // "i32.popcnt"
/*0x6a*/ i32_binop, // "i32.add"
/*0x6b*/ i32_binop, // "i32.sub"
/*0x6c*/ i32_binop, // "i32.mul"
/*0x6d*/ i32_binop, // "i32.div_s"
/*0x6e*/ i32_binop, // "i32.div_u"
/*0x6f*/ i32_binop, // "i32.rem_s"
/*0x70*/ i32_binop, // "i32.rem_u"
/*0x71*/ i32_binop, // "i32.and"
/*0x72*/ i32_binop, // "i32.or"
/*0x73*/ i32_binop, // "i32.xor"
/*0x74*/ not_supported, // "i32.shl"
/*0x75*/ not_supported, // "i32.shr_s"
/*0x76*/ not_supported, // "i32.shr_u"
/*0x77*/ not_supported, // "i32.rotl"
/*0x78*/ not_supported, // "i32.rotr"
/*0x79*/ not_supported, // "i64.clz"
/*0x7a*/ not_supported, // "i64.ctz"
/*0x7b*/ not_supported, // "i64.popcnt"
/*0x7c*/ not_supported, // "i64.add"
/*0x7d*/ not_supported, // "i64.sub"
/*0x7e*/ not_supported, // "i64.mul"
/*0x7f*/ not_supported, // "i64.div_s"
/*0x80*/ not_supported, // "i64.div_u"
/*0x81*/ not_supported, // "i64.rem_s"
/*0x82*/ not_supported, // "i64.rem_u"
/*0x83*/ not_supported, // "i64.and"
/*0x84*/ not_supported, // "i64.or"
/*0x85*/ not_supported, // "i64.xor"
/*0x86*/ not_supported, // "i64.shl"
/*0x87*/ not_supported, // "i64.shr_s"
/*0x88*/ not_supported, // "i64.shr_u"
/*0x89*/ not_supported, // "i64.rotl"
/*0x8a*/ not_supported, // "i64.rotr"
/*0x8b*/ not_supported, // "f32.abs"
/*0x8c*/ not_supported, // "f32.neg"
/*0x8d*/ not_supported, // "f32.ceil"
/*0x8e*/ not_supported, // "f32.floor"
/*0x8f*/ not_supported, // "f32.trunc"
/*0x90*/ not_supported, // "f32.nearest"
/*0x91*/ not_supported, // "f32.sqrt"
/*0x92*/ not_supported, // "f32.add"
/*0x93*/ not_supported, // "f32.sub"
/*0x94*/ not_supported, // "f32.mul"
/*0x95*/ not_supported, // "f32.div"
/*0x96*/ not_supported, // "f32.min"
/*0x97*/ not_supported, // "f32.max"
/*0x98*/ not_supported, // "f32.copysign"
/*0x99*/ not_supported, // "f64.abs"
/*0x9a*/ not_supported, // "f64.neg"
/*0x9b*/ not_supported, // "f64.ceil"
/*0x9c*/ not_supported, // "f64.floor"
/*0x9d*/ not_supported, // "f64.trunc"
/*0x9e*/ not_supported, // "f64.nearest"
/*0x9f*/ not_supported, // "f64.sqrt"
/*0xa0*/ not_supported, // "f64.add"
/*0xa1*/ not_supported, // "f64.sub"
/*0xa2*/ not_supported, // "f64.mul"
/*0xa3*/ not_supported, // "f64.div"
/*0xa4*/ not_supported, // "f64.min"
/*0xa5*/ not_supported, // "f64.max"
/*0xa6*/ not_supported, // "f64.copysign"
/*0xa7*/ not_supported, // "i32.wrap_i64"
/*0xa8*/ not_supported, // "i32.trunc_f32_s"
/*0xa9*/ not_supported, // "i32.trunc_f32_u"
/*0xaa*/ not_supported, // "i32.trunc_f64_s"
/*0xab*/ not_supported, // "i32.trunc_f64_u"
/*0xac*/ not_supported, // "i64.extend_i32_s"
/*0xad*/ not_supported, // "i64.extend_i32_u"
/*0xae*/ not_supported, // "i64.trunc_f32_s"
/*0xaf*/ not_supported, // "i64.trunc_f32_u"
/*0xb0*/ not_supported, // "i64.trunc_f64_s"
/*0xb1*/ not_supported, // "i64.trunc_f64_u"
/*0xb2*/ not_supported, // "f32.convert_i32_s"
/*0xb3*/ not_supported, // "f32.convert_i32_u"
/*0xb4*/ not_supported, // "f32.convert_i64_s"
/*0xb5*/ not_supported, // "f32.convert_i64_u"
/*0xb6*/ not_supported, // "f32.demote_f64"
/*0xb7*/ not_supported, // "f64.convert_i32_s"
/*0xb8*/ not_supported, // "f64.convert_i32_u"
/*0xb9*/ not_supported, // "f64.convert_i64_s"
/*0xba*/ not_supported, // "f64.convert_i64_u"
/*0xbb*/ not_supported, // "f64.promote_f32"
/*0xbc*/ not_supported, // "i32.reinterpret_f32"
/*0xbd*/ not_supported, // "i64.reinterpret_f64"
/*0xbe*/ not_supported, // "f32.reinterpret_i32"
/*0xbf*/ not_supported, // "f64.reinterpret_i64"
/*0xc0*/ not_supported, // "i32.extend8_s"
/*0xc1*/ not_supported, // "i32.extend16_s"
/*0xc2*/ not_supported, // "i64.extend8_s"
/*0xc3*/ not_supported, // "i64.extend16_s"
/*0xc4*/ not_supported, // "i64.extend32_s"
/*0xc5*/ not_supported, // REVERVED
/*0xc6*/ not_supported, // REVERVED
/*0xc7*/ not_supported, // REVERVED
/*0xc8*/ not_supported, // REVERVED
/*0xc9*/ not_supported, // REVERVED
/*0xca*/ not_supported, // REVERVED
/*0xcb*/ not_supported, // REVERVED
/*0xcc*/ not_supported, // REVERVED
/*0xcd*/ not_supported, // REVERVED
/*0xce*/ not_supported, // REVERVED
/*0xcf*/ not_supported, // REVERVED
/*0xd0*/ not_supported, // "ref.null"
/*0xd1*/ not_supported, // "ref.is_null"
/*0xd2*/ not_supported, // "ref.func"
/*0xd3*/ not_supported, // REVERVED
/*0xd4*/ not_supported, // REVERVED
/*0xd5*/ not_supported, // REVERVED
/*0xd6*/ not_supported, // REVERVED
/*0xd7*/ not_supported, // REVERVED
/*0xd8*/ not_supported, // REVERVED
/*0xd9*/ not_supported, // REVERVED
/*0xda*/ not_supported, // REVERVED
/*0xdb*/ not_supported, // REVERVED
/*0xdc*/ not_supported, // REVERVED
/*0xdd*/ not_supported, // REVERVED
/*0xde*/ not_supported, // REVERVED
/*0xdf*/ not_supported, // REVERVED
/*0xe0*/ not_supported, // REVERVED
/*0xe1*/ not_supported, // REVERVED
/*0xe2*/ not_supported, // REVERVED
/*0xe3*/ not_supported, // REVERVED
/*0xe4*/ not_supported, // REVERVED
/*0xe5*/ not_supported, // REVERVED
/*0xe6*/ not_supported, // REVERVED
/*0xe7*/ not_supported, // REVERVED
/*0xe8*/ not_supported, // REVERVED
/*0xe9*/ not_supported, // REVERVED
/*0xea*/ not_supported, // REVERVED
/*0xeb*/ not_supported, // REVERVED
/*0xec*/ not_supported, // REVERVED
/*0xed*/ not_supported, // REVERVED
/*0xee*/ not_supported, // REVERVED
/*0xef*/ not_supported, // REVERVED
/*0xf0*/ not_supported, // REVERVED
/*0xf1*/ not_supported, // REVERVED
/*0xf2*/ not_supported, // REVERVED
/*0xf3*/ not_supported, // REVERVED
/*0xf4*/ not_supported, // REVERVED
/*0xf5*/ not_supported, // REVERVED
/*0xf6*/ not_supported, // REVERVED
/*0xf7*/ not_supported, // REVERVED
/*0xf8*/ not_supported, // REVERVED
/*0xf9*/ not_supported, // REVERVED
/*0xfa*/ not_supported, // REVERVED
/*0xfb*/ not_supported, // REVERVED
/*0xfc*/ not_supported, // see another table
/*0xfd*/ not_supported, // see another table
/*0xfe*/ not_supported, // not defined
/*0xff*/ not_supported, // not defined
];
