use primitive_types::U256;

use super::block::Block;

pub const TWO_WEEKS: u32 = 60*60*24*14;

// Takes little endian, returns big endian
pub fn bits_to_target(mut bits: [u8; 4]) -> U256 {
    // Convert from little-endian to big-endian
    bits.reverse();

    // Extract the exponent and coefficient
    let exponent = bits[0] as u32;
    let coefficient = ((bits[1] as u32) << 16) | ((bits[2] as u32) << 8) | (bits[3] as u32);

    // Handle special case where coefficient is zero
    if coefficient == 0 {
        return U256::zero();
    }

    // Calculate the target
    U256::from(coefficient) * U256::from(256).pow(U256::from(exponent - 3))
}

// Takes big endian, returns little endian
pub fn target_to_bits(target: U256) -> [u8; 4] {
    let raw_bytes = target.to_big_endian();
    let first_nonzero = raw_bytes.iter().position(|&x| x != 0).unwrap_or(0);
    let raw_bytes = &raw_bytes[first_nonzero..];
    let exponent: usize;
    let mut coefficient = vec![];
    if raw_bytes[0] > 0x7f {
        exponent = raw_bytes.len() + 1;
        coefficient.extend_from_slice(&[0x00]);
        coefficient.extend_from_slice(&raw_bytes[..2]);
    } else {
        exponent = raw_bytes.len();
        coefficient.extend_from_slice(&raw_bytes[..3]);
    }
    let mut new_bits = coefficient.clone();
    new_bits.reverse();
    new_bits.push(exponent.try_into().unwrap());
    let new_bits: [u8; 4] = new_bits.try_into().unwrap();
    new_bits
}

pub fn calculate_new_bits(first_block: Block, last_block: Block) -> [u8; 4] {
    let first_timestamp = u32::from_le_bytes(first_block.timestamp);
    let last_timestamp = u32::from_le_bytes(last_block.timestamp);
    dbg!(first_timestamp);
    dbg!(last_timestamp);

    // Clamp the time_differential and find the new target
    let mut time_differential = last_timestamp - first_timestamp;
    dbg!(time_differential);

    if time_differential > TWO_WEEKS * 4 {
        time_differential = TWO_WEEKS * 4;
    } else if time_differential < TWO_WEEKS / 4 {
        time_differential = TWO_WEEKS / 4
    }

    let new_target = last_block.target() * time_differential / TWO_WEEKS;
    // let formatted_new_target = format!("{:064x}", new_target);
    // println!("{}", formatted_new_target); // 0000000000000000018d30aa2cdc67600f9a9342cdc67600f9a9342cdc67600f

    target_to_bits(new_target)
}

pub fn calculate_new_bits_from_previous(previous_bits: [u8; 4], mut time_differential: u32) -> [u8; 4] {
    // Clamp time differential between 1/4 and 4 weeks
    if time_differential > TWO_WEEKS * 4 {
        time_differential = TWO_WEEKS * 4;
    } else if time_differential < TWO_WEEKS / 4 {
        time_differential = TWO_WEEKS / 4;
    }

    // Convert previous bits to target and calculate new target
    let previous_target = bits_to_target(previous_bits);
    let new_target = previous_target * U256::from(time_differential) / U256::from(TWO_WEEKS);

    // Convert new target back to bits format
    target_to_bits(new_target)
}
