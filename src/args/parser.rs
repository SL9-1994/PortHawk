/// Represents a port range, which can be either a single port or a range of ports.
#[derive(Debug, Clone, PartialEq)]
pub enum PortRange {
    Range(Vec<(u16, u16)>),
    Single(u16),
}

/// Parses a string representation of a port range and returns a `PortRange` enum.
///
/// # Arguments
///
/// * `target_ports` - A string representing the target port range.
///
/// # Returns
///
/// * `Ok(PortRange)` - If the parsing is successful, returns a `PortRange` enum.
/// * `Err(String)` - If the parsing fails, returns an error message.
pub fn parse_port_range(target_ports: String) -> Result<PortRange, String> {
    // Single port
    if !target_ports.contains('-') && !target_ports.contains(',') {
        let port = target_ports
            .parse::<u16>()
            .map_err(|_| format!("Invalid port: {}", target_ports))?;
        return Ok(PortRange::Single(port));
    }

    // Range port
    let ranges: Vec<&str> = target_ports.split(',').collect();
    let mut port_ranges = Vec::new();

    for range in ranges {
        let ports: Vec<&str> = range.split('-').collect();
        if ports.len() != 2 {
            return Err(format!("Invalid port range: {}", range));
        }

        let start = ports[0]
            .trim()
            .parse::<u16>()
            .map_err(|_| format!("Invalid start port: {}", ports[0]))?;
        let end = ports[1]
            .trim()
            .parse::<u16>()
            .map_err(|_| format!("Invalid end port: {}", ports[1]))?;

        if start > end {
            return Err(format!("Start port is greater than end port: {}", range));
        }

        port_ranges.push((start, end));
    }

    match port_ranges.len() {
        1 => Ok(PortRange::Range(port_ranges)),
        _ => Ok(PortRange::Range(port_ranges)),
    }
}

// Unit tests >------------------------------------------------------------<
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_port_range_single_port() {
        // Single port
        let result = parse_port_range("8080".to_string());
        assert_eq!(result, Ok(PortRange::Single(8080)));
    }

    #[test]
    fn test_parse_port_range_invalid_single_port() {
        // Invalid single port
        let result = parse_port_range("abc".to_string());
        assert_eq!(result, Err("Invalid port: abc".to_string()));
    }

    #[test]
    fn test_parse_port_invalid_start_port() {
        let result: Result<PortRange, String> = parse_port_range("a-1000".to_string());
        assert_eq!(result, Err("Invalid start port: a".to_string()));
    }

    #[test]
    fn test_parse_port_invalid_end_port() {
        let result: Result<PortRange, String> = parse_port_range("1000-b".to_string());
        assert_eq!(result, Err("Invalid end port: b".to_string()));
    }

    #[test]
    fn test_parse_port_range_range_ports() {
        // Range ports
        let result = parse_port_range("8000-8080".to_string());
        assert_eq!(result, Ok(PortRange::Range(vec![(8000, 8080)])));
    }

    #[test]
    fn test_parse_port_range_invalid_range_ports() {
        // Invalid range ports
        let result = parse_port_range("8000-9000-10000".to_string());
        assert_eq!(
            result,
            Err("Invalid port range: 8000-9000-10000".to_string())
        );
    }

    #[test]
    fn test_parse_port_range_start_greater_than_end() {
        // Start port greater than end port
        let result = parse_port_range("8080-8000".to_string());
        assert_eq!(
            result,
            Err("Start port is greater than end port: 8080-8000".to_string())
        );
    }

    #[test]
    fn test_parse_port_range_multiple_ranges() {
        // Multiple ranges
        let result = parse_port_range("8000-8080,9000-9090".to_string());
        assert_eq!(
            result,
            Ok(PortRange::Range(vec![(8000, 8080), (9000, 9090)]))
        );
    }
}
