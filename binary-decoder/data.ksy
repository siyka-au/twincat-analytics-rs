meta:
  id: data
  endian: le
  bit-endian: le
seq:
  - id: header
    type: data_header
  - id: samples
    type: sample
    repeat: expr
    repeat-expr: header.samples

types:
  # Stream
  data_header:
    seq:
      - id: major_version
        type: u1
      - id: minor_version
        type: u1
      - id: len_header
        type: u1
      - id: len_sample_header
        type: u1
      - id: len_data
        type: u4
      - id: cycle_time
        type: u4
      - id: flags
        type: stream_flags
        doc: Needs further decoding work
      - id: layout
        type: guid
      - id: samples
        type: u8
        if: major_version == 1 and minor_version == 1
      - id: start_time
        type: u8
        if: major_version == 1 and minor_version == 1
      - id: stop_time
        type: u8
        if: major_version == 1 and minor_version == 1
  sample:
    seq:
      - id: header
        type: sample_header
        if: _root.header.flags.sample_timestamp
      - id: a_bool_value
        type: u1
      - id: b_sint_value
        type: s1
      - id: c_usint_value
        type: u1
      - id: d_int_value
        type: s2
      - id: e_uint_value
        type: u2
      - id: f_dint_value
        type: s4
      - id: g_udint_value
        type: u4
      - id: h_lint_value
        type: s8
      - id: i_ulint_value
        type: u8
      - id: j_real_value
        type: f4
      - id: k_lreal_value
        type: f8
      - id: l_string_value
        size: 21
      - id: m_wstring_value
        size: 41
  sample_header:
    seq:
      - id: timestamp
        type: u8

  stream_flags:
    seq:
      - id: head_timestamp
        type: b1
      - id: sample_timestamp
        type: b1
      - id: dc_time
        type: b1
      - id: reserved
        type: b1
      - id: compression_method
        type: b3
        doc: Needs further decoding work
      - id: padding01
        type: b25

  # Supporting Types
  guid:
    seq:
      - id: data1
        type: u4
      - id: data2
        type: u2
      - id: data3
        type: u2
      - id: data4
        type: u4be
      - id: data4a
        type: u4be

# Enums
enums:
  compression_type:
    0: none
    1: run_length
    2: reserver
    
  ads_data_type:
    0: void
    16: int8
    17: uint8
    2: int16
    18: uint16
    3: int32
    19: uint32
    20: int64
    21: uint64
    4: real32
    5: real64
    65: big_type
    30: string
    31: w_string
    32: read80
    33: bit
    34: max_types

  data_category:
    0: none_or_unknown # Uninitialized / NotProcessed (0)
    1: primitive # Simple / Base Data Type (1)
    2: alias # Alias data type (2)
    3: enum # Enumeration data type (3)
    4: array # Array data type (4)
    5: struct # Structure data type (5)
    6: function_block # Function block (POU) (6)
    7: program # Program (POU) (7)
    8: function # Function (POU) (8)
    9: sub_range # SubRange (9)
    10: string # Fixed length string (10)
    12: bitset # Bitset (12)
    13: pointer # Pointer type (13)
    14: union # Union type (14)
    15: reference # Reference type (15)
    16: interface # The interface

  method_parameter_flag_mask:
    # see method_parameter_flag
    5: mask_in
    6: mask_out
