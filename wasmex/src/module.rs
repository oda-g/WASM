// SPDX-License-Identifier: MIT
// Copyright(c) 2023 Itsuro Oda
// https://opensource.org/license/mit/

#![allow(dead_code)]

use std::collections::HashMap;
use std::fmt;

use crate::bytecode::*;
use crate::inst::*;
use crate::exec::*;

pub struct Module {
    sec_summary: Vec<SectionSummary>,
    sections: HashMap<u8, Section>,
    funcs: Vec<Function>,
}

enum SummaryItem {
    Num(usize),
    Index(u32),
    Name(String)
}

struct SectionSummary {
    id: u8,
    size: u32,
    start: usize,
    item: SummaryItem,
}

impl fmt::Display for SectionSummary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]{}:\tsize({})\t ",
            self.id, SECID2NAME[self.id as usize], self.size)?;
        match &self.item {
            SummaryItem::Num(num) => write!(f, "items: {}", num),
            SummaryItem::Index(idx) => write!(f, "index: {}", idx),
            SummaryItem::Name(name) => write!(f, "name: {}", &name),
        }
    }
}

const SECID2NAME: [&str; 13] = [
"custom",   // 0
"type",     // 1
"import",   // 2
"function", // 3
"table",    // 4
"memory",   // 5
"global",   // 6
"export",   // 7
"start",    // 8
"element",  // 9
"code",     // 10
"data",     // 11
"datacount" // 12
];

struct Typesec {
    ft: Vec<Functype>,
}

impl Typesec {
    fn get(buf: &mut ByteCodeBuff) -> Self {
        Typesec {ft: get_vector::<Functype>(buf),}
    }

    fn show(&self) {
        let vec = &self.ft;
        for (i, v) in vec.iter().enumerate() {
            println!("type[{}]: {}", i, v);
        }
    }
}

enum Importdesc {
    Func(u32), // type index
    Table(Tabletype),
    Mem(Limits), // memtype == limits
    Global(Globaltype),
}

struct Import {
    module: String,
    name: String,
    desc: Importdesc,
}

impl GetType for Import {
    fn get(buf: &mut ByteCodeBuff) -> Self {
        let module = buf.get_name();
        let name = buf.get_name();
        let d = buf.get_byte();
        let desc = match d {
            0 => Importdesc::Func(buf.get_u32()),
            1 => Importdesc::Table(Tabletype::get(buf)),
            2 => Importdesc::Mem(Limits::get(buf)),
            3 => Importdesc::Global(Globaltype::get(buf)),
            _ => panic!("unknown import desc"),
        };
        Import {module, name, desc,}
    }
}

struct Importsec {
    import: Vec<Import>,
}

impl Importsec {
    fn get(buf: &mut ByteCodeBuff) -> Self {
        Importsec {import: get_vector::<Import>(buf),}
    }

    fn show(&self) {
        for i in 0..self.import.len() {
            print!("import[{}]: {}.{} ",
                i, &self.import[i].module, &self.import[i].name);
            match &self.import[i].desc {
                Importdesc::Func(idx) => println!("type={}", idx),
                Importdesc::Table(table) => println!("{}", table),
                Importdesc::Mem(lm) => println!("{}", lm),
                Importdesc::Global(gt) => println!("{}", gt),
            };
        }
    }
}

struct Funcsec {
    typeidx: Vec<u32>,
}

impl Funcsec {
    fn get(buf: &mut ByteCodeBuff) -> Self {
        Funcsec {typeidx: get_vector::<u32>(buf),}
    }

    fn show(&self) {
        for i in 0..self.typeidx.len() {
            println!("func[{}]: type={}", i, &self.typeidx[i]);
        }
    }
}

struct Tablesec {
    table: Vec<Tabletype>,
}

impl Tablesec {
    fn get(buf: &mut ByteCodeBuff) -> Self {
        Tablesec {table: get_vector::<Tabletype>(buf),}
    }

    fn show(&self) {
        for i in 0..self.table.len() {
            println!("table[{}]: {}", i, &self.table[i]);
        }
    }
}

struct Memsec {
    mem: Vec<Limits>,
}

impl Memsec {
    fn get(buf: &mut ByteCodeBuff) -> Self {
        Memsec {mem: get_vector::<Limits>(buf),}
    }

    fn show(&self) {
        for i in 0..self.mem.len() {
            println!("memory[{}]: {}", i, &self.mem[i]);
        }
    }
}

struct Global {
    globaltype: Globaltype,
    expr: Expr,
}

