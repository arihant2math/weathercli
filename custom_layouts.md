# Custom Layouts

Layouts are json files which contain information about how you would like the application to look.

## Format

There are two major sections in the root of the json file, `defaults` and `layout`. A version key is required, with the
only version currently being 0.
Your json file should look like this:

```json
{
    "version": 3,
    "defaults": {
        
    },
    "layout": {
        
    }
}
```

### Defaults

This section is optional but sets important defaults that will be used.

| Key            | Values                       |
|----------------|------------------------------|
| variable_color | coloroma.Fore.{variablename} |
| text_color     | coloroma.Fore.{variablename} |
| unit_color     | coloroma.Fore.{variablename} |

possible values are as follows for colors:

```
BLACK          
RED            
GREEN          
YELLOW         
BLUE           
MAGENTA        
CYAN           
WHITE          
RESET          
LIGHTBLACK_EX  
LIGHTRED_EX    
LIGHTGREEN_EX  
LIGHTYELLOW_EX 
LIGHTBLUE_EX   
LIGHTMAGENTA_EX
LIGHTCYAN_EX   
LIGHTWHITE_EX  
```

### Layout

The layout should be a list of rows, which are themselves lists.
Each row contains `LayoutItems`, which are of the following format.

```json
{
    "type": "text",
    "data": {
        
    }
}
```

The type can be either `text`, `variable`, or `function`

#### Shorthand notation

Simply use this in a string the place of a layout item

* Text: `text here`
* Variable: `@current_weather.wind.speed|mph|km/h` or `@region`
* Function: `#bar` or `#color_aqi|5` or even `#color_aqi|@current_weather.aqi` or `#color_aqi|aqi=@current_weather.aqi`

#### Text

The data section should have a key `text` with the text you want to display.

#### Variable

The data section should have a key named `name` with the name of the variable you want to retrieve, variables are
retrieved from a `WeatherData` class.

#### Function

Functions are gotten from `cli/layout/util.py`

## Things to note

* To nest list items try `[n].[i]` instead of `[n][i]`

## Pointing weathercli to the layout

Set the `DEFAULT_LAYOUT` setting to the location of the layout relative to `~/.weathercli/layouts`