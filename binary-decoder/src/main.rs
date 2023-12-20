use std::fmt;
use std::fs::File;
use std::io::Read;
use std::{error::Error, ffi::CString};

use bytes::{Buf, Bytes};
use uuid::{Builder, Uuid, uuid};

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

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SymbolStreamHeader {
    pub version: Version,
    pub header_len: u16,
    pub symbol_count: u32,
    pub symbol_data_len: usize,
    pub data_type_count: u32,
    pub data_type_data_len: usize,
    pub used_dynamic_symbols: u32,
    pub code_page: u32,
    pub flags: StreamFlags,
    pub layout: Uuid,
}

#[derive(Debug, Copy, Clone)]
pub struct StreamFlags {
    pub is_online_change: bool,
    pub is_target_64_bit: bool,
    pub are_base_types_included: bool,
    pub perform_q_sort: bool,
}

impl TryFrom<u32> for StreamFlags {
    type Error = ();

    fn try_from(v: u32) -> Result<Self, Self::Error> {

        let is_online_change        = v & 0b0001 != 0;
        let is_target_64_bit        = v & 0b0010 != 0;
        let are_base_types_included = v & 0b0100 != 0;
        let perform_q_sort          = v & 0b1000 != 0;
    
        Ok(StreamFlags {
            is_online_change,
            is_target_64_bit,
            are_base_types_included,
            perform_q_sort,
        })
    }
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
pub struct SymbolFlags {
    pub is_persistent: bool,
    pub is_bit_value: bool,
    pub is_reference_to: bool,
    pub has_type_guid: bool,
    pub is_twincat_com_interface_pointer: bool,
    pub is_read_only: bool,
    pub is_interface_method_access: bool,
    pub is_method_deref: bool,
    pub context_mask: u8,
    pub has_attributes: bool,
    pub is_static: bool,
    pub is_initialised_on_reset: bool,
    pub has_extended_flags: bool,
}

impl TryFrom<u32> for SymbolFlags {
    type Error = ();

