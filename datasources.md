# Data Sources
## Comparison
| Data Source       | Metric            | Regional Availability | Min/Max | Cloud Data | Conditions Sentence | Special Conditions | Forecast Sentence | API Key Required | AQI | Website                     |
|-------------------|-------------------|-----------------------|---------|------------|---------------------|--------------------|-------------------|------------------|-----|-----------------------------|
| NWS               | Only Temperatures | US only               | No      | No         | Clouds only         | No                 | No                | No               | No  | https://www.weather.gov/    |
| TheWeatherChannel | Regional          | Worldwide             | Yes     | No         | No                  | No                 | No                | No               | Yes | https://weather.com         |
| Meteo             | Yes               | Worldwide             | Yes     | Yes        | Full                | No                 | Full              | No               | Yes | https://open-meteo.com      |
| OpenWeatherMap    | Yes               | Worldwide             | Yes     | Yes        | Full                | Yes                | Full              | Yes              | Yes | https://openweathermap.org/ |
Note that while some services may support a feature, this table only indicated weathercli's support of the feature.