# Weather.gov API consumption in Rust #

### Description ###

This project aims to consume the [Weather.gov API](https://www.weather.gov/documentation/services-web-api) in a way that is simple to parse for directly analysis or piping into more complicated systems down the line.

#### Note ####
This is still a work in progress, I recently (4/18/25MDY) ripped out most of the image generation code and make it more text based and focus on what I can tangibly use this for.  More things will be added back as I get back into the swing of things with Rust.

### Requirements ###

[Rust](https://www.rust-lang.org/tools/install)

[reqwest](https://docs.rs/reqwest/latest/reqwest/)
[serde_json](https://docs.rs/serde_json/latest/serde_json/)
[bmp](https://docs.rs/bmp/latest/bmp/)

These cargos can also be found in the Cargo.toml file with versions and features

### Usage ###

To begin you'll have to make a config.rs file to point to the URL you want.  This is explained more in the next section.

Once you have config.rs tailored for the "forecast" not hourly, compile and run.  You'll get a terminal output of the next 7 days and evenings.

### config.rs ###

Make a "config" file I made a .rs file that has public functions that returned private information like keys and the such.
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

### License ###

Licensed under 
Apache License Version 2.0 https://apache.org/licenses/LICENSE-2.0
