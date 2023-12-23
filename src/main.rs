// use std::io;

use reqwest::blocking::get;

fn main() {
    // println!("enter your city");
    // let mut city = String::new();

    // io::stdin()
    // .read_line(&mut city)
    // .expect("Enter a word");
    
    // println!("Result: {city}");

    
    let base_url: &str = "http://api.weatherapi.com/v1/current.json";

    let resq: Result<Response, Error> = get(base_url)?;

    // let resp: serde_json::Value = get(url)?.json()?;
    println!("{:?}", resq);
    
        
}
