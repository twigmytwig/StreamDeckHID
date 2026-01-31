use serde::Deserialize;

// TODO: Make location configurable via action params and persist to disk
pub const WEATHER_API: &str = "https://wttr.in/28376?format=j1";

#[derive(Deserialize)]
pub struct WttrResponse {
    current_condition: Vec<CurrentCondition>,
}

#[derive(Deserialize)]
pub struct CurrentCondition {
    temp_F: String,
    weatherCode: String,
}

pub fn get_weather() -> Option<String> {
    let response = reqwest::blocking::get(WEATHER_API).ok()?;

    if response.status().is_success() {
        let weather: WttrResponse = response.json().ok()?;
        let condition = weather.current_condition.first()?;

        let temp = &condition.temp_F;
        let emoji = weather_code_to_emoji(&condition.weatherCode);

        Some(format!("{}°F {}", temp, emoji))
    } else {
        None
    }
}

fn weather_code_to_emoji(code: &str) -> &'static str {
    match code {
        // Clear/Sunny
        "113" => "☀️",
        // Partly cloudy
        "116" => "⛅",
        // Cloudy
        "119" => "☁️",
        // Overcast
        "122" => "☁️",
        // Mist/Fog
        "143" | "248" | "260" => "🌫️",
        // Rain variants
        "176" | "263" | "266" | "293" | "296" | "353" => "🌦️",
        "299" | "302" | "305" | "308" | "356" | "359" => "🌧️",
        // Freezing rain/drizzle
        "185" | "281" | "284" | "311" | "314" => "🌧️",
        // Snow
        "179" | "323" | "326" => "🌨️",
        "227" | "230" | "329" | "332" | "335" | "338" | "368" | "371" => "❄️",
        // Sleet/Ice
        "182" | "317" | "320" | "362" | "365" | "350" | "374" | "377" => "🌨️",
        // Thunderstorm
        "200" | "386" | "389" | "392" | "395" => "⛈️",
        // Default
        _ => "🌡️",
    }
}
