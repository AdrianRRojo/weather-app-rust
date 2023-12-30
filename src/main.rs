// Include dependencies
use reqwest;
use tokio;
use dotenv::dotenv;
//use std::io;
 use serde::{Deserialize, Serialize};

//Druid is how we handle the GUI
use druid::widget::prelude::*;
use druid::widget::{Flex, Label, TextBox, Button};
use druid::{AppLauncher, Data, Lens, UnitPoint, WidgetExt, WindowDesc};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 300.0;
const BUTTON_BOX_WIDTH: f64 = 150.0;

#[derive(Clone,Data,Lens)]
struct WeatherUi {
    // GUI
    // data: i32,
    city: String,
    temp_f: String,
    temp_c: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct Location {
    // API - WeatherData.location
    name: String,
    region: String,
    country: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Current {
    // API - WeatherData.current
    temp_f: f32,
    temp_c: f32,
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

//         "wind_mph":17.4,
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
            temp_f: "".into(),
            temp_c: "".into(),
        };

        // Launch the app
        AppLauncher::with_window(main_window)
            .log_to_console()
            .launch(initial_state)
            .expect("Oh no! Failed to launch");

    }

  
    fn build_root_widget() -> impl Widget<WeatherUi> {
        // a label that will determine its text based on the current app data.
        let label = Label::new(|data: &WeatherUi, _env: &Env| {
            if data.city.is_empty() {
                "Please enter a city".to_string()
            } else {
                // format!("{:#?}",fetch_api_data(data.city.clone()))
                // data.city.clone();
                data.temp_f.clone()
            }
        })
        .with_text_size(32.0);

        let temp_c_label = Label::new(|data: &WeatherUi, _env: &Env| {
            if data.city.is_empty() {
                " ".to_string()
            } else {
                // format!("{:#?}",fetch_api_data(data.city.clone()))
                // data.city.clone();
                data.temp_c.clone()
            }
        })
        .with_text_size(32.0);
        
        let textbox = TextBox::new()
            .with_placeholder("Where should we go?")
            .with_text_size(20.0)
            .fix_width(TEXT_BOX_WIDTH)
            .lens(WeatherUi::city);

        let button = Button::new("Search")
            .on_click(|_ctx, data: &mut WeatherUi, _env| {
                match fetch_api_data(data.city.clone()){
                    Ok((temp_c, temp_f)) => {
                        // data.city = fetched.to_string();
                        data.temp_f = temp_f.to_string();
                        data.temp_c = temp_c.to_string();
                    }
                    Err(e) =>{
                        eprintln!("Error fetching data: {:?}", e);
                    }
                }
            })
            .fix_width(BUTTON_BOX_WIDTH);
        
    // arrange the two widgets vertically, with some padding
        Flex::column()
            // .with_child(button)
            .with_child(textbox)
            .with_spacer(VERTICAL_WIDGET_SPACING)
            .with_child(button)
            .with_spacer(VERTICAL_WIDGET_SPACING)
            .with_child(label)
            .with_child(temp_c_label)
            .align_vertical(UnitPoint::CENTER)
    }    

    #[tokio::main]
    async fn fetch_api_data(city: String) -> Result<(f32,f32), reqwest::Error> {
        dotenv().ok();
        // set our api key to a usuable variable
        let api_key = std::env::var("API_KEY").expect("API key is not set");

        let url = format!("http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no", api_key, city);
        
        // Send a GET request to the url
        let res = reqwest::get(url).await?;
    
        // get the status code, should be 200
        println!("Status: {}", res.status());
    
        //     api response
             let body = res.text().await?;
    
             let weather_data: WeatherData = serde_json::from_str(&body).expect("Error parsing JSON");
    
        //     println!("Body:\n{}", body);
    
        //     println!("{:#?}", weather_data);
        //     println!("{}", weather_data.current.temp_f);
        let temp_f = weather_data.current.temp_f;
        let temp_c = weather_data.current.temp_c;
        //let region = weath_data.location.region;
            // this is successful return the result
            Ok((temp_f, temp_c))
    }



