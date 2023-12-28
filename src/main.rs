// Include dependencies
use reqwest;
use tokio;
use dotenv::dotenv;
//use std::io;
 use serde::{Deserialize, Serialize};

//Druid is how we handle the GUI
use druid::widget::prelude::*;
use druid::widget::{Flex, Label, TextBox};
use druid::{AppLauncher, Data, Lens, UnitPoint, WidgetExt, WindowDesc};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 300.0;

#[derive(Clone,Data,Lens)]
struct WeatherUi {
    // GUI
    // data: i32,
    city: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct Location {
    // API - WeatherData.Location
    name: String,
    region: String,
    country: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Current {
    // API - WeatherData.Current
    temp_f: f32,
}
#[derive(Serialize, Deserialize, Debug)]
struct WeatherData {
    location: Location,
    current: Current,
}

// API JSON response
// {
//     "location":{
            // "name":"Miami",
            // "region":"Florida",
            // "country":"United States of America",
            // "lat":25.77,
            // "lon":-80.19,
            // "tz_id":"America/New_York",
            // "localtime_epoch":1703446600,
            // "localtime":"2023-12-24 14:36"},

//     "current":{
//         "last_updated_epoch":1703446200,
//         "last_updated":"2023-12-24 14:30",
//         "temp_c":25.0,
//         "temp_f":77.0,
//         "is_day":1,
//         "condition":{"text":"Overcast",
//         "icon":"//cdn.weatherapi.com/weather/64x64/day/122.png",
//         "code":1009},

//          "wind_mph":17.4,
//         "wind_kph":28.1,
//         "wind_degree":90,
//         "wind_dir":"E",
//         "pressure_mb":1018.0,
//         "pressure_in":30.07,
//         "precip_mm":0.22,
//         "precip_in":0.01,
//         "humidity":62,
//         "cloud":100,
//         "feelslike_c":26.5,
//         "feelslike_f":79.7,
//         "vis_km":16.0,
//         "vis_miles":9.0,
//         "uv":5.0,
//         "gust_mph":23.9,
//         "gust_kph":38.5
//     }
// }

// fn ui() -> impl Widget<WeatherUi> {
//     let label = Label::new(|data: &WeatherUi, _: &Env| format!("Counter: {}", data.data));
//     let increment = Button::new("+")
//         .on_click(|_ctx, data: &mut WeatherUi, _env| data.data += 1);
//     let decrement = Button::new("-")
//         .on_click(|_ctx, data: &mut WeatherUi, _env| data.data -= 1);

//     Flex::column().with_child(label).with_child(Flex::row().with_child(increment).with_child(decrement))

//     let txt_Box = TextBox::new()
//         .with_placeholder("Who are we greeting?")
//         .with_text_size(18.0)
//         // .fix_width(TEXT_BOX_WIDTH)
//         .lens(WeatherUi::name);
// }

// Tells the compiler to run this async function at runtime, in this case main.

    fn main() {



        // load .env variables
        // dotenv().ok();
        // // set our api key to a usuable variable
        // let api_key = std::env::var("API_KEY").expect("API key is not set");
        // print!("{}",api_key);

        // Describe the window we are creating
        let main_window = WindowDesc::new(build_root_widget())
            .title("Weather app in Rust")
            .window_size((400.0,750.0));

        // initial app state
        let initial_state: WeatherUi = WeatherUi{
            city: "".into(),
        };

        // Launch the app
        AppLauncher::with_window(main_window)
            .log_to_console()
            .launch(initial_state)
            .expect("Oh no! Failed to launch");





    // // The API endpoint
    //     let url = format!("http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no", api_key, city);
        
    // // Send a GET request to the url
    //    let res = reqwest::get(url).await?;

    //     // get the status code, should be 200
    //     println!("Status: {}", res.status());
    //     // println!("Headers:\n{:#?}", res.headers());

    //     // api response
    //     let body = res.text().await?;

    //     let weather_data: WeatherData = serde_json::from_str(&body).expect("error");

    //     println!("Body:\n{}", body);

    //     println!("{:#?}", weather_data);
    //     println!("{}", weather_data.current.temp_f);

        // this is successful return the result
        // Ok(())
    }

  
    fn build_root_widget() -> impl Widget<WeatherUi> {
        // a label that will determine its text based on the current app data.
        let label = Label::new(|data: &WeatherUi, _env: &Env| {
            if data.city.is_empty() {
                "Please enter a city".to_string()
            } else {
                // format!("{}!", data.city)
                // let user_city = data.city;
                format!("{}",return_api_data(data.city.clone()))
            }
        })
        .with_text_size(32.0);
        
        let textbox = TextBox::new()
            .with_placeholder("Where should we go?")
            .with_text_size(20.0)
            .fix_width(TEXT_BOX_WIDTH)
            .lens(WeatherUi::city);

    // arrange the two widgets vertically, with some padding
        Flex::column()
            .with_child(label)
            .with_spacer(VERTICAL_WIDGET_SPACING)
            .with_child(textbox)
            .align_vertical(UnitPoint::CENTER)
    }    

    fn return_api_data(city: String) -> String{
        return city
    }
    #[tokio::main]
    async fn fetch_api_data() -> Result<String, reqwest::Error> {
        //Testing func connection
        //let x = String::from("connected");
        //return x
        dotenv().ok();
        // set our api key to a usuable variable
        let api_key = std::env::var("API_KEY").expect("API key is not set");
        print!("{}",api_key);
        let city = "Miami";
        let url = format!("http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no", api_key, city);
        
        // // Send a GET request to the url
        let res = reqwest::get(url).await?;
    
        // get the status code, should be 200
        println!("Status: {}", res.status());
        //     // println!("Headers:\n{:#?}", res.headers());
    
        //     api response
             let body = res.text().await?;
    
             let weather_data: WeatherData = serde_json::from_str(&body).expect("error");
    
        //     println!("Body:\n{}", body);
    
        //     println!("{:#?}", weather_data);
        //     println!("{}", weather_data.current.temp_f);
        let region = weather_data.location.region;
            // this is successful return the result
           Ok(region)
            
            
    }


