use std::io::Read;

use crate::util::{read_u32_from_leb128, read_vec};

use super::typeidx::Typeidx;
use super::tabletype::TableType;
use super::memtype::MemType;
use super::globaltype::{GlobalType, read_globaltype};
use super::tabletype::read_tabletype;
use super::limits::read_limits;
use super::typeidx::read_typeidx;
use super::name::{Name, read_name};

pub(super) struct Import {
    module: Name,
    name: Name,
    desc: ImportDesc,
}

enum ImportDesc {
    Func(Typeidx),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
}

pub(super) fn read_importsec(reader: &mut impl Read) -> Vec<Import> {
    // prefixはsection number 2
    let length = read_u32_from_leb128(reader);
    let mut handle = reader.take(length as u64);
    read_vec(&mut handle, read_import)
}

fn read_import(reader: &mut impl Read) -> Import {
    let module_identifier = read_name(reader);
    let name_identifier = read_name(reader);
    let importdesc = read_importdesc(reader);
    Import {
        module: module_identifier,
        name: name_identifier,
        desc: importdesc,
    }
}

fn read_importdesc(reader: &mut impl Read) -> ImportDesc {
    if let Some(Ok(byte)) = reader.bytes().next() {
        match byte {
            0x00 => { return read_importdesc_func(reader) },
            0x01 => { return read_importdesc_tabletype(reader) },
            0x02 => { return read_importdesc_memtype(reader) },
            0x03 => { return read_importdesc_globaltype(reader) },
            _ => panic!("invalid on read_importdesc"),
        }
    }

    unimplemented!()
}

fn read_importdesc_func(reader: &mut impl Read) -> ImportDesc {
    let typeidx = read_typeidx(reader);
    ImportDesc::Func(typeidx)
}

fn read_importdesc_tabletype(reader: &mut impl Read) -> ImportDesc {
    ImportDesc::Table(read_tabletype(reader))
}

fn read_importdesc_memtype(reader: &mut impl Read) -> ImportDesc {
    let limits = read_limits(reader);
    ImportDesc::Mem(MemType::new(limits))
}

fn read_importdesc_globaltype(reader: &mut impl Read) -> ImportDesc {
    let globaltype = read_globaltype(reader);
    ImportDesc::Global(globaltype)
}

