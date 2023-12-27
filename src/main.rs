//  use std::io;

// use reqwest::blocking::get;
    // Include dependencies
    use reqwest;
    use tokio;
    use dotenv::dotenv;
    use std::io;
    use serde::{Deserialize, Serialize};
    use druid::{Data, widget::{Label, Button, Flex}, Env, Widget, WindowDesc, AppLauncher};
    
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
#[derive(Clone,Data)]
struct WeatherUi {
    data: i32
}


#[derive(Serialize, Deserialize, Debug)]
struct Location {
    name: String,
    region: String,
    country: String,
    //temp_f: String,
    // feelslike_c: u32,
    // feelslike_f: u32,
    // cloud: u32,
    // wind_mph: u32,
    // humidity: i32,
    // wind_dir: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Current {
    temp_f: f32,
}
#[derive(Serialize, Deserialize, Debug)]
struct WeatherData {
    location: Location,
    current: Current,
}

// API JSON response
// {
//     "location":{"name":"Miami","region":"Florida","country":"United States of America","lat":25.77,"lon":-80.19,"tz_id":"America/New_York","localtime_epoch":1703446600,"localtime":"2023-12-24 14:36"},
//
//     "current":{"last_updated_epoch":1703446200,"last_updated":"2023-12-24 14:30","temp_c":25.0,"temp_f":77.0,"is_day":1,
//         "condition":{"text":"Overcast","icon":"//cdn.weatherapi.com/weather/64x64/day/122.png","code":1009},
//         "wind_mph":17.4,"wind_kph":28.1,"wind_degree":90,"wind_dir":"E","pressure_mb":1018.0,"pressure_in":30.07,"precip_mm":0.22,
//         "precip_in":0.01,"humidity":62,"cloud":100,"feelslike_c":26.5,"feelslike_f":79.7,"vis_km":16.0,"vis_miles":9.0,"uv":5.0,"gust_mph":23.9,"gust_kph":38.5
//     }
// }

fn ui() -> impl Widget<WeatherUi> {
    let label = Label::new(|data: &WeatherUi, _: &Env| format!("Counter: {}", data.data));
    let increment = Button::new("+")
        .on_click(|_ctx, data: &mut WeatherUi, _env| data.data += 1);
    let decrement = Button::new("-")
        .on_click(|_ctx, data: &mut WeatherUi, _env| data.data -= 1);

    Flex::column().with_child(label).with_child(Flex::row().with_child(increment).with_child(decrement))
}

// Tells the compiler to run this async function at runtime, in this case main.
#[tokio::main]
    async fn main() -> Result<(), reqwest::Error> {

        let main_window = WindowDesc::new(ui())
        .title("test Window");
        AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(WeatherUi { data: 0 }).unwrap();

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

        let weather_data: WeatherData = serde_json::from_str(&body).expect("error");

        println!("Body:\n{}", body);

        println!("{:#?}", weather_data);
        println!("{}", weather_data.current.temp_f);
        //println!("Country: {}", weather_data.location.country);
        //println!("F: {}",weather_data.location.feelslike_f);
        //println!("humidity: {}", weather_data.location.humidity);
        //println!("wind_mph: {}", weather_data.location.wind_mph);
        // this is successful return the result
        Ok(())
    }

  
        



