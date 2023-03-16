struct DynNumTypeInfo {
    tagsize: u8,
    tagcode: u8,
    bytecapacity: u8,
    bitrange: u8,
}

enum DynNumType {
    Byte,
    Byte2,
    Byte4,
    Byte8,
}

impl DynNumType {
    fn as_list() -> [DynNumType; 4] {
        return [Self::Byte, Self::Byte2, Self::Byte4,Self::Byte8];
    }

    fn max_value() -> u64 {
        1 << Self::max_value_info().bitrange
    }

    fn max_value_info() -> DynNumTypeInfo{
        Self::as_list().last().unwrap().get_typeinfo()
    }

    const fn get_typeinfo(&self) -> DynNumTypeInfo {
        match self {
            DynNumType::Byte => DynNumTypeInfo {
                tagsize: 1,
                tagcode: 0b001,
                bytecapacity: 1,
                bitrange: 7, //8 bit - tagsize
            },
            DynNumType::Byte2 => DynNumTypeInfo {
                tagsize: 2,
                tagcode: 0b010,
                bytecapacity: 2,
                bitrange: 14, //16 bit - tagsize
            },
            DynNumType::Byte4 => DynNumTypeInfo {
                tagsize: 3,
                tagcode: 0b100,
                bytecapacity: 4,
                bitrange: 29, //32 bit - tagsize
            },
            DynNumType::Byte8 => DynNumTypeInfo {
                tagsize: 3,
                tagcode: 0b000,
                bytecapacity: 8,
                bitrange: 61, //64 bit - tagsize
            },
        }
    }
}

/// This function encodes the given number into a binary stream using the dynamic number type that can represent it.
/// It takes the number and a closure that accepts a u8.
/// This closure gets called for every generated byte
/// It returns either the number of generated bytes or an error message if the given value exceeds the supported range.
pub fn encode_into(mut num: u64, mut target: impl FnMut(u8)) -> Result<u8,&'static str> {
    let typ = DynNumType::as_list()
        .into_iter()
        .find(|d| {
            let info = d.get_typeinfo();
            num >> info.bitrange == 0
        })
        .ok_or("Given Value exceeds the supported range of 2^61 bits")?;

    let typdata = typ.get_typeinfo();

    //Insert tagdata
    num <<= typdata.tagsize;
    num |= typdata.tagcode as u64;

    let bts = num.to_le_bytes();
    for n in 0..typdata.bytecapacity {
        target(bts[n as usize])
    }
    Ok(typdata.bytecapacity)
}


/// Decode a binary stream into a u64 value using the appropriate dynamic number type.
/// Returns none if the first read byte doesn't contains a valid tag
pub fn decode_binary_stream(input: &mut impl Iterator<Item = u8>) -> Option<u64> {
    let mut first = input.next().unwrap();

    let typ = DynNumType::as_list().into_iter().find(|d| {
        let info = d.get_typeinfo();
        let invts = 8 - info.tagsize;
        (first << invts) >> invts == info.tagcode
    })?;
    let typdata = typ.get_typeinfo();

    let mut buff = [0u8; std::mem::size_of::<usize>()];
    buff[0] = first;
    for i in 0..typdata.bytecapacity - 1 {
        buff[(i + 1) as usize] = input.next().unwrap();
    }

    //Remove tagdata and decode
    Some(u64::from_le_bytes(buff) >> typdata.tagsize)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut buff = Vec::with_capacity(10);
        for i in 0..1<<31{
            let i = i as u64;
            buff.clear();
            encode_into(i, |a| buff.push(a));
            let result = decode_binary_stream(&mut buff.iter().cloned())
                .expect("Given value outside of range");
            assert_eq!(i, result);
        }
        println!("Testing from upper bound");
        for i in (0..DynNumType::max_value()).rev().step_by(1<<32){
            let i = i as u64;
            buff.clear();
            encode_into(i, |a| buff.push(a));
            let result = decode_binary_stream(&mut buff.iter().cloned())
                .expect("Given value outside of range");
            assert_eq!(i, result);
        }
    }
}