impl GetType for Global {
    fn get(buf: &mut ByteCodeBuff) -> Self {
        Global {
            globaltype: Globaltype::get(buf),
            expr: Expr::get(buf),
        }
    }
}

struct Globalsec {
    global: Vec<Global>,
}

impl Globalsec {
    fn get(buf: &mut ByteCodeBuff) -> Self {
        Globalsec {global: get_vector::<Global>(buf),}
    }

    fn show(&self) {
        for i in 0..self.global.len() {
            println!("global[{}]: {} {}", i,
                &self.global[i].globaltype, &self.global[i].expr);
        }
    }
}

enum Exportdesc {
    Func(u32),
    Table(u32),
    Mem(u32),
    Global(u32),
}

struct Export {
    name: String,
    desc: Exportdesc,
}

impl GetType for Export {
    fn get(buf: &mut ByteCodeBuff) -> Self {
        let name = buf.get_name();
        let d = buf.get_byte();
        let idx = buf.get_u32();
        let desc = match d {
            0 => Exportdesc::Func(idx),
            1 => Exportdesc::Table(idx),
            2 => Exportdesc::Mem(idx),
            3 => Exportdesc::Global(idx),
            _ => panic!("unknown exportdesc"),
        };
        Export {name, desc,}
    }
}

struct Exportsec {
    export: Vec<Export>,
}

impl Exportsec {
    fn get(buf: &mut ByteCodeBuff) -> Self {
        Exportsec {export: get_vector::<Export>(buf),}
    }

    fn show(&self) {
        for i in 0..self.export.len() {
            print!("export[{}]: {} ", i, &self.export[i].name);
            match &self.export[i].desc {
                Exportdesc::Func(idx) => println!("func={}", idx),
                Exportdesc::Table(idx) => println!("table={}", idx),
                Exportdesc::Mem(idx) => println!("memory={}", idx),
                Exportdesc::Global(idx) => println!("global={}", idx),
            };
        }
    }
}

struct Startsec {
    idx: u32,
}

impl Startsec {
    fn get(buf: &mut ByteCodeBuff) -> Self {
        Startsec {idx: buf.get_u32(),}
    }

    fn show(&self) {
        println!("start: func={}", self.idx);
    }
}

struct Elem0 {
    expr: Expr,
    funcidx: Vec<u32>,
}

fn format_funcidx(idx: &Vec<u32>) -> String {
    let mut s = "[".to_string();
    for (i, v) in idx.iter().enumerate() {
        s.push_str(&format!("{}", v));
        if i != idx.len() - 1 {
            s.push_str(" ");
        }
    }
    s.push_str("]");
    s
}

impl Elem0 {
    fn get(buf: &mut ByteCodeBuff) -> Self {
        Elem0 {
            expr: Expr::get(buf),
            funcidx: get_vector::<u32>(buf),
        }
    }

    fn format(&self) -> String {
        let s = format!("active table=0 offset{} funcidx{}", &self.expr,
            &format_funcidx(&self.funcidx));
        s
    }
}

struct Elem1 {
    elemkind: u8,
    funcidx: Vec<u32>,
}

impl Elem1 {
    fn get(buf: &mut ByteCodeBuff) -> Self {
        Elem1 {
            elemkind: buf.get_byte(),
            funcidx: get_vector::<u32>(buf),
        }
    }

    fn format(&self) -> String {
        let s = format!("passive funcidx{}", &format_funcidx(&self.funcidx));
        s
    }
}

struct Elem2 {
    tableidx: u32,
    expr: Expr,
    elemkind: u8,
    funcidx: Vec<u32>,
}

impl Elem2 {
    fn get(buf: &mut ByteCodeBuff) -> Self {
        Elem2 {
            tableidx: buf.get_u32(),
            expr: Expr::get(buf),
            elemkind: buf.get_byte(),
            funcidx: get_vector::<u32>(buf),
        }
    }

    fn format(&self) -> String {
        let s = format!("active table={} offset{} funcidx{}", self.tableidx,
            &self.expr, &format_funcidx(&self.funcidx));
        s
    }
}

struct Elem3 {
    elemkind: u8,
    funcidx: Vec<u32>,
}

impl Elem3 {
    fn get(buf: &mut ByteCodeBuff) -> Self {
        Elem3 {
            elemkind: buf.get_byte(),
            funcidx: get_vector::<u32>(buf),
        }
    }

    fn format(&self) -> String {
        let s = format!("declarative funcidx{}", &format_funcidx(&self.funcidx));
        s
    }
}

struct Elem4 {
    expr: Expr,
    el: Vec<Expr>,
}

