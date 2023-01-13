# Data Sources
## Comparison
| Data Source       | Metric            | Regional Availability | Min/Max | Cloud Data | Conditions Sentence | Special Conditions | Forecast Sentence | API Key Required | AQI | Time (seconds) | Website                     |
|-------------------|-------------------|-----------------------|---------|------------|---------------------|--------------------|-------------------|------------------|-----|----------------|-----------------------------|
| NWS               | Only Temperatures | US only               | Yes     | Yes        | Full                | No†                | No†               | No               | No† | 0.5            | https://www.weather.gov/    |
| TheWeatherChannel | Regional          | Worldwide             | Yes     | No†        | No                  | No                 | No                | No               | Yes | ?              | https://weather.com         |
| Meteo             | Yes               | Worldwide             | Yes     | Yes        | Full                | No                 | Full              | No               | Yes | 0.9            | https://open-meteo.com      |
| OpenWeatherMap    | Yes               | Worldwide             | Yes     | Yes        | Full                | Yes                | Full              | Yes              | Yes | 0.5            | https://openweathermap.org/ |

† Supported by service but not by weathercli
