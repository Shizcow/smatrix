/*
 * Requests.rs
 *
 * Handles requests to get stock prices 
 * 
 */
use curl::easy::Easy;
use std::str;

#[allow(dead_code)]
struct Report {
    ticker: String,
    price: f32,
    change: f32,
    change_percent: f32
}

fn main(){

    let mut dst = Vec::new();
    {
	let mut easy = Easy::new();
	easy.url("https://query1.finance.yahoo.com/v7/finance/quote?symbols=TSLA,AMD").unwrap();
	
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
	let ticker = results[i]["symbol"].as_str().unwrap();
	let price = results[i]["regularMarketPrice"].as_f32().unwrap();
	let change = results[i]["regularMarketChange"].as_f32().unwrap();
	let change_percent = results[i]["regularMarketChangePercent"].as_f32().unwrap();
	reports.push(Report {ticker: ticker.to_string(),
			     price: price,
			     change: change,
			     change_percent: change_percent});
    }

    for report in reports {
	println!("Ticker: {:?}", report.ticker);
	println!("Price: {:?}", report.price);
	println!("Change: {:?}", report.change);
	println!("Change Percet: {:?}%", report.change_percent);
    }
}