impl Elem4 {
    fn get(buf: &mut ByteCodeBuff) -> Self {
        Elem4 {
            expr: Expr::get(buf),
            el: get_vector::<Expr>(buf),
        }
    }

    fn format(&self) -> String {
        let s = format!("type 4: todo");
        s
    }
}

struct Elem5 {
    reftype: Valtype,
    el: Vec<Expr>,
}

impl Elem5 {
    fn get(buf: &mut ByteCodeBuff) -> Self {
        Elem5 {
            reftype: Valtype::get(buf),
            el: get_vector::<Expr>(buf),
        }
    }

    fn format(&self) -> String {
        let s = format!("type 5: todo");
        s
    }
}

struct Elem6 {
    tableidx: u32,
    expr: Expr,
    reftype: Valtype,
    el: Vec<Expr>,
}

impl Elem6 {
    fn get(buf: &mut ByteCodeBuff) -> Self {
        Elem6 {
            tableidx: buf.get_u32(),
            expr: Expr::get(buf),
            reftype: Valtype::get(buf),
            el: get_vector::<Expr>(buf),
        }
    }

    fn format(&self) -> String {
        let s = format!("type 6: todo");
        s
    }
}

struct Elem7 {
    reftype: Valtype,
    el: Vec<Expr>,
}

impl Elem7 {
    fn get(buf: &mut ByteCodeBuff) -> Self {
        Elem7 {
            reftype: Valtype::get(buf),
            el: get_vector::<Expr>(buf),
        }
    }

    fn format(&self) -> String {
        let s = format!("type 7: todo");
        s
    }
}

enum Elem {
    Elem0(Elem0),
    Elem1(Elem1),
    Elem2(Elem2),
    Elem3(Elem3),
    Elem4(Elem4),
    Elem5(Elem5),
    Elem6(Elem6),
    Elem7(Elem7),
}

impl GetType for Elem {
    fn get(buf: &mut ByteCodeBuff) -> Self {
        let elem_id = buf.get_u32();
        match elem_id {
            0 => Elem::Elem0(Elem0::get(buf)),
            1 => Elem::Elem1(Elem1::get(buf)),
            2 => Elem::Elem2(Elem2::get(buf)),
            3 => Elem::Elem3(Elem3::get(buf)),
            4 => Elem::Elem4(Elem4::get(buf)),
            5 => Elem::Elem5(Elem5::get(buf)),
            6 => Elem::Elem6(Elem6::get(buf)),
            7 => Elem::Elem7(Elem7::get(buf)),
            _ => panic!("unknown element type"),
        }
    }
}

struct Elemsec {
    elem: Vec<Elem>,
}

impl Elemsec {
    fn get(buf: &mut ByteCodeBuff) -> Self {
        Elemsec {elem: get_vector::<Elem>(buf),}
    }

    fn show(&self) {
        for i in 0..self.elem.len() {
            let s = format!("element[{}]:", i);
            let a = match &self.elem[i] {
                Elem::Elem0(el) => el.format(),
                Elem::Elem1(el) => el.format(),
                Elem::Elem2(el) => el.format(),
                Elem::Elem3(el) => el.format(),
                Elem::Elem4(el) => el.format(),
                Elem::Elem5(el) => el.format(),
                Elem::Elem6(el) => el.format(),
                Elem::Elem7(el) => el.format(),
            };
            println!("{} {}", &s, &a);
        }
    }
}

pub struct Locals {
    num: u32,
    valtype: Valtype,
}

impl GetType for Locals {
    fn get(buf: &mut ByteCodeBuff) -> Self {
        Locals {
            num: buf.get_u32(),
            valtype: Valtype::get(buf),
        }
    }
}

struct Code {
    size: u32,
    start: usize,
    locals: Vec<Valtype>,
}

impl GetType for Code {
    fn get(buf: &mut ByteCodeBuff) -> Self {
        let len = buf.get_u32();
        let end = buf.get_cur() + len as usize;
        let vec_locals = get_vector::<Locals>(buf);
        let start = buf.get_cur();
        buf.set_cur(end);
        let mut locals = Vec::new();
        for l in vec_locals.iter() {
            for _ in 0..l.num {
                locals.push(l.valtype.clone());
            }
        }
        Code {size: len, start, locals,}
    }
}

fn format_locals(locals: &Vec<Valtype>) -> String {
    let mut s = "[".to_string();
    for (i, l) in locals.iter().enumerate() {
        s.push_str(&format!("{}", &l));
        if i != locals.len() - 1 {
            s.push_str(" ");
        }
    }
    s.push_str("]");
    s
}

struct Codesec {
    code: Vec<Code>,
}

