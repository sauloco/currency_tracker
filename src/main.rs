mod exchanges;
mod input_handler;

extern crate dotenv;

#[macro_use]
extern crate dotenv_codegen;
extern crate core;

use crate::exchanges::{get_commission, request_ars_usd_blue, request_usd_eur, ValidCurrencies};
use crate::input_handler::validate_input_args;
use dotenv::dotenv;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let args = validate_input_args()?;

    let ars_usd = request_ars_usd_blue(args.ars_usd).await?;

    let currency = args.currency;

    let commission = get_commission(args.commission);

    let mut factor = 1.0;
    if currency == ValidCurrencies::EUR {
        factor = request_usd_eur(args.usd_eur).await?;
    }

    let total = args.amount / (ars_usd * factor) * (1f64 + commission);

    if currency == ValidCurrencies::EUR {
        println!(
            "€ {:.2} = AR$ {:.2} / (AR$ {:.2} * U$D {}) * (1 + {}%)",
            total,
            args.amount,
            ars_usd,
            factor,
            (commission * 100f64).round()
        )
    } else {
        println!(
            "USD {:.2} = AR$ {:.2} / AR$ {:.2} * (1 + {}%)",
            total,
            args.amount,
            ars_usd,
            (commission * 100f64).round()
        )
    }

    Ok(())
}
