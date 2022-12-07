import requests
from bs4 import BeautifulSoup


def get_data(location):
    r = requests.get("https://weather.com/weather/today/l/" + str(location[0]) + "," + str(location[1]))

    soup = BeautifulSoup(r.content, "html.parser")
    result = soup.find('span', attrs={'data-testid': 'TemperatureValue'})
    temperature = result.text


print()
# TemperatureValue