impl Codesec {
    fn get(buf: &mut ByteCodeBuff) -> Self {
        Codesec {code: get_vector::<Code>(buf),}
    }

    fn show(&self) {
        for i in 0..self.code.len() {
            println!("code[{}]: size({}) locals{}", i, self.code[i].size,
                &format_locals(&self.code[i].locals));
        }
    }
}

struct Data {
    id: u32,
    expr: Expr,
    data: Vec<u8>,
    memidx: u32,
}

impl GetType for Data {
    fn get(buf: &mut ByteCodeBuff) -> Self {
        let t = buf.get_u32();
        match t {
            0 => Data {
                    id: t,
                    expr: Expr::get(buf),
                    data: buf.get_data(),
                    memidx: 0
                },
            1 => Data {
                    id: t,
                    expr: Expr(vec![]),
                    data: buf.get_data(),
                    memidx: 0
                },
            2 => Data {
                    id: t,
                    memidx: buf.get_u32(),
                    expr: Expr::get(buf),
                    data: buf.get_data(),
                },
            _ => panic!("unknown data type"),
        }
    }
}

struct Datasec {
    data: Vec<Data>,
}

impl Datasec {
    fn get(buf: &mut ByteCodeBuff) -> Self {
        Datasec {data: get_vector::<Data>(buf),}
    }

    fn show(&self) {
        for i in 0..self.data.len() {
            print!("data[{}]: ", i);
            match self.data[i].id {
                0 | 2 => println!("size({}) active memory={} {}", self.data[i].data.len(),
                        self.data[i].memidx, &self.data[i].expr),
                1 => println!("size({}) passive", self.data[i].data.len()),
                _ => (),
            }
        }
    }
}

struct DataCountsec {
    count: u32,
}

impl DataCountsec {
    fn get(buf: &mut ByteCodeBuff) -> Self {
        DataCountsec {count: buf.get_u32(),}
    }

    fn show(&self) {
        println!("to do");
    }
}

struct Customsec {
    name: String,
}

impl Customsec {
    fn show(&self) {
        println!("to do");
    }
}

enum Section {
    Custom(Customsec),
    Type(Typesec),
    Import(Importsec),
    Function(Funcsec),
    Table(Tablesec),
    Memory(Memsec),
    Global(Globalsec),
    Export(Exportsec),
    Start(Startsec),
    Element(Elemsec),
    Code(Codesec),
    Data(Datasec),
    DataCount(DataCountsec),
}

impl Section {
    pub fn get_section(sec_id: u8, size: u32, buf: &mut ByteCodeBuff) -> Section {
        match sec_id {
            0 => {
                let end = buf.get_cur() + size as usize;
                let name = buf.get_name();
                buf.set_cur(end);
                Section::Custom(Customsec{name,})
            },
            1 => Section::Type(Typesec::get(buf)),
            2 => Section::Import(Importsec::get(buf)),
            3 => Section::Function(Funcsec::get(buf)),
            4 => Section::Table(Tablesec::get(buf)),
            5 => Section::Memory(Memsec::get(buf)),
            6 => Section::Global(Globalsec::get(buf)),
            7 => Section::Export(Exportsec::get(buf)),
            8 => Section::Start(Startsec::get(buf)),
            9 => Section::Element(Elemsec::get(buf)),
            10 => Section::Code(Codesec::get(buf)),
            11 => Section::Data(Datasec::get(buf)),
            12 => Section::DataCount(DataCountsec::get(buf)),
            _ => panic!("unknown section id"),
        }
    }

    fn summary_item(&self) -> SummaryItem {
        match self {
            Section::Custom(sec) => SummaryItem::Name(sec.name.clone()),
            Section::Type(sec) => SummaryItem::Num(sec.ft.len()),
            Section::Import(sec) => SummaryItem::Num(sec.import.len()),
            Section::Function(sec) => SummaryItem::Num(sec.typeidx.len()),
            Section::Table(sec) => SummaryItem::Num(sec.table.len()),
            Section::Memory(sec) => SummaryItem::Num(sec.mem.len()),
            Section::Global(sec) => SummaryItem::Num(sec.global.len()),
            Section::Export(sec) => SummaryItem::Num(sec.export.len()),
            Section::Start(sec) => SummaryItem::Index(sec.idx),
            Section::Element(sec) => SummaryItem::Num(sec.elem.len()),
            Section::Code(sec) => SummaryItem::Num(sec.code.len()),
            Section::Data(sec) => SummaryItem::Num(sec.data.len()),
            Section::DataCount(sec) => SummaryItem::Num(sec.count as usize),
        }
    }

