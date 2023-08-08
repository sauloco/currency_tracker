extern crate reqwest;
extern crate roxmltree;

use roxmltree::Node;
use serde_json::Value;
use std::error::Error;

#[derive(PartialEq, Debug)]
pub enum ValidCurrencies {
    USD,
    EUR,
}

pub fn get_commission(provided_commission: f64) -> f64 {
    if provided_commission > 0f64 {
        return provided_commission;
    }

    let commission = dotenv!("current_commission");

    commission.parse::<f64>().unwrap_or(1f64)
}

pub async fn request_ars_usd_blue(provided_ars_usd: f64) -> Result<f64, Box<dyn Error>> {
    if provided_ars_usd > 0.0 {
        return Ok(provided_ars_usd);
    }

    let usd_rate_provider = dotenv!("blue_usd_provider_url");

    let result: Value = reqwest::get(usd_rate_provider).await?.json().await?;

    let blue = &result["blue"]["value_buy"];
    Ok(blue.as_f64().unwrap_or(0f64))
}

pub async fn request_usd_eur(provided_usd_eur: f64) -> Result<f64, Box<dyn Error>> {
    if provided_usd_eur > 0f64 {
        return Ok(provided_usd_eur);
    }

    let usd_rate_provider = dotenv!("eur_usd_provider_url");

    let result: String = reqwest::get(usd_rate_provider).await?.text().await?;

    let result = find_usd_eur(result);

    Ok(result)
}

fn find_usd_eur(result: String) -> f64 {
    let opt = roxmltree::ParsingOptions {
        allow_dtd: true,
        ..roxmltree::ParsingOptions::default()
    };
    match roxmltree::Document::parse_with_options(result.as_str(), opt) {
        Ok(doc) => {
            let value = traverse_elements(doc.root_element()).unwrap_or(0f64);
            value
        }
        Err(_) => 0f64,
    }
}

fn traverse_elements(node: Node) -> Option<f64> {
    if let Some(value) = search_relevant_value(node, ("currency", "USD"), "rate") {
        return Some(value);
    }

    if node.has_children() {
        for child in node.children() {
            let value = traverse_elements(child);
            match value {
                None => continue,
                _ => return value,
            }
        }
    }

    None
}

fn search_relevant_value(
    node: Node,
    searched: (&str, &str),
    returned_attribute: &str,
) -> Option<f64> {
    for attribute in node.attributes() {
        if attribute.name() == searched.0 && attribute.value() == searched.1 {
            let value = node.attribute(returned_attribute).unwrap().to_string();
            let value = value.parse::<f64>().unwrap();
            return Some(value);
        }
    }

    None
}
