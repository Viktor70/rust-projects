#[derive(Debug, PartialEq)]
pub struct Memory {
    page_size: Option<u32>,
    physical: Option<u32>,
    physical_free: Option<u32>,
    swap: Option<u32>,
    swap_free: Option<u32>,
}
impl Memory {
    fn parse_numbers(input: &str) -> Option<u32> {
        //  49021208k
        if input.ends_with('k') {
            if let Ok(output) = input[..input.len() - 1].parse::<u32>() {
                return Some(output);
            }
        }
        None
    }

    fn parse_page_size(input: &str) -> Option<u32> {
        //  Memory: 4k page
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.len() >= 2 && parts[0] == "Memory:" {
            let size_part = parts[1];
            let total = Self::parse_numbers(size_part)?;
            return Some(total);
        }
        None
    }

    fn parse_memory(input: &str) -> Option<(u32, u32)> {
        //  physical 49021208k(38608508k free)
        //  swap 2097148k(1136376k free)
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() < 3 {
            return None;
        }

        let (total_part, rest) = parts[1].split_once('(')?;
        let total = Self::parse_numbers(total_part)?;
        let free = Self::parse_numbers(rest)?;
        Some((total, free))
    }

    pub fn parse_memory_info(s: &str) -> Option<Memory> {
        let parts: Vec<&str> = s.split(",").collect();
        let mut page_size = None;
        let mut physical = None;
        let mut physical_free = None;
        let mut swap = None;
        let mut swap_free = None;

        for part in parts {
            if part.contains("Memory") {
                page_size = Some(Self::parse_page_size(part)?);
            } else if part.contains("physical") {
                let (p, pf) = Self::parse_memory(part)?;
                physical = Some(p);
                physical_free = Some(pf);
            } else if part.contains("swap") {
                let (s, sf) = Self::parse_memory(part)?;
                swap = Some(s);
                swap_free = Some(sf);
            } else {
                return None;
            }
        }

        Some(Memory {
            page_size,
            physical,
            physical_free,
            swap,
            swap_free,
        })
    }
}
// Тест
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_memory_info_valid_input() {
        let input =
            "Memory: 4k page, physical 49021208k(38608508k free), swap 2097148k(2097148k free)";
        let expected = Memory {
            page_size: Some(4),
            physical: Some(49021208),
            physical_free: Some(38608508),
            swap: Some(2097148),
            swap_free: Some(2097148),
        };
        let result = Memory::parse_memory_info(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_memory_info_different_page_size() {
        let input =
            "Memory: 8k page, physical 10000000k(5000000k free), swap 2000000k(1000000k free)";
        let expected = Memory {
            page_size: Some(8),
            physical: Some(10000000),
            physical_free: Some(5000000),
            swap: Some(2000000),
            swap_free: Some(1000000),
        };
        let result = Memory::parse_memory_info(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_memory_info_empty_swap() {
        let input = "Memory: 4k page, physical 49021208k(38608508k free), swap 0k(0k free)";
        let expected = Memory {
            page_size: Some(4),
            physical: Some(49021208),
            physical_free: Some(38608508),
            swap: Some(0),
            swap_free: Some(0),
        };
        let result = Memory::parse_memory_info(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_memory_info_invalid_format() {
        let input = "Invalid format";
        assert!(Memory::parse_memory_info(input).is_none());
    }

    #[test]
    fn test_parse_memory_info_missing_parts() {
        let input = "Memory: 4k page, physical 49021208k(38608508k free)";
        let expected = Memory {
            page_size: Some(4),
            physical: Some(49021208),
            physical_free: Some(38608508),
            swap: None,
            swap_free: None,
        };
        let result = Memory::parse_memory_info(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_memory_info_invalid_numbers() {
        let input =
            "Memory: 4k page, physical invalid(38608508k free), swap 2097148k(2097148k free)";
        assert!(Memory::parse_memory_info(input).is_none());
    }
}
