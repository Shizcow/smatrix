/*
 * Requests.rs
 *
 * Handles requests to get stock prices 
 * 
 */
use curl::easy::Easy;
use std::str;
use scraper::{Html, Selector};


fn build_url(tickers: &Vec<String>) -> String {
    "https://query1.finance.yahoo.com/v7/finance/quote?symbols=".to_string() + &tickers.join(",")
}

#[allow(dead_code)]
struct Report {
    ticker: String,
    price: f32,
    change: f32,
    change_percent: f32
}

fn request_tickers(tickers: &Vec<String>) -> Vec<Report> {
    let mut dst = Vec::new();
    {
	let mut easy = Easy::new();
	easy.url(&build_url(tickers)).unwrap();
	
	let mut transfer = easy.transfer();
	transfer.write_function(|data| {
            dst.extend_from_slice(data);
            Ok(data.len())
	}).unwrap();
	transfer.perform().unwrap();
    }

    let pre_parsed = str::from_utf8(&dst).unwrap();
    let parsed = json::parse(pre_parsed).unwrap();

    let results = &parsed["quoteResponse"]["result"];
    let mut reports = vec![];
    for i in 0..results.len() {
	if let (Some(ticker), Some(price), Some(change), Some(change_percent))
	    = (results[i]["symbol"].as_str(),
	       results[i]["regularMarketPrice"].as_f32(),
	       results[i]["regularMarketChange"].as_f32(),
	       results[i]["regularMarketChangePercent"].as_f32()) { // some tickers don't have useful information
		
		reports.push(Report {ticker: ticker.to_string(),
				 price: price,
				 change: change,
				 change_percent: change_percent});
	}
    }

    reports
}

// NOTE: there's more than 500 tickers
fn get_sp500_tickers() -> Vec<String>  {
    let mut dst = Vec::new();
    {
	let mut easy = Easy::new();
	easy.url("https://en.wikipedia.org/wiki/List_of_S%26P_500_companies").unwrap();
	
	let mut transfer = easy.transfer();
	transfer.write_function(|data| {
            dst.extend_from_slice(data);
            Ok(data.len())
	}).unwrap();
	transfer.perform().unwrap();
    }

    let document = Html::parse_fragment(str::from_utf8(&dst).unwrap());

    // tbody here includes thead -- it's a browser thing
    let tbody = document.select(&Selector::parse("tbody").unwrap()).next().unwrap();

    let mut tickers = Vec::new();
    for tr in tbody.select(&Selector::parse("tr").unwrap()) {
	if let Some(td) = tr.select(&Selector::parse("td").unwrap()).next() { // ignore thead
	    tickers.push(td.text().collect::<Vec<_>>()[0].to_string());
	}
    }
    
    tickers    
}

fn main(){
    
    let reports = request_tickers(&get_sp500_tickers());

    for report in reports {
	println!("Ticker: {:?}", report.ticker);
	println!("Price: {:?}", report.price);
	println!("Change: {:?}", report.change);
	println!("Change Percet: {:?}%", report.change_percent);
    }
    
}
