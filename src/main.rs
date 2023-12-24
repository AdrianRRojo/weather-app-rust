//  use std::io;

// use reqwest::blocking::get;
    // Include dependencies
    use reqwest;
    use tokio;
    use dotenv::dotenv;
    use std::io;
    
//   fn main() {
//     println!("enter your city");
//     let mut city = String::new();

//     io::stdin()
//     .read_line(&mut city)
//     .expect("Enter a word");
    
//     println!("Result: {city}");

    
//     let base_url: &str = "http://api.weatherapi.com/v1/current.json";

//     // let resq: Result<Response, Error> = get(base_url)?;

//     // let resp: serde_json::Value = get(url)?.json()?;
//     // println!("{:?}", resq);

#[tokio::main]
    async fn main() -> Result<(), reqwest::Error> {
        // load .env variables
        dotenv().ok();
        // set our api key to a usuable variable
        let api_key = std::env::var("API_KEY").expect("API Key is not set");
        //println!("{}",api_key);

        println!("Enter your city");
        let mut city = String::new();
        
        io::stdin()
        .read_line(&mut city)
        .expect("Enter a word");
            
        //     println!("Result: {city}");
    // The API endpoint
        let url = format!("http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no", api_key, city);
        
    // Send a GET request to the url
       let res = reqwest::get(url).await?;

        // get the status code, should be 200
        println!("Status: {}", res.status());
        // println!("Headers:\n{:#?}", res.headers());

        // api response
        let body = res.text().await?;
        println!("Body:\n{}", body);

        // this is successful return the result
        Ok(())
    }

  
        



