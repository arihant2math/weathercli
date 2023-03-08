# weathercli
[![Website](https://github.com/arihant2math/weathercli/actions/workflows/pages/pages-build-deployment/badge.svg)]([https://github.com/arihant2math/weathercli/actions/workflows/pages/pages-build-deployment](https://arihant2math.github.io/weathercli/index.html))
[![CI](https://github.com/arihant2math/weathercli/actions/workflows/build.yml/badge.svg)](https://github.com/arihant2math/weathercli/actions/workflows/build.yml)
## Usage
For the open weather map api to work you need to configure your API key.
Get an API key and run `weather config OPEN_WEATHER_MAP_API_KEY --value [your api key here]`
If you want better geo-positioning, get a Bing Maps API Key and run `weather config BING_MAPS_API_KEY --value [your key here]`
More config values can be found in the table below
## Config
| Name                     | Values                                        | Function                                                                                                       |
|--------------------------|-----------------------------------------------|----------------------------------------------------------------------------------------------------------------|
| OPEN_WEATHER_MAP_API_KEY | any string                                    | The API key for Open Weather Maps                                                                              |
| BING_MAPS_API_KEY        | any string                                    | The API key for Bing Maps                                                                                      |
| NCDC_API_KEY             | any string                                    | NOAA NCDC API KEY (unused as of now)                                                                           |
| METRIC_DEFAULT           | true, false                                   | if true, the default units will be metric                                                                      |
| WEATHER_DATA_HASH        | any string                                    | the sha512 hash of ~/.weathercli/weather_codes.json if the hashes don't match an updated version is downloaded |
| DEFAULT_BACKEND          | METEO, NWS, THEWEATHERCHANNEL, OPENWEATHERMAP | sets the default backend to get data from, see datasources.md for more info                                    |
| CONSTANT_LOCATION        | true, false                                   | if true, the users current location will be cached                                                             |

Usage: `weather config [NAME] --value [VALUE]`, to get the value of a config name try `weather config [NAME]`
## Custom Layouts
WIP, for now edit `~/.weathercli/layouts/default.json`
