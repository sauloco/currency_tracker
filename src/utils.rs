pub fn format_str_arg_to_f64(amount: Option<&String>) -> f64 {
    match amount {
        Some(amount) => amount.parse::<f64>().unwrap_or(0f64),
        None => 0f64,
    }
}

#[cfg(test)]
mod tests {
    use super::format_str_arg_to_f64;

    #[test]
    fn validate_valid_str_to_f64() {
        let amount = "15.00".to_string();
        let result = format_str_arg_to_f64(Some(&amount));
        assert_eq!(result, 15f64)
    }

    #[test]
    fn validate_zero_str_to_f64() {
        let amount = "0".to_string();
        let result = format_str_arg_to_f64(Some(&amount));
        assert_eq!(result, 0f64)
    }

    #[test]
    fn validate_invalid_str_to_f64() {
        let amount = "LALA".to_string();
        let result = format_str_arg_to_f64(Some(&amount));
        assert_eq!(result, 0f64)
    }

    #[test]
    fn validate_none_to_f64() {
        let result = format_str_arg_to_f64(None);
        assert_eq!(result, 0f64)
    }
}
