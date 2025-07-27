use std::vec;

use clap::{Args, Parser};
use scraper::element_ref::Select;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    
    #[arg(long)]
    eur: bool,

    #[arg(long)]
    cny: bool,

    #[arg(long)]
    tl: bool,

    #[arg(long)]
    rub: bool,

    #[arg(long)]
    usd: bool,
}

fn main() {
    let mut currencies: Vec<String> = vec::Vec::new();
    let cli = Cli::parse();

    if cli.usd {
        currencies.push("USD".to_string());
    }
    if cli.cny {
        currencies.push("CNY".to_string());
    }
    if cli.eur {
        currencies.push("EUR".to_string());
    }
    if cli.rub {
        currencies.push("RUB".to_string());
    }
    if cli.tl {
        currencies.push("TRY".to_string());
    }
    if !cli.usd && !cli.cny && !cli.eur && !cli.rub && !cli.tl {
        currencies.push("USD".to_string());
        currencies.push("CNY".to_string());
        currencies.push("EUR".to_string());
        currencies.push("RUB".to_string());
        currencies.push("TRY".to_string());
    }

    let client = reqwest::blocking::Client::builder()
    .danger_accept_invalid_certs(true)
    .build()
    .unwrap();

    let response = client.get("https://www.bcv.org.ve/").send().unwrap();

    let html_content = response.text().unwrap();
    let document = scraper::Html::parse_document(&html_content);

    let html_price_selector = scraper::Selector::parse("div.row.recuadrotsmc").unwrap();
    let html_date_selector = scraper::Selector::parse("div.pull-right.dinpro.center").unwrap();

    let html_prices = document.select(&html_price_selector);
    let html_date = document.select(&html_date_selector);

    #[derive(Debug)]
    struct Tasa {
        currency: Option<String>,
        price: Option<String>,
    }

    let mut prices: Vec<Tasa> = Vec::new();

    fn get_subnode(selector: &str, price: Select<'_, '_>) -> Option<String> {
        price
            .flat_map(|precio| {
                precio
                    .select(&scraper::Selector::parse(selector).unwrap())
                    .next()
                    .map(|p| p.text().collect::<String>())
            })
            .next()
    }

    for html_price in html_prices {
        let mut local = scraper::Selector::parse("div.col-sm-6.col-xs-6.centrado").unwrap();
        let precio = html_price.select(&local);
        let price = get_subnode("strong", precio);

        local = scraper::Selector::parse("div.col-sm-6.col-xs-6").unwrap();
        let moneda = html_price.select(&local);
        let currency = get_subnode("span", moneda);

        let tasa = Tasa { currency, price };
        prices.push(tasa);
    }

    for i in prices {
        if let Some(currency) = &i.currency {
            let clean_currency = currency.trim();
            if currencies.iter().any(|c| c == clean_currency) {
                if let Some(price) = &i.price {
                    println!("{}        {}", clean_currency, price.replace(',', "."));
                }
            }
        }
    }

    for date in html_date {
        let local = scraper::Selector::parse("span").unwrap();
        let mut fecha = date.select(&local);
        println!("{}", fecha.next().map(|m| m.inner_html()).unwrap());
    }
}
