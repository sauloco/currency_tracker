use crate::exchanges::ValidCurrencies;
use std::convert::TryFrom;
use std::env;
use std::error::Error;

#[derive(Debug)]
pub struct ArgsFormat {
    pub amount: f64,
    pub ars_usd: f64,
    pub usd_eur: f64,
    pub currency: ValidCurrencies,
    pub commission: f64,
}

pub fn validate_input_args() -> Result<ArgsFormat, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let amount: &str = args.get(1).ok_or("No amount provided")?;
    let amount = amount.parse::<f64>()?;

    if amount <= 0f64 {
        return Err(Box::try_from("First argument must be a number bigger than 0").unwrap());
    }
    let currency = validate_currency(args.get(2));

    let ars_usd = format_str_arg_to_f64(args.get(3));

    let usd_eur = format_str_arg_to_f64(args.get(4));

    let commission = format_str_arg_to_f64(args.get(5));

    Ok(ArgsFormat {
        amount,
        currency,
        ars_usd,
        usd_eur,
        commission,
    })
}

fn format_str_arg_to_f64(amount: Option<&String>) -> f64 {
    match amount {
        Some(amount) => amount.parse::<f64>().unwrap_or(0f64),
        None => 0f64,
    }
}

fn validate_currency(currency: Option<&String>) -> ValidCurrencies {
    match currency {
        Some(currency) => match currency.trim() {
            "EUR" => ValidCurrencies::EUR,
            "USD" => ValidCurrencies::USD,
            value => {
                println!("Currency {} is not supported, using USD", value);
                ValidCurrencies::USD
            }
        },
        _ => ValidCurrencies::USD,
    }
}
