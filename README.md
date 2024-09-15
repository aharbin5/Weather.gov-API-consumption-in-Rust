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

Main initalizes the API client, sends and recieves the HTTP get then prints a basic output to the CLI interface.

This is not the fancy version of this program that makes a cool image.  This is the version that I needed to work.

### License ###

Licensed under 
Apache License Version 2.0 https://apache.org/licenses/LICENSE-2.0
