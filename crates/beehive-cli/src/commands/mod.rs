//! Subcommand implementations.

pub mod devices;
pub mod permit_join;
pub mod scan;
pub mod send;

/// Decode a hex string into bytes.
///
/// Accepts an optional `0x` prefix and ignores spaces, `:` and `-`.
pub fn parse_hex(input: &str) -> anyhow::Result<Vec<u8>> {
    let trimmed = input.trim();
    let trimmed = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
        .unwrap_or(trimmed);

    let digits: String = trimmed
        .chars()
        .filter(|c| !matches!(c, ':' | '-' | ' ' | '_'))
        .collect();

    if digits.len() % 2 != 0 {
        anyhow::bail!("hex payload has an odd number of digits");
    }

    let mut bytes = Vec::with_capacity(digits.len() / 2);
    for index in (0..digits.len()).step_by(2) {
        let byte = u8::from_str_radix(&digits[index..index + 2], 16)
            .map_err(|_| anyhow::anyhow!("invalid hex byte at offset {index}"))?;
        bytes.push(byte);
    }
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_plain_hex() {
        assert_eq!(parse_hex("0102ff").unwrap(), vec![0x01, 0x02, 0xff]);
    }

    #[test]
    fn decodes_prefixed_and_separated() {
        assert_eq!(parse_hex("0x01:02-ff").unwrap(), vec![0x01, 0x02, 0xff]);
    }

    #[test]
    fn rejects_odd_length() {
        assert!(parse_hex("abc").is_err());
    }
}
