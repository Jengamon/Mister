// Here is where we put the code that actually stores the format to disk.
// Our file format is like this: (Numbers are stored in little endian order)
// 4-byte string:
//  MSFI - Mister File Int (colors are encoded as integers)
//  MSFF - Mister File Float (colors are encoded as IEEE 754 floats)
// Then 3 bytes for format version: (major, minor, patch)

// Sooner or later, we might need to do this in a more refined way, a.k.a. make this a struct...
use std::io::Write;
use byteorder::{LittleEndian, WriteBytesExt};

const FORMAT_VERSION: &'static [u8] = &[0, 1, 0];

/// Represents how the MisterFile is expected to store colors
enum ColorStorage {
     /// Use u8 to store color
    Int,
    /// Use f32 to store color
    Float,
}

/// This struct represents a MISTER project on disk.
struct MisterFile {

}
