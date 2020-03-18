/*
 * Requests.rs
 *
 * Handles requests to get stock prices 
 * 
 */
use curl::easy::Easy;
use std::str;


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
    for i in 0..results.len() {
	println!("Ticker: {:}", results[i]["symbol"]);
	println!("Price: {:}", results[i]["regularMarketPrice"]);
	println!("Change: {:}", results[i]["regularMarketChange"]);
	println!("Change Percet: {:}%", results[i]["regularMarketChangePercent"]);
    }
}
