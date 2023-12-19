use std::fs::File;
use std::io::Read;
use std::{error::Error, ffi::CString};

use bytes::{Buf, Bytes};
use uuid::{Builder, Uuid, uuid};

use bitflags::bitflags;

pub struct SymbolStream {
    pub header: SymbolStreamHeader,
    pub symbols: Vec<Symbol>,
    pub data_types: Vec<DataType>,
}

#[derive(Debug, Copy, Clone)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

#[derive(Debug, Copy, Clone)]
pub struct SymbolStreamHeader {
    pub version: Version,
    pub len_header: u16,
    pub num_symbols: u32,
    pub len_symbols: usize,
    pub num_data_types: u32,
    pub len_data_types: usize,
    pub used_dynamic_symbols: u32,
    pub code_page: u32,
    pub flags: StreamFlags,
    pub layout: Uuid,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct StreamFlags2: u32 {
        const OnlineChange = 0b00000001;
        const Target64Bit = 0b00000010;
        const BaseTypesIncluded = 0b00000100;
        const PerformQSort = 0b00000100;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct StreamFlags {
    pub is_online_change: bool,
    pub is_target_64_bit: bool,
    pub are_base_types_included: bool,
    pub perform_q_sort: bool,
}

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum StreamFlags3 {
    OnlineChange = 0x01,
    Target64Bit = 0x02,
    BaseTypesIncluded = 0x04,
    PerformQSort = 0x08,
}

#[derive(Debug, Clone)]
pub enum AdsDataType {
    Void = 0,
    Bit = 33,
    Int8 = 16,
    UInt8 = 17,
    Int16 = 2,
    UInt16 = 18,
    Int32 = 3,
    UInt32 = 19,
    Int64 = 20,
    UInt64 = 21,
    Real32 = 4,
    Real64 = 5,
    Real80 = 32,
    String = 30,
    WString = 31,
    MaxTypes = 34,
    BigType = 65,
}

impl TryFrom<u32> for AdsDataType {
    type Error = ();

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            x if x == AdsDataType::Void as u32 => Ok(AdsDataType::Void),
            x if x == AdsDataType::Bit as u32 => Ok(AdsDataType::Bit),
            x if x == AdsDataType::Int8 as u32 => Ok(AdsDataType::Int8),
            x if x == AdsDataType::UInt8 as u32 => Ok(AdsDataType::UInt8),
            x if x == AdsDataType::Int16 as u32 => Ok(AdsDataType::Int16),
            x if x == AdsDataType::UInt16 as u32 => Ok(AdsDataType::UInt16),
            x if x == AdsDataType::Int32 as u32 => Ok(AdsDataType::Int32),
            x if x == AdsDataType::UInt32 as u32 => Ok(AdsDataType::UInt32),
            x if x == AdsDataType::Int64 as u32 => Ok(AdsDataType::Int64),
            x if x == AdsDataType::UInt64 as u32 => Ok(AdsDataType::UInt64),
            x if x == AdsDataType::Real32 as u32 => Ok(AdsDataType::Real32),
            x if x == AdsDataType::Real64 as u32 => Ok(AdsDataType::Real64),
            x if x == AdsDataType::Real80 as u32 => Ok(AdsDataType::Real80),
            x if x == AdsDataType::String as u32 => Ok(AdsDataType::String),
            x if x == AdsDataType::WString as u32 => Ok(AdsDataType::WString),
            x if x == AdsDataType::MaxTypes as u32 => Ok(AdsDataType::MaxTypes),
            x if x == AdsDataType::BigType as u32 => Ok(AdsDataType::BigType),
            _ => Err(()),
        }
    }
}

pub enum SymbolFlags {}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub index_group: u32,
    pub index_offset: u32,
    pub len: usize,
    pub data_type: AdsDataType,
    pub flags: u32, //SymbolFlags,
    pub name: String,
    pub data_type_name: String,
    pub comment: String,
    pub data_type_guid: Uuid,
}

#[derive(Debug, Clone)]
pub struct DataType {
    pub name: String,
}

fn parse(stream: &mut Bytes) -> Result<SymbolStream, Box<dyn Error>> {
    let header = parse_header(stream)?;

    let mut symbol_data = stream.copy_to_bytes(header.len_symbols);
    let symbols = parse_symbols(header.num_symbols, &mut symbol_data)?;

    let mut data_type_data = stream.copy_to_bytes(header.len_data_types);
    let data_types = parse_data_types(header.num_data_types, &mut data_type_data)?;

    Ok(SymbolStream {
        header,
        symbols,
        data_types,
    })
}