    fn show(&self) {
        match self {
            Section::Custom(sec) => sec.show(),
            Section::Type(sec) => sec.show(),
            Section::Import(sec) => sec.show(),
            Section::Function(sec) => sec.show(),
            Section::Table(sec) => sec.show(),
            Section::Memory(sec) => sec.show(),
            Section::Global(sec) => sec.show(),
            Section::Export(sec) => sec.show(),
            Section::Start(sec) => sec.show(),
            Section::Element(sec) => sec.show(),
            Section::Code(sec) => sec.show(),
            Section::Data(sec) => sec.show(),
            Section::DataCount(sec) => sec.show(),
        }
    }
}

impl Module {
    pub fn show_summary(&self) {
        for sec_s in &self.sec_summary {
            println!("{}", sec_s);
        }
    }

    pub fn show_section(&self) {
        for sec_s in &self.sec_summary {
            if sec_s.id == 0 || sec_s.id == 12 {
                println!("{}", sec_s);
            } else {
                if let Some(sec) = self.sections.get(&sec_s.id) {
                    sec.show();
                }
            }
        }
    }

    pub fn show_code(&self) {
        if let Some(Section::Code(sec)) = self.sections.get(&10) {
            let num = self.num_import_func();
            for i in 0..sec.code.len() {
                println!("code[{}]: size({}) locals{}", i, sec.code[i].size,
                    &format_locals(&sec.code[i].locals));
                let func = self.get_local_func(num + i);
                func.show_insts();
                println!("");
            }
        }
    }

    pub fn num_funcs(&self) -> usize {
        self.funcs.len()
    }

    fn num_import_func(&self) -> usize {
        let mut num: usize = 0;
        for item in &self.funcs {
            match item {
                Function::Import(_) => num += 1,
                _ => (),
            }
        }
        num
    }

    pub fn is_import_func(&self, idx: usize) -> bool {
        let item = self.funcs.get(idx);
        if let Some(Function::Import(_)) = item {
            return true
        }
        false
    }

    pub fn get_local_func(&self, idx: usize) -> &LocalFunc {
        let item = self.funcs.get(idx);
        if let Some(Function::Local(lc_func)) = item {
            return lc_func
        } else {
            panic!("illegale index");
        }
    }
}

fn check_magic(buf: &[u8]) -> Result<(), String> {
    if buf.len() < 8 || buf[0..4] != [0, b'a', b's', b'm'] {
        return Err("invalid format".to_string());
    }

    let ver = u32::from_le_bytes(buf[4..8].try_into().unwrap());
    if ver == 1 {
        Ok(())
    } else {
        Err(format!("version {} is invalid", ver))
    }
}

pub fn init_module(buf: Vec<u8>) -> Result<Module, String> {
    check_magic(&buf)?;

    let mut buf = ByteCodeBuff::new(buf);
    let mut sec_summary = Vec::new();
    let mut sections = HashMap::new();
    let mut funcs = Vec::new();

    buf.add_cur(8);
    loop {
        if !buf.more() {
            break;
        }
        let sec_id = buf.get_byte();
        let size = buf.get_u32();
        let start = buf.get_cur();
        let section = Section::get_section(sec_id, size, &mut buf);
        let item = section.summary_item();
        let summary = SectionSummary {id: sec_id, size, start, item,};
        sec_summary.push(summary);
        if sec_id != 0 {
            sections.insert(sec_id, section);
        }
    }

    if let Some(Section::Import(import_sec)) = sections.get(&2) {
        for im in &import_sec.import {
            if let Importdesc::Func(idx) = &im.desc {
                if let Some(Section::Type(type_sec)) = sections.get(&1) {
                    let im_func = ImportFunc {
                        name: format!("{}.{}", im.module, im.name),
                        ft: type_sec.ft[*idx as usize].clone(),
                    };
                    funcs.push(Function::Import(im_func));
                }
            }
        }
    }

    if let Some(Section::Code(code_sec)) = sections.get(&10) {
        if let Some(Section::Function(func_sec)) = sections.get(&3) {
            for (i, idx) in func_sec.typeidx.iter().enumerate() {
                let code = &code_sec.code[i];
                if let Some(Section::Type(type_sec)) = sections.get(&1) {
                    buf.set_cur(code.start);
                    let lc_func = LocalFunc {
                        ft: type_sec.ft[*idx as usize].clone(),
                        locals: code.locals.clone(),
                        insts: get_insts(&mut buf),
                    };
                    funcs.push(Function::Local(lc_func));
                }
            }
        }
    }

    Ok(Module{sec_summary, sections, funcs,})
}
