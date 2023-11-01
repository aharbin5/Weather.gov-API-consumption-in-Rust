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

To begin you'll have to make a config.rs file to point to the URL you want.  This is explained more in the next section.

Once you have config.rs made, compile and run main.rs and it'll output a "template_test.bmp" image in the directory the executable is in.

### config.rs ###

As I said, I'm learning Rust through this project so, if there's a better way to do this let me know, but to make a "config" file I made a .rs file that has public functions that returned private information like keys and the such.
If you want to use this and replicate the config.rs file here's a copy of my config.rs with the information removed,

    pub fn return_user_agent() -> &'static str {
        "your_email_goes_here"
    }

    pub fn return_target_url() -> &'static str {
        "your_target_api_url_goes_here"
    }

Weather.gov's API uses a user_agent field to differentiate who you are so you're supposed to put your website and contact email address in said field so if there are outages or you're sending too many requests they can reach out.  This information is also on their API documentation website which I linked in the beginning.

An example of the target URL would be,
`https://api.weather.gov/gridpoints/LWX/97,72/forecast/hourly`
LWX 97, 72 is Washington DC's API code

### main ###

Main initalizes the API client, sends and recieves the HTTP get, and passes it off to the other functions for processing.  If you create your own function and want to run instead of the ones I've made, you can replace the `gen_hum_and_rain_graph(resp.clone());` line.

### draw_detailed_graph ###

This function takes the input dataset and, given it's in the correct format, will create a 644px by 120px graph with humidity% (orange line) and rain% (blue line) data displayed.

First, it draws the percentage reference on the Y axis in alternating black and white dots
Then the hour references on the X axis is alternating black and white lines
With hour numbers marked every 4 hours

Then it finally draws the humidity and rain lines

### bmp_numbers ###

So this really should be it's own package but this file is meant to expand the [bmp](https://docs.rs/bmp/latest/bmp/) package to be able to draw pixel numbers and eventually letters if I get around to adding them.

Eventually I will make a draw_line function to make drawing the characters less of a hassle.

### License ###

Licensed under 
Apache License Version 2.0 https://apache.org/licenses/LICENSE-2.0
