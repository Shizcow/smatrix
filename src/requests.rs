/*
 * Requests.rs
 *
 * Handles requests to get stock prices 
 * 
 */
use curl::easy::Easy;
use std::str;
use scraper::{Html, Selector};


fn build_quote_url(tickers: &Vec<String>) -> String {
    "https://query1.finance.yahoo.com/v7/finance/quote?symbols=".to_string() + &tickers.join(",")
}

pub struct Report { // TODO: change these into strings, impl len() and such
    pub ticker: String,
    pub price: f32,
    pub change: f32,
    pub change_percent: f32
}

#[allow(dead_code)]
pub fn request_tickers(tickers: &Vec<String>) -> Vec<Report> {
    let mut response = Vec::new();
    {
	let mut easy = Easy::new();
	easy.url(&build_quote_url(tickers)).unwrap();
	
	let mut transfer = easy.transfer();
	transfer.write_function(|data| {
            response.extend_from_slice(data);
            Ok(data.len())
	}).unwrap();
	transfer.perform().unwrap();
    }

    let parsed = json::parse(str::from_utf8(&response).unwrap()).unwrap();
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
#[allow(dead_code)]
pub fn get_sp500_tickers() -> Vec<String>  {
    let mut response = Vec::new();
    {
	let mut easy = Easy::new();
	easy.url("https://en.wikipedia.org/wiki/List_of_S%26P_500_companies").unwrap();
	
	let mut transfer = easy.transfer();
	transfer.write_function(|data| {
            response.extend_from_slice(data);
            Ok(data.len())
	}).unwrap();
	transfer.perform().unwrap();
    }

    let mut tickers = Vec::new();
    for tr in Html::parse_fragment(str::from_utf8(&response).unwrap()) // document
	.select(&Selector::parse("tbody").unwrap()).next().unwrap()    // table body (includes head, thanks HTML5)
	.select(&Selector::parse("tr").unwrap()) {                     // each row in the first table
	    if let Some(td) = tr.select(&Selector::parse("td").unwrap()).next() { // ignore thead
		tickers.push(td.text().collect::<Vec<_>>()[0].to_string());
	    }
	}
    
    tickers    
}
