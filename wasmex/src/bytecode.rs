// SPDX-License-Identifier: MIT
// Copyright(c) 2023 Itsuro Oda
// https://opensource.org/license/mit/

#![allow(dead_code)]

use std::fmt;

pub trait GetType {
    fn get(buf: &mut ByteCodeBuff) -> Self;
}

impl GetType for u32 {
    fn get(buf: &mut ByteCodeBuff) -> Self {
        buf.get_u32()
    }
}

pub fn get_vector<T: GetType>(buf: &mut ByteCodeBuff) -> Vec<T> {
    let mut vec: Vec<T> = Vec::new();
    let n = buf.get_u32();
    for _ in 0..n {
        vec.push(T::get(buf));
    }
    vec
}

#[derive(Clone)]
pub struct Valtype(pub u8);

impl GetType for Valtype {
    fn get(buf: &mut ByteCodeBuff) -> Self {
        Valtype(buf.get_byte())
    }
}

impl fmt::Display for Valtype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match &self.0 {
            0x7f => "i32",
            0x7e => "i64",
            0x7d => "f32",
            0x7c => "f64",
            0x7b => "v128",
            0x70 => "funcref",
            0x6f => "externref",
            _ => "?",
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone)]
pub struct Resulttype(pub Vec<Valtype>);

impl GetType for Resulttype {
    fn get(buf: &mut ByteCodeBuff) -> Self {
        Resulttype(get_vector::<Valtype>(buf))
    }
}

impl fmt::Display for Resulttype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;
        let vec = &self.0;
        for (i, v) in vec.iter().enumerate() {
            write!(f, "{}", v)?;
            if i != vec.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, ")")
    }
}

#[derive(Clone)]
pub struct Functype {
    pub input: Resulttype,
    pub output: Resulttype,
}

impl GetType for Functype {
    fn get(buf: &mut ByteCodeBuff) -> Self {
        assert!(buf.get_byte() == 0x60);
        Functype {
            input: Resulttype::get(buf),
            output: Resulttype::get(buf),
        }
    }
}

impl fmt::Display for Functype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", &self.input, &self.output)
    }
}

pub enum Limits {
    Min(u32),
    Minmax(u32, u32),
}

impl GetType for Limits {
    fn get(buf: &mut ByteCodeBuff) -> Self {
        let t = buf.get_byte();
        match t {
            0 => {
                let min = buf.get_u32();
                Limits::Min(min)
            },
            1 => {
                let min = buf.get_u32();
                let max = buf.get_u32();
                Limits::Minmax(min, max)
            },
            _ => panic!("unknown limits type"),
        }
    }
}

impl fmt::Display for Limits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Limits::Min(min) => write!(f, "min: {}", min),
            Limits::Minmax(min, max) => write!(f, "min: {} max: {}", min, max),
        }
    }
}

pub struct Tabletype {
    reftype: Valtype,
    limits: Limits,
}

impl GetType for Tabletype {
    fn get(buf: &mut ByteCodeBuff) -> Self {
        Tabletype {
            reftype: Valtype::get(buf),
            limits: Limits::get(buf),
        }
    }
}

impl fmt::Display for Tabletype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {}", &self.reftype, &self.limits)
    }
}

pub struct Globaltype {
    valtype: Valtype,
    mutable: u8,
}

impl GetType for Globaltype {
    fn get(buf: &mut ByteCodeBuff) -> Self {
        Globaltype {
            valtype: Valtype::get(buf),
            mutable: buf.get_byte(),
        }
    }
}

impl fmt::Display for Globaltype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ", &self.valtype)?;
        match self.mutable {
            0 => write!(f, "const"),
            1 => write!(f, "var"),
            _ => write!(f, "illegal mut"),
        }
    }
}

pub struct Expr(pub Vec<u8>);

impl GetType for Expr {
    fn get(buf: &mut ByteCodeBuff) -> Self {
        let mut expr = Vec::new();
        loop {
            let e = buf.get_byte();
            expr.push(e);
            if e == 0x0b {
                break;
            }
        }
        Expr(expr)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let vec = &self.0;
        for (i, v) in vec.iter().enumerate() {
            write!(f, "{:02x}", v)?;
            if i != vec.len() - 1 {
                write!(f, " ")?;
            }
        }
        write!(f, "]")
    }
}

pub struct ByteCodeBuff {
    buf: Vec<u8>,
    c: usize, // cursol
}

impl ByteCodeBuff {
    pub fn new(buf: Vec<u8>) -> Self {
        Self {
            buf,
            c: 0
        }
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn get_cur(&self) -> usize {
        self.c
    }

    pub fn set_cur(&mut self, cur: usize) {
        self.c = cur;
    }

    pub fn add_cur(&mut self, cur: usize) {
        self.c += cur;
    }

    pub fn more(&self) -> bool {
        self.c < self.buf.len()
    }

    pub fn get_byte(&mut self) -> u8 {
        let byte = self.buf[self.c];
        self.c += 1;
        byte
    }

    pub fn get_u32(&mut self) -> u32 {
        // get u32: unsigned LEB128 encoding
        let mut num:u32 = 0;
        let mut shift = 0;
        loop {
            let b = self.buf[self.c];
            self.c += 1;
            num |= ((b & 0x7f) as u32) << shift;
            if b & 0x80 == 0 {
                break;
            }
            shift += 7;
        }

        num
    }

    pub fn get_i32(&mut self) -> i32 {
        let mut num:i32 = 0;
        let mut shift = 0;
        let mut b:u8;
        let size = 32;

        loop {
            b = self.buf[self.c];
            self.c += 1;
            num |= ((b & 0x7f) as i32) << shift;
            shift += 7;
            if b & 0x80 == 0 {
                break;
            }
        }

        if (shift < size) && (b & 0x40 != 0) {
            num |= !0 << shift;
        }

        num
    }

    pub fn get_i64(&mut self) -> i64 {
        let mut num:i64 = 0;
        let mut shift = 0;
        let mut b:u8;
        let size = 64;

        loop {
            b = self.buf[self.c];
            self.c += 1;
            num |= ((b & 0x7f) as i64) << shift;
            shift += 7;
            if b & 0x80 == 0 {
                break;
            }
        }

        if (shift < size) && (b & 0x40 != 0) {
            num |= !0 << shift;
        }

        num
    }

    pub fn get_f32(&mut self) -> f32 {
        // IEEE 754 little endian
        let mut tmp: [u8; 4] = [0; 4];
        tmp.copy_from_slice(&self.buf[self.c..self.c + 4]);
        let bin:u32 = u32::from_le_bytes(tmp);
        let num:f32 = f32::from_bits(bin);
        self.c += 4;

        num
    }

    pub fn get_f64(&mut self) -> f64 {
        // IEEE 754 little endian
        let mut tmp: [u8; 8] = [0; 8];
        tmp.copy_from_slice(&self.buf[self.c..self.c + 8]);
        let bin:u64 = u64::from_le_bytes(tmp);
        let num:f64 = f64::from_bits(bin);
        self.c += 8;

        num
    }

    pub fn get_name(&mut self) -> String {
        let n = self.get_u32() as usize;
        let v = self.buf[self.c..self.c + n].to_vec();
        let name = String::from_utf8(v).unwrap();
        self.add_cur(n);

        name
    }

    pub fn get_data(&mut self) -> Vec<u8> {
        let len = self.get_u32() as usize;
        let data = self.buf[self.c..self.c + len].to_vec();
        self.c += len;
        data
    }
}