    fn try_from(v: u32) -> Result<Self, Self::Error> {

        let is_persistent                    = v & 0b0000_0000_0000_0001 != 0;
        let is_bit_value                     = v & 0b0000_0000_0000_0010 != 0;
        let is_reference_to                  = v & 0b0000_0000_0000_0100 != 0;
        let has_type_guid                    = v & 0b0000_0000_0000_1000 != 0;
        let is_twincat_com_interface_pointer = v & 0b0000_0000_0001_0000 != 0;
        let is_read_only                     = v & 0b0000_0000_0010_0000 != 0;
        let is_interface_method_access       = v & 0b0000_0000_0100_0000 != 0;
        let is_method_deref                  = v & 0b0000_0000_1000_0000 != 0;
        let context_mask                      = ((v & 0b0000_1111_0000_0000) >> 8) as u8;
        let has_attributes                   = v & 0b0001_0010_0000_0000 != 0;
        let is_static                        = v & 0b0010_0000_0000_0000 != 0;
        let is_initialised_on_reset          = v & 0b0100_0000_0000_0000 != 0;
        let has_extended_flags               = v & 0b1000_0000_0000_0000 != 0;
    
        Ok(SymbolFlags {
            is_persistent,
            is_bit_value,
            is_reference_to,
            has_type_guid,
            is_twincat_com_interface_pointer,
            is_read_only,
            is_interface_method_access,
            is_method_deref,
            context_mask,
            has_attributes,
            is_static,
            is_initialised_on_reset,
            has_extended_flags,
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct DataTypeFlags {
    pub is_data_type: bool,
    pub is_data_item: bool,
    pub is_reference_to: bool,
    pub is_method_deref: bool,
    pub is_oversampling_array: bool,
    pub is_bit_value: bool,
    pub is_property_item: bool,
    pub has_type_guid: bool,
    pub is_persistent: bool,
    pub has_copy_mask: bool,
    pub is_twincat_com_interface_pointer: bool,
    pub has_method_infos: bool,
    pub has_attributes: bool,
    pub has_enum_infos: bool,
    pub is_byte_aligned: bool,
    pub is_static: bool,
    pub sp_levels: bool,
    pub ignore_persist: bool,
    pub is_any_size_array: bool,
    pub is_persistant_datatype: bool,
    pub is_initialised_on_result: bool,
}

impl TryFrom<u32> for DataTypeFlags {
    type Error = ();

    fn try_from(v: u32) -> Result<Self, Self::Error> {

        let is_data_type = v & (1 << 0) != 0;
        let is_data_item = v & (1 << 1) != 0;
        let is_reference_to = v & (1 << 2) != 0;
        let is_method_deref = v & (1 << 3) != 0;
        let is_oversampling_array = v & (1 << 4) != 0;
        let is_bit_value = v & (1 << 5) != 0;
        let is_property_item = v & (1 << 6) != 0;
        let has_type_guid = v & (1 << 7) != 0;
        let is_persistent = v & (1 << 8) != 0;
        let has_copy_mask = v & (1 << 9) != 0;
        let is_twincat_com_interface_pointer = v & (1 << 10) != 0;
        let has_method_infos = v & (1 << 11) != 0;
        let has_attributes = v & (1 << 12) != 0;
        let has_enum_infos = v & (1 << 13) != 0;
        let is_byte_aligned = v & (1 << 16) != 0;
        let is_static = v & (1 << 0) != 17;
        let sp_levels = v & (1 << 0) != 18;
        let ignore_persist = v & (1 << 19) != 0;
        let is_any_size_array = v & (1 << 20) != 0;
        let is_persistant_datatype = v & (1 << 21) != 0;
        let is_initialised_on_result = v & (1 << 22) != 0;

        Ok(DataTypeFlags {
            is_data_type,
            is_data_item,
            is_reference_to,
            is_method_deref,
            is_oversampling_array,
            is_bit_value,
            is_property_item,
            has_type_guid,
            is_persistent,
            has_copy_mask,
            is_twincat_com_interface_pointer,
            has_method_infos,
            has_attributes,
            has_enum_infos,
            is_byte_aligned,
            is_static,
            sp_levels,
            ignore_persist,
            is_any_size_array,
            is_persistant_datatype,
            is_initialised_on_result,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub index_group: u32,
    pub index_offset: u32,
    pub len: usize,
    pub data_type: AdsDataType,
    pub flags: SymbolFlags,
    pub name: String,
    pub data_type_name: String,
    pub comment: String,
    pub data_type_guid: Uuid,
}

#[derive(Debug, Clone)]
pub struct DataType {
    pub version: u32,
    
    pub hash_value: u32, //Don't know what this is
    pub type_hash_value: u32, // Don't know what this is
    
    pub data_type_len: u32,
    
    pub offset: u32,
    
    pub base_data_type: AdsDataType,
    
    pub flags: DataTypeFlags,
    
    pub array_dimension_count: u16,
    pub sub_item_count: u16,
    
    pub name: String,
    pub data_type_name: String,
    pub comment: String,
    
    pub array_information: Option<ArrayInformation>,
    pub sub_items: Option<Vec<DataType>>,

    pub guid: Option<Uuid>,
    
//   pub copy_mask:     type: u8
//     repeat: expr
//     repeat-expr: len_data_type
//     if: flags.has_copy_mask
    
    // pub methods: Vec<Method>, // type: methods if flags.has_method_infos
    // pub attributes: Vec<Attribute>,// type: attributes if flags.has_attributes
    
    // pub enums: Vec<Enum>, // type: enums(len_data_type) if flags.has_enum_infos
}

#[derive(Debug, Copy, Clone)]
pub struct ArrayInformation {
    pub lower_bounds: u32,
    pub upper_bounds: u32,
}

fn parse(stream: &mut Bytes) -> Result<SymbolStream, Box<dyn Error>> {
    let header = parse_header(stream)?;

    let mut symbol_data = stream.copy_to_bytes(header.symbol_data_len);
    let symbols = parse_symbols(header.symbol_count, &mut symbol_data)?;

    let mut data_type_data = stream.copy_to_bytes(header.data_type_data_len);
    let data_types = parse_data_types(header.data_type_count, &mut data_type_data)?;

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

    let header_len = stream.get_u16_le();
    let symbol_count = stream.get_u32_le();
    let symbol_data_len = stream.get_u32_le().try_into().unwrap();
    let data_type_count = stream.get_u32_le();
    let data_type_data_len = stream.get_u32_le().try_into().unwrap();
    let used_dynamic_symbols = stream.get_u32_le();
    let code_page = stream.get_u32_le();

    stream.advance(16);

    let layout = stream.get_u128_le();
    let layout = Builder::from_u128(layout).into_uuid();

    let flags = stream.get_u32_le();
    let flags = flags.try_into().unwrap();

    Ok(SymbolStreamHeader {
        version,
        header_len,
        symbol_count,
        symbol_data_len,
        data_type_count,
        data_type_data_len,
        used_dynamic_symbols,
        code_page,
        flags,
        layout,
    })
}

fn parse_symbols(count: u32, stream: &mut Bytes) -> Result<Vec<Symbol>, Box<dyn Error>> {
    // println!("Parsing {count} symbols");
    let mut symbols = vec![];
    for _ in 0..count {
        // println!("Parsing symbol index {i}");

        let mut tmp = stream.clone();
        let symbol_data_len = tmp.get_u32_le() as usize;

        let mut symbol_data  = stream.copy_to_bytes(symbol_data_len);

        let symbol = parse_symbol(&mut symbol_data)?;
        symbols.push(symbol);
    }
    Ok(symbols)
}

fn parse_symbol(stream: &mut Bytes) -> Result<Symbol, Box<dyn Error>> {
    // Skip len since we already know that from parse_symbols()
    stream.advance(4);

    let index_group = stream.get_u32_le();
    // println!("IndexGroup {index_group:?}");
    let index_offset = stream.get_u32_le();
    // println!("IndexOffset {index_offset:?}");

    let data_len = stream.get_u32_le() as usize;
    // println!("DataLen {data_len:?}");

    let data_type = stream.get_u32_le();
    // println!("DataType {data_type:?}");
    let data_type = data_type.try_into().unwrap();
    // println!("DataType {data_type:?}");

    let flags = stream.get_u32_le();
    // println!("Flags {flags:?}");
    let flags = flags.try_into().unwrap();
    // println!("Flags {flags:?}");

    let name_len = stream.get_u16_le() as usize;
    // println!("NameLen {name_len:?}");
    let data_type_name_len = stream.get_u16_le() as usize;
    // println!("DataTypeNameLen {data_type_name_len:?}");
    let comment_len = stream.get_u16_le() as usize;
    // println!("CommentLen {comment_len:?}");

    let name = stream.copy_to_bytes(name_len + 1);
    let name = CString::from_vec_with_nul(name.to_vec())?;
    let name = name.to_str()?.to_string();
    // println!("Name {name:?}");

    let data_type_name = stream.copy_to_bytes(data_type_name_len + 1);
    let data_type_name = CString::from_vec_with_nul(data_type_name.to_vec())?;
    let data_type_name = data_type_name.to_str()?.to_string();
    // println!("DataTypeName {data_type_name:?}");

    let comment = stream.copy_to_bytes(comment_len + 1);
    let comment = CString::from_vec_with_nul(comment.to_vec())?;
    let comment = comment.to_str()?.to_string();
    // println!("Comment {comment:?}");

    stream.advance(16);

    // let data_type_guid = stream.get_u128_le();
    // let data_type_guid = Builder::from_u128(data_type_guid).into_uuid();
    // FIXME - need real implementation that works
    let data_type_guid = uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");
    // println!("DataTypeGuid {data_type_guid:?}");

    // println!();

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
    // println!("Parsing {count} data types");
    let mut data_types = vec![];
    for _ in 0..count {
        // println!("Parsing data type index {i}");

        let mut tmp = stream.clone();
        let data_type_data_len = tmp.get_u32_le() as usize;

        let mut data_type_data  = stream.copy_to_bytes(data_type_data_len);

        let data_type = parse_data_type(&mut data_type_data)?;
        data_types.push(data_type);
    }
    Ok(data_types)
}

fn parse_data_type(stream: &mut Bytes) -> Result<DataType, Box<dyn Error>> {
    // Skip len since we already know that from parse_symbols()
    stream.advance(4);

    let version = stream.get_u32_le();
    // println!("Version {version:?}");

    let hash_value = stream.get_u32_le();
    // println!("HashValue {hash_value:?}");
    let type_hash_value = stream.get_u32_le();
    // println!("TypeHashValue {type_hash_value:?}");

    let data_type_len = stream.get_u32_le();
    // println!("LenDataType {data_type_len:?}");
    let offset = stream.get_u32_le();
    // println!("Offset {offset:?}");

    let base_data_type = stream.get_u32_le();
    // println!("BaseDataType {base_data_type:?}");
    let base_data_type = base_data_type.try_into().unwrap();
    // println!("BaseDataType {base_data_type:?}");

    let flags = stream.get_u32_le();
    // println!("Flags {flags:?}");
    let flags: DataTypeFlags = flags.try_into().unwrap();
    // println!("Flags {flags:?}");

    let name_len = stream.get_u16_le() as usize;
    // println!("NameLen {name_len:?}");
    let data_type_name_len = stream.get_u16_le() as usize;
    // println!("DataTypeNameLen {data_type_name_len:?}");
    let comment_len = stream.get_u16_le() as usize;
    // println!("CommentLen {comment_len:?}");

    let array_dimension_count = stream.get_u16_le();
    let sub_item_count = stream.get_u16_le();

    let name = stream.copy_to_bytes(name_len + 1);
    let name = CString::from_vec_with_nul(name.to_vec())?;
    let name = name.to_str()?.to_string();
    // println!("Name {name:?}");

    let data_type_name = stream.copy_to_bytes(data_type_name_len + 1);
    let data_type_name = CString::from_vec_with_nul(data_type_name.to_vec())?;
    let data_type_name = data_type_name.to_str()?.to_string();
    // println!("DataTypeName {data_type_name:?}");

    let comment = stream.copy_to_bytes(comment_len + 1);
    let comment = CString::from_vec_with_nul(comment.to_vec())?;
    let comment = comment.to_str()?.to_string();
    // println!("Comment {comment:?}");

    let array_information = if array_dimension_count > 0 {
        let lower_bounds = stream.get_u32_le();
        let upper_bounds = stream.get_u32_le();
        Some(ArrayInformation{lower_bounds, upper_bounds})
    } else { None };

    let sub_items = if sub_item_count > 0 {
        // TODO - parse sub items which is just more data types!
        Some(vec![])
    } else { None };

    let guid = if flags.has_type_guid {
        // FIXME - need real implementation
        Some(uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8"))
    } else { None };

    Ok(DataType {
        version,
        hash_value,
        type_hash_value,
        data_type_len,
        offset,
        base_data_type,
        flags,
        array_dimension_count,
        sub_item_count,
        name,
        data_type_name,
        comment,
        array_information,
        sub_items,
        guid,
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("ema.symbol_stream")?;
    let mut data = vec![];
    file.read_to_end(&mut data)?;

    let mut data = Bytes::from(data);

    let symbol_stream = parse(&mut data)?;
    let h = symbol_stream.header;

    println!("Version: {}", h.version);
    println!("Header: {} bytes", h.header_len);
    println!("Symbols: {} ({} bytes)", h.symbol_count, h.symbol_data_len);
    println!(
        "Data Types: {} ({} bytes)",
        h.data_type_count, h.data_type_data_len
    );
    println!("Used Dynamic Symbols: {} ", h.used_dynamic_symbols);
    println!("Code Page: {}", h.code_page);
    println!("Flags: {:?}", h.flags);
    println!("Layout: {}", h.layout);

    Ok(())
}
