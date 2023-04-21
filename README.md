<img src="./icon/icon.svg" alt="Logo" height="100" width="100">

# Weather CLI
[![Website](https://github.com/arihant2math/weathercli/actions/workflows/pages/pages-build-deployment/badge.svg)]([https://github.com/arihant2math/weathercli/actions/workflows/pages/pages-build-deployment](https://arihant2math.github.io/weathercli/index.html))

Weather in your terminal, now 100% rust!
## Usage
For the open weather map api to work you need to configure your API key.
Get an API key and run `weather config open_weather_map_api_key [your api key here]`
If you want better geo-positioning, get a Bing Maps API Key and run `weather config bing_maps_api_key [your key here]`
More config values can be found in the table below
## Config
| Name                     | Values                                        | Function                                                                                                       |
|--------------------------|-----------------------------------------------|----------------------------------------------------------------------------------------------------------------|
| open_weather_map_api_key | any string                                    | The API key for Open Weather Maps                                                                              |
| bing_maps_api_key        | any string                                    | The API key for Bing Maps                                                                                      |
| ncdc_api_key             | any string                                    | NOAA NCDC API KEY (unused as of now)                                                                           |
| metric_default           | true, false                                   | if true, the default units will be metric                                                                      |
| default_backend          | METEO, NWS, THEWEATHERCHANNEL, OPENWEATHERMAP | sets the default backend to get data from, see datasources.md for more info                                    |
| constant_location        | true, false                                   | if true, the users current location will be cached                                                             |

Usage: `weather config [NAME] [VALUE]`, to get the value of a config name try `weather config [NAME]`
## Custom Layouts
See custom_layouts.md for more info.
