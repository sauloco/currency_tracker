mod exchanges;
mod input_handler;
mod savings;
mod utils;

extern crate dotenv;

#[macro_use]
extern crate dotenv_codegen;
extern crate core;

use crate::exchanges::{get_commission, request_ars_usd_blue, request_usd_eur, ValidCurrencies};
use crate::input_handler::{validate_input_args, InputModes};
use dotenv::dotenv;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();
    let args = validate_input_args(args)?;

    match args {
        InputModes::Simple(d) => {
            let value = if d.ticker == "arsusd".to_string() {
                request_ars_usd_blue(0f64).await?
            } else if d.ticker == "usdeur" {
                request_usd_eur(0f64).await?
            } else {
                0f64
            };
            println!("{:.2}", value);
        }
        InputModes::Detailed(d) => {
            let ars_usd = request_ars_usd_blue(d.ars_usd).await?;

            let currency = d.currency;

            let commission = get_commission(d.commission);

            let mut factor = 1.0;
            if currency == ValidCurrencies::EUR {
                factor = request_usd_eur(d.usd_eur).await?;
            }

            let total = d.amount / (ars_usd * factor) * (1f64 + commission);

            if currency == ValidCurrencies::EUR {
                println!(
                    "€ {:.2} = AR$ {:.2} / (AR$ {:.2} * U$D {}) * (1 + {}%)",
                    total,
                    d.amount,
                    ars_usd,
                    factor,
                    (commission * 100f64).round()
                )
            } else {
                println!(
                    "USD {:.2} = AR$ {:.2} / AR$ {:.2} * (1 + {}%)",
                    total,
                    d.amount,
                    ars_usd,
                    (commission * 100f64).round()
                )
            }
        }
    };

    Ok(())
}
