use getrandom::register_custom_getrandom;

register_custom_getrandom!(custom_getrandom);

fn custom_getrandom(buf: &mut [u8]) -> Result<(), getrandom::Error> {
    // TODO get some randomness
    return Ok(());
}

mod environment;
mod profile;
