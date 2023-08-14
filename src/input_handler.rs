use crate::exchanges::ValidCurrencies;
use crate::utils::format_str_arg_to_f64;
use std::convert::TryFrom;
use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum InputModes {
    Simple(Simple),
    Detailed(Detailed),
}

#[derive(Debug, PartialEq)]
pub struct Simple {
    pub ticker: String,
}

#[derive(Debug, PartialEq)]
pub struct Detailed {
    pub amount: f64,
    pub ars_usd: f64,
    pub usd_eur: f64,
    pub currency: ValidCurrencies,
    pub commission: f64,
}

pub fn validate_input_args(args: Vec<String>) -> Result<InputModes, Box<dyn Error>> {
    let amount: &str = args
        .get(1)
        .ok_or("An amount value or 'rate' must be the first argument")?;

    if amount.trim() == "rate" {
        let ticker: &str = args.get(2).ok_or("No ticker in second argument")?;
        return Ok(InputModes::Simple(Simple {
            ticker: ticker.to_string(),
        }));
    }
    let amount = amount.parse::<f64>()?;

    if amount <= 0f64 {
        return Err(Box::try_from("First argument must be a number bigger than 0").unwrap());
    }
    let currency = args.get(2).ok_or("")?;
    let currency = validate_currency(currency);

    let ars_usd = format_str_arg_to_f64(args.get(3));

    let usd_eur = format_str_arg_to_f64(args.get(4));

    let commission = format_str_arg_to_f64(args.get(5));

    Ok(InputModes::Detailed(Detailed {
        amount,
        currency,
        ars_usd,
        usd_eur,
        commission,
    }))
}

fn validate_currency(currency: &String) -> ValidCurrencies {
    match currency.trim().to_uppercase().as_str() {
        "EUR" => ValidCurrencies::EUR,
        "USD" => ValidCurrencies::USD,
        value => {
            println!("Currency {} is not supported, using USD", value);
            ValidCurrencies::USD
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{validate_currency, validate_input_args, ValidCurrencies};
    use crate::input_handler::{Detailed, InputModes, Simple};

    #[test]
    fn validate_currency_as_eur() {
        let currency = "EUR".to_string();
        assert_eq!(validate_currency(&currency), ValidCurrencies::EUR)
    }

    #[test]
    fn validate_currency_as_usd() {
        let currency = "USD".to_string();
        assert_eq!(validate_currency(&currency), ValidCurrencies::USD)
    }

    #[test]
    fn validate_currency_fallback() {
        let currency = "LALA".to_string();
        assert_eq!(validate_currency(&currency), ValidCurrencies::USD)
    }

    #[test]
    fn validate_currency_empty() {
        let currency = "".to_string();
        assert_eq!(validate_currency(&currency), ValidCurrencies::USD)
    }

    #[test]
    fn validate_validate_input_args_rate_usdeur() {
        let args = vec![
            "always ignored".to_string(),
            "rate".to_string(),
            "usdeur".to_string(),
        ];
        let result = validate_input_args(args).unwrap();

        assert_eq!(
            result,
            InputModes::Simple(Simple {
                ticker: "usdeur".to_string()
            })
        );
    }

    #[test]
    fn validate_validate_input_args_rate_arsusd() {
        let args = vec![
            "always ignored".to_string(),
            "rate".to_string(),
            "arsusd".to_string(),
        ];
        let result = validate_input_args(args).unwrap();

        assert_eq!(
            result,
            InputModes::Simple(Simple {
                ticker: "arsusd".to_string()
            })
        );
    }

    #[test]
    fn validate_validate_input_args_get_100000_as_eur() {
        let args = vec![
            "always ignored".to_string(),
            "100000".to_string(),
            "eur".to_string(),
        ];
        let result = validate_input_args(args).unwrap();

        assert_eq!(
            result,
            InputModes::Detailed(Detailed {
                amount: 100000f64,
                currency: ValidCurrencies::EUR,
                ars_usd: 0f64,
                usd_eur: 0f64,
                commission: 0f64,
            })
        )
    }

    #[test]
    fn validate_validate_input_args_get_100000_as_usd() {
        let args = vec![
            "always ignored".to_string(),
            "100000".to_string(),
            "usd".to_string(),
        ];
        let result = validate_input_args(args).unwrap();

        assert_eq!(
            result,
            InputModes::Detailed(Detailed {
                amount: 100000f64,
                currency: ValidCurrencies::USD,
                ars_usd: 0f64,
                usd_eur: 0f64,
                commission: 0f64,
            })
        )
    }

    #[test]
    fn validate_validate_input_args_set_all() {
        let args = vec![
            "always ignored".to_string(),
            "100000".to_string(),
            "usd".to_string(),
            "500".to_string(),
            "1.1".to_string(),
            "6".to_string(),
        ];
        let result = validate_input_args(args).unwrap();

        assert_eq!(
            result,
            InputModes::Detailed(Detailed {
                amount: 100000f64,
                currency: ValidCurrencies::USD,
                ars_usd: 500f64,
                usd_eur: 1.1f64,
                commission: 6f64,
            })
        )
    }

    #[test]
    fn validate_validate_input_args_invalid_first_arg() {
        let args = vec![
            "always ignored".to_string(),
            "not_rate_string_nor_a_number".to_string(),
        ];

        let result = validate_input_args(args);

        assert!(result.is_err())
    }

    #[test]
    fn validate_validate_input_args_invalid_second_arg() {
        let args = vec!["always ignored".to_string(), "rate".to_string()];

        let result = validate_input_args(args);

        assert!(result.is_err())
    }
}
