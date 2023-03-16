# Dynum: A Dynamic Binary Encoding Library

Dynum is a Rust library for encoding and decoding unsigned integers into a binary format using a dynamic number type. This means that the library selects the appropriate type to represent a given value, based on its magnitude, to reduce the number of bytes needed to store the value.

## Dynum Type

The Dynum type is an enumeration of the supported dynamic number types:

- Byte
- Byte2
- Byte4
- Byte8

Each type is associated with a type information struct that provides details about the type:

- Tag size: The number of bits used for the type tag.
- Tag code: The type tag value used to identify the type in the encoded stream.
- Byte capacity: The number of bytes required to store the value.
- Bit range: The maximum number of bits that can be represented by the type.

## Encoding and Decoding

Dynum provides two main functions for encoding and decoding:

- `encode_into`: Encodes a given number into a binary stream using the appropriate dynamic number type that can represent it. The function takes a number and a closure that accepts a u8. This closure is called for every generated byte. The function returns either the number of generated bytes or an error message if the given value exceeds the supported range.
- `decode_binary_stream`: Decodes a binary stream into a u64 value using the appropriate dynamic number type. Returns `None` if the first read byte doesn't contain a valid tag.

## Compression strategy

The memory-saving technique employed in this library is based on the fact that a Integer may not always require all usually intended bytes of memory to be represented in binary. For example, the number 1 only requires 1 byte to be represented in binary, and the number 128 requires 2 bytes. In contrast, the maximum value of a 64-bit integer requires all 8 bytes to be represented in binary.

To take advantage of this property of unsigned Integers, the library uses a set of four dynamic number types: Byte, Byte2, Byte4, and Byte8. Each dynamic number type has a different capacity in terms of the number of bytes required to represent a given integer. For example, the Byte type can represent integers up to 2^7-1 (i.e., 127) using only one byte, while the Byte8 type can represent numbers up to 2^61 using 8 bytes.

The library determines the appropriate dynamic number type to use for a given integer by testing the integer against each dynamic number type's capacity.

When encoding an integer using a dynamic number type, the library first appends a tag to the binary stream that identifies the type of the dynamic number used. The tag code is appended to the binary stream using a number of bits equal to the tag size. This tag ensures that the appropriate dynamic number type is used when decoding the binary stream.

After the tag is appended to the binary stream, the library appends the encoded integer to the binary stream using the number of bytes required by the chosen dynamic number type. Finally, the binary stream is decoded back into the original integer using the appropriate dynamic number type.

This memory-saving technique reduces the memory consumption required to encode and decode 64-bit integers by using the smallest possible number of bytes for each integer. It also ensures that the appropriate dynamic number type is always used when decoding the binary stream, regardless of the size of the integer being encoded or the number of bytes required to represent it in binary.

## Example Usage

```rust
use dynum::{encode_into, decode_binary_stream};

fn main() {
    let mut buff = Vec::with_capacity(10);
    let val = 1234567890;
    encode_into(val, |a| buff.push(a)).unwrap();
    let decoded_val = decode_binary_stream(&mut buff.iter().cloned()).unwrap();
    assert_eq!(val, decoded_val);
}
```

## Testing
The tests module contains a test that encodes and decodes a range of values using the library and checks if the decoded value matches the original value. The test covers values from 0 to 2^31 - 1, as well as values from 2^61 down to 2^31.