use scraper::element_ref::Select;


fn main() {
    // download the target HTML document
    let response = reqwest::blocking::get("https://www.bcv.org.ve/");
    // get the HTML content from the request response
    // and print it
    let html_content = response.unwrap().text().unwrap();
    println!("{html_content}");

    let document = scraper::Html::parse_document(&html_content);
    
    let html_price_selector = scraper:: Selector::parse("div.row.recuadrotsmc").unwrap();
    let html_date_selector = scraper::Selector::parse("div.pull-right.dinpro.center").unwrap();

    let html_prices = document.select(&html_price_selector);
    let html_date= document.select(&html_date_selector);
    
    #[derive (Debug)]
    struct Tasa {
        currency: Option<String>,
        price: Option<String>
    }

    let mut prices: Vec<Tasa>= Vec::new();

    fn get_subnode(selector:&str,price:Select<'_,'_>)->Option<String>{
        let mut tasa = Option::Some(String::new());
        for precio in price{
            tasa = precio.select(&scraper::Selector::parse(selector).unwrap())
            .next()
            .map(|price| price.text().collect::<String>());
        }
        return tasa;
    }

    
    for html_price in html_prices{
        let mut local = scraper::Selector::parse("div.col-sm-6.col-xs-6").unwrap();
        let moneda = html_price.select(&local);
        let currency = get_subnode("span",moneda);

        local = scraper::Selector::parse("div.col-sm-6.col-xs-6.centrado").unwrap();
        let precio = html_price.select(&local);
        let price = get_subnode("strong",precio);

        
        let tasa = Tasa {
           currency,
            price
        };
        prices.push(tasa);


    }

    for i in prices {
        println!("{:?}", i);
    }

    /*for date in html_date{
        let  local = scraper::Selector::parse("span").unwrap();
        let fecha = date.select(&local);
        println!("{:?}", fecha);
    }*/
    



}

