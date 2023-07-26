use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde_json::Value;
use bmp::{Image, Pixel};
use bmp::px as px;
mod config;
mod bmp_numbers;

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

Start: {} {} -> End: {} {}
{}
Rain %: {}
Humidity: {}
Temperature: {} {}

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
            draw_detailed_graph(resp.clone());
            //gen_hum_and_rain_graph(resp.clone());
            //print_hourly(resp.clone());
        },
    }
}

fn draw_detailed_graph(dataset: Value)
{
    let mut img = Image::new(644, 120); // Width, Height

    // Draws 0-100 height reference
    let mut color = true;
    for y in 10..111
    {
        if color
        {
            if y % 10 == 0
            {
                img.set_pixel(9, y, px!(255,0,0));
            }
            else
            {
                img.set_pixel(9, y, px!(255,255,255));
            }

            color = false;
        }
        else
        {
            if y % 10 == 0
            {
                img.set_pixel(9, y, px!(255,0,0));
            }

            color = true;
        }
    }

    // Draws 0-156 hour reference
    color = true;
    for x in 0..156
    {
        if color
        {
            // 4x + 10
            for offset in 0..4
            {
                img.set_pixel(4 * x + 10 + offset, 112, px!(255,255,255));
            }
            color = false;
        }
        else
        {
            color = true;
        }
    }

    // TEMPLATE DRAWING ENDS

    for x in 0..156 {
        // Draws humidity line
        let y = 110 - serde_json::from_value::<u32>(dataset["properties"]["periods"][x]["relativeHumidity"]["value"].clone()).unwrap();
        for offset in 0..4
        {
            img.set_pixel((4 * x + 10 + offset).try_into().unwrap(), y, px!(255, 125, 0));
        }

        // Draws rain line
        let y = 110 - serde_json::from_value::<u32>(dataset["properties"]["periods"][x]["probabilityOfPrecipitation"]["value"].clone()).unwrap();
        for offset in 0..4
        {
            img.set_pixel((4 * x + 10 + offset).try_into().unwrap(), y, px!(0, 125, 255));
        }
    }

    // Draw hour markers every 4 hours
    for x in 0..39
    {
        // 16x + 9
        img = bmp_numbers::draw_number(dataset["properties"]["periods"][x*4]["startTime"].as_str().unwrap()[11..12].parse::<u32>().unwrap(), img.clone(), (16 * x + 10).try_into().unwrap(), 113);
        img = bmp_numbers::draw_number(dataset["properties"]["periods"][x*4]["startTime"].as_str().unwrap()[12..13].parse::<u32>().unwrap(), img.clone(), (16 * x + 14).try_into().unwrap(), 113);
    }

    let _ = img.save("template_test.bmp");
}

fn gen_hum_and_rain_graph(dataset: Value) {
    let mut img = Image::new(176, 120); // Width, Height

    for x in 0..156 {
        let y = 110 - serde_json::from_value::<u32>(dataset["properties"]["periods"][x]["relativeHumidity"]["value"].clone()).unwrap();
        img.set_pixel((x + 10).try_into().unwrap(), y, px!(0, 125, 255));
        if &dataset["properties"]["periods"][x]["startTime"].as_str().unwrap()[11..13] == "00"
        {
            img.set_pixel((x + 10).try_into().unwrap(), 0, px!(255, 255, 255));
        }

        let y = 110 - serde_json::from_value::<u32>(dataset["properties"]["periods"][x]["probabilityOfPrecipitation"]["value"].clone()).unwrap();
        img.set_pixel((x + 10).try_into().unwrap(), y, px!(255, 125, 0));
        if &dataset["properties"]["periods"][x]["startTime"].as_str().unwrap()[11..13] == "00"
        {
            img.set_pixel((x + 10).try_into().unwrap(), 0, px!(255, 255, 255));
        }
    }
    let _ = img.save("hum_and_rain_graph.bmp");
}

fn print_hourly(resp: Value) {
    for item in 0..156
    {
        // This is a disgusting one line monstrosity, I know.  But it worked for testing, leave me alone.
        println!("Start: {} {} -> End: {} {}\n{}\nRain %: {}\nHumidity: {}\nTemperature: {} {}", &resp["properties"]["periods"][item]["startTime"].as_str().unwrap()[..10], &resp["properties"]["periods"][item]["startTime"].as_str().unwrap()[11..19], &resp["properties"]["periods"][item]["endTime"].as_str().unwrap()[..10], &resp["properties"]["periods"][item]["endTime"].as_str().unwrap()[11..19], resp["properties"]["periods"][item]["shortForecast"], resp["properties"]["periods"][item]["probabilityOfPrecipitation"]["value"], resp["properties"]["periods"][item]["relativeHumidity"]["value"], resp["properties"]["periods"][item]["temperature"], resp["properties"]["periods"][item]["temperatureUnit"]);
    }
}