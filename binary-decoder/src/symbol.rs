/// Represents a symbol in the PLC memory.
#[derive(Debug, Copy, Clone, PackedStruct)]
#[packed_struct(endian = "lsb")]
pub struct Symbol {
    /// Hierarchical name of the symbol.
    pub name: String,
    /// Index group of the symbol location.
    pub ix_group: u32,
    /// Index offset of the symbol location.
    pub ix_offset: u32,
    /// Type name of the symbol.
    pub typ: String,
    /// Total size of the symbol, in bytes.
    pub size: usize,
    /// Base type:
    /// - 0 - void
    /// - 2 - INT (i16)
    /// - 3 - DINT (i32)
    /// - 4 - REAL (f32)
    /// - 5 - LREAL (f64)
    /// - 16 - SINT (i8)
    /// - 17 - USINT/BYTE (u8)
    /// - 18 - UINT/WORD (u16)
    /// - 19 - UDINT/DWORD (u32)
    /// - 20 - LINT (i64)
    /// - 21 - ULINT/LWORD (u64)
    /// - 30 - STRING
    /// - 31 - WSTRING
    /// - 32 - REAL80 (f80)
    /// - 33 - BOOL (u1)
    /// - 65 - Other/Compound type
    pub base_type: u32,
    /// Symbol flags:
    /// - 0x01 - Persistent
    /// - 0x02 - Bit value
    /// - 0x04 - Reference to
    /// - 0x08 - Type GUID present
    /// - 0x10 - TComInterfacePtr
    /// - 0x20 - Read only
    /// - 0x40 - ITF method access
    /// - 0x80 - Method deref
    /// - 0x0F00 - Context mask
    /// - 0x1000 - Attributes present
    /// - 0x2000 - Static
    /// - 0x4000 - Init on reset
    /// - 0x8000 - Extended flags present
    pub flags: u32,
}
