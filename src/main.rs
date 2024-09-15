use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde_json::Value;
mod config;

/*

Reference links

https://docs.rs/reqwest/latest/reqwest/blocking/struct.RequestBuilder.html#method.headers
https://docs.rs/reqwest/latest/reqwest/blocking/struct.Response.html
https://www.weather.gov/documentation/services-web-api
https://weather-gov.github.io/api/general-faqs
https://docs.rs/serde_json/latest/serde_json/index.html

https://docs.rs/serde_json/latest/serde_json/fn.from_value.html
https://docs.rs/bmp/latest/bmp/
https://docs.rs/bmp/latest/bmp/struct.Image.html

Print_hourly format

resp["properties"]["periods"][item]["startTime"]
resp["properties"]["periods"][item]["endTime"]
resp["properties"]["periods"][item]["shortForecast"]
resp["properties"]["periods"][item]["probabilityofPrecipitation"][value]
resp["properties"]["periods"][item]["relativeHumidity"][value]
resp["properties"]["periods"][item]["Temperature"]
resp["properties"]["periods"][item]["temperatureUnit"]
*/

fn main() {
    let client = Client::new();
    let mut my_header_map = HeaderMap::new();
    my_header_map.insert(USER_AGENT, HeaderValue::from_static(config::return_user_agent()));
    let resp = client.get(config::return_target_url())
                     .headers(my_header_map)
                     .send();

    match resp
    {
        Err(e) => println!("{}", e),
        Ok(t) => {
            let resp: Value = serde_json::from_str(&t.text().unwrap()).unwrap();
            
            /*
                Period
                Detailed forcast
                If rain not null, %
            */

            for period in 0..14 {
                println!("{}", serde_json::from_value::<String>(resp["properties"]["periods"][period]["name"].clone()).unwrap());
                println!("{}", serde_json::from_value::<String>(resp["properties"]["periods"][period]["detailedForecast"].clone()).unwrap());
                println!("");
            }
        },
    }
}