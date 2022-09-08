pub fn push_le_u16(value: u16, output: &mut Vec<u8>) {
    let mut buff: [u8; 2] = Default::default();
    buff[0] = (value & 0xFF) as u8;
    buff[1] = ((value >> 8) & 0xFF) as u8;
    output.extend(buff)
}

pub fn push_le_u32(value: u32, output: &mut Vec<u8>) {
    let mut buff: [u8; 4] = Default::default();
    buff[0] = (value & 0xFF) as u8;
    buff[1] = ((value >> 8) & 0xFF) as u8;
    buff[2] = ((value >> 16) & 0xFF) as u8;
    buff[3] = ((value >> 24) & 0xFF) as u8;
    output.extend(buff)
}

pub fn push_le_u64(value: u64, output: &mut Vec<u8>) {
    let mut buff: [u8; 8] = Default::default();
    buff[0] = (value & 0xFF) as u8;
    buff[1] = ((value >> 8) & 0xFF) as u8;
    buff[2] = ((value >> 16) & 0xFF) as u8;
    buff[3] = ((value >> 24) & 0xFF) as u8;
    buff[4] = ((value >> 32) & 0xFF) as u8;
    buff[5] = ((value >> 40) & 0xFF) as u8;
    buff[6] = ((value >> 48) & 0xFF) as u8;
    buff[7] = ((value >> 56) & 0xFF) as u8;
    output.extend(buff)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_le_u32() {
        let mut output = vec![];
        push_le_u32(0x1A2B3C4D, &mut output);
        assert_eq!(output, [0x4D, 0x3c, 0x2B, 0x1A]);
    }
}