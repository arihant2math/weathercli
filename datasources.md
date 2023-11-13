# Data Sources

## Comparison

|                       | NWS                 | Meteo                   | OpenWeatherMap              | OpenWeatherMap OneCall (2.5)            |
|-----------------------|---------------------|-------------------------|-----------------------------|-----------------------------------------|
| Metric                | Yes                 | Yes                     | Yes                         | Yes                                     |
| Regional Availability | U.S. Only           | Worldwide               | Worldwide                   | Worldwide                               |
| Min/Max               | Yes                 | Yes                     | Yes                         | Yes                                     |
| Cloud Data            | Yes                 | Yes                     | Yes                         | Yes                                     |
| Conditions Sentence   | Yes                 | Yes                     | Yes                         | Yes                                     |
| Special Conditions    | No†                 | No†                     | Partial†                    | Yes                                     |
| Forecast Sentence     | No                  | Yes                     | Yes                         | Yes                                     |
| API Key Required      | No                  | No                      | Yes                         | No                                      |
| AQI                   | No                  | Yes                     | Yes                         | No                                      |
| Forecast              | No                  | Yes (1 hour)            | Yes (3 hours)               | Yes (1 hour, minute implementation TBD) |
| Network Requests      | 1+reverse geocode   | 1+reverse geocode       | 3+reverse geocode           | 1+reverse geocode                       |
| Website               | https://weather.gov | https://open-meteo.com/ | https://openweathermap.org/ | https://openweathermap.org/             |

Reverse Geocode requests are cached

† Supported by service but not by weathercli
