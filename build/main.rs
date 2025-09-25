use forecast_full::gen_forecast_full;
use general::gen_general;
use site::gen_site;
use site_list::gen_site_list;
use utils::rustfmt_pretty_print;
use weather::gen_weather;
use xsd_parser::Error;

mod forecast_full;
mod general;
mod site;
mod site_list;
mod utils;
mod weather;

fn main() -> Result<(), Error> {
    gen_site_list()?;
    gen_general()?;
    gen_weather()?;
    gen_forecast_full()?;
    gen_site()?;
    Ok(())
}
