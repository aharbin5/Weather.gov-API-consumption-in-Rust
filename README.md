# Weather.gov API consumption in Rust #

### Description ###

This project is partially becuase I need a quick way to check the weather in one specific area for the next few days for humidity and rain %'s and partially because I wanted to get better at using Rust.  This project was primarily used to consume the [Weather.gov API](https://www.weather.gov/documentation/services-web-api)

### Requirements ###

[Rust](https://www.rust-lang.org/tools/install)

[reqwest](https://docs.rs/reqwest/latest/reqwest/)
[serde_json](https://docs.rs/serde_json/latest/serde_json/)
[bmp](https://docs.rs/bmp/latest/bmp/)

These cargos can also be found in the Cargo.toml file with versions and features

### Usage ###

To begin you'll have to fill in the config.rs file as I have mine .gitignore'd

Once you have that done, you can use either the "gen_hum_and_rain_graph" function that will create a .bmp graph of the next 156 hours humidity and precipitation chances.
I'll add a picture of this working later.

Or, if you want a command line output of the start and end times, short forecast, rain and humidity percent and temperature the "print_hourly" function prints all 156 hours.
Which will look something like this,

    Start: 2023-04-23 05:00:00 -> End: 2023-04-23 06:00:00
    "Partly Cloudy"
    Rain %: 12
    Humidity: 81
    Temperature: 73 "F"

### config.rs ###

As I said, I'm learning Rust through this project so, if there' a better way to do this let me know, but to make a "config" file I made a .rs file that has public functions that returned private information like keys and the such.
If you want to use this and replicate the config.rs file here's a copy of my config.rs with the information removed,

    pub fn return_user_agent() -> &'static str {
        "your_email_or_key_goes_here"
    }

    pub fn return_target_url() -> &'static str {
        "your_target_api_url_goes_here"
    }

Weather.gov's API uses a user_agent field to differentiate who you are so you're supposed to put your website and contact email address in said field so if there are outages or you're sending too many requests they can reach out.  This information is also on their API documentation website which I linked in the beginning.

### main ###

Main initalizes the API client, sends and recieves the HTTP get, and passes it off to the other functions for processing.  If you create your own function and want to run instead of the ones I've made, you can reaplce the `gen_hum_and_rain_graph(resp.clone());` line.

### gen_hum_and_rain_graph ###

For ease of use I have the data being turned into a graph of humidity% and rain% in this function.  The bmp library gives you really easy control of each bit of a bitmap so I have each hourly% mapped to a place on the graph.
This function writes a file out called "hum_and_rain_graph.bmp" to the top level of the project.

### print_hourly ###

This is a debug function I made that formats weather.gov's hourly forecast and prints out all 156 sections to verify I was recieving all the information.  It's a really disgusting one line of a formatted println! macro.

### License ###

Licensed under 
Apache License Version 2.0 https://apache.org/licenses/LICENSE-2.0