fn parse_header(stream: &mut Bytes) -> Result<SymbolStreamHeader, Box<dyn Error>> {
    let major = stream.get_u8();
    let minor = stream.get_u8();
    let version = Version { major, minor };

    let len_header = stream.get_u16_le();
    let num_symbols = stream.get_u32_le();
    let len_symbols = stream.get_u32_le().try_into().unwrap();
    let num_data_types = stream.get_u32_le();
    let len_data_types = stream.get_u32_le().try_into().unwrap();
    let used_dynamic_symbols = stream.get_u32_le();
    let code_page = stream.get_u32_le();

    stream.advance(16);

    let layout = stream.get_u128_le();
    let layout = Builder::from_u128(layout).into_uuid();

    let flags = stream.get_u32_le();
    let is_online_change = flags & 0x01 > 0;
    let is_target_64_bit = flags & 0x02 > 0;
    let are_base_types_included = flags & 0x04 > 0;
    let perform_q_sort = flags & 0x08 > 0;

    let flags = StreamFlags {
        is_online_change,
        is_target_64_bit,
        are_base_types_included,
        perform_q_sort,
    };

    Ok(SymbolStreamHeader {
        version,
        len_header,
        num_symbols,
        len_symbols,
        num_data_types,
        len_data_types,
        used_dynamic_symbols,
        code_page,
        flags,
        layout,
    })
}

fn parse_symbols(count: u32, stream: &mut Bytes) -> Result<Vec<Symbol>, Box<dyn Error>> {
    println!("Parsing {count} symbols");
    let mut symbols = vec![];
    for i in 0..count {
        println!("Parsing symbol index {i}");

        let mut tmp = stream.clone();
        let symbol_data_len = tmp.get_u32_le() as usize;

        let mut symbol_data  = stream.copy_to_bytes(symbol_data_len);

        let symbol = parse_symbol(&mut symbol_data)?;
        symbols.push(symbol);
    }

    Ok(symbols)
}

fn parse_symbol(stream: &mut Bytes) -> Result<Symbol, Box<dyn Error>> {
    stream.advance(4);

    let index_group = stream.get_u32_le();
    println!("IndexGroup {index_group:?}");
    let index_offset = stream.get_u32_le();
    println!("IndexOffset {index_offset:?}");

    let data_len = stream.get_u32_le() as usize;
    println!("DataLen {data_len:?}");

    let data_type = stream.get_u32_le();
    println!("DataType {data_type:?}");
    let data_type = data_type.try_into().unwrap();
    println!("DataType {data_type:?}");

    let flags = stream.get_u32_le();
    println!("Flags {flags:?}");

    let name_len = stream.get_u16_le() as usize;
    println!("NameLen {name_len:?}");
    let data_type_name_len = stream.get_u16_le() as usize;
    println!("DataTypeNameLen {data_type_name_len:?}");
    let comment_len = stream.get_u16_le() as usize;
    println!("CommentLen {comment_len:?}");

    let name = stream.copy_to_bytes(name_len + 1);
    let name = CString::from_vec_with_nul(name.to_vec())?;
    let name = name.to_str()?.to_string();
    println!("Name {name:?}");

    let data_type_name = stream.copy_to_bytes(data_type_name_len + 1);
    let data_type_name = CString::from_vec_with_nul(data_type_name.to_vec())?;
    let data_type_name = data_type_name.to_str()?.to_string();
    println!("DataTypeName {data_type_name:?}");

    let comment = stream.copy_to_bytes(comment_len + 1);
    let comment = CString::from_vec_with_nul(comment.to_vec())?;
    let comment = comment.to_str()?.to_string();
    println!("Comment {comment:?}");

    stream.advance(16);

    // let data_type_guid = stream.get_u128_le();
    // let data_type_guid = Builder::from_u128(data_type_guid).into_uuid();
    let data_type_guid = uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");
    println!("DataTypeGuid {data_type_guid:?}");

    println!();

    Ok(Symbol {
        index_group,
        index_offset,
        len: data_len,
        data_type,
        flags,
        name,
        data_type_name,
        comment,
        data_type_guid,
    })
}

fn parse_data_types(count: u32, stream: &mut Bytes) -> Result<Vec<DataType>, Box<dyn Error>> {
    println!("Parsing {count} data types");
    let mut data_types = vec![];
    for i in 0..count {
        println!("Parsing data type index {i}");

        let mut tmp = stream.clone();
        let data_type_data_len = tmp.get_u32_le() as usize;

        let mut data_type_data  = stream.copy_to_bytes(data_type_data_len);

        let data_type = parse_data_type(&mut data_type_data)?;
        data_types.push(data_type);
    }
    Ok(data_types)
}

fn parse_data_type(_stream: &mut Bytes) -> Result<DataType, Box<dyn Error>> {
    Ok(DataType {
        name: "Bob".to_string()
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("ema.symbol_stream")?;
    let mut data = vec![];
    file.read_to_end(&mut data)?;

    let mut data = Bytes::from(data);

    let symbol_stream = parse(&mut data)?;
    let h = symbol_stream.header;

    println!("Version: {}.{}", h.version.major, h.version.minor);
    println!("Header: {} bytes", h.len_header);
    println!("Symbols: {} ({} bytes)", h.num_symbols, h.len_symbols);
    println!(
        "Data Types: {} ({} bytes)",
        h.num_data_types, h.len_data_types
    );
    println!("Used Dynamic Symbols: {} ", h.used_dynamic_symbols);
    println!("Code Page: {}", h.code_page);
    println!("Flags: {:?}", h.flags);
    println!("Layout: {}", h.layout);

    Ok(())
}
