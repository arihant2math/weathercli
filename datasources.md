# Data Sources
## Comparison
| Data Source       | Metric | Regional Availability | Min/Max | Cloud Data | Conditions Sentence | Special Conditions | Forecast Sentence | API Key Required | AQI | Forecast | Time (seconds) | Website                     |
|-------------------|--------|-----------------------|---------|------------|---------------------|--------------------|-------------------|------------------|-----|----------|----------------|-----------------------------|
| NWS               | Yes    | US only               | Yes     | Yes        | Full                | No†                | No†               | No               | No† | No       | 0.5            | https://www.weather.gov/    |
| TheWeatherChannel | Yes    | Worldwide             | Yes     | No†        | No                  | No                 | No                | No               | Yes | No       | ?              | https://weather.com         |
| Meteo             | Yes    | Worldwide             | Yes     | Yes        | Full                | No                 | Full              | No               | Yes | Yes      | 0.9            | https://open-meteo.com      |
| OpenWeatherMap    | Yes    | Worldwide             | Yes     | Yes        | Full                | Yes                | Full              | Yes              | Yes | Yes      | 0.5            | https://openweathermap.org/ |

† Supported by service but not by weathercli
