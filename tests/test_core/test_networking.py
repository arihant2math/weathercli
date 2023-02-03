import json

from core import networking


def test_get_url():
    r = networking.get_url("https://httpbin.org/get")
    j = json.loads(r)
    assert j["args"] == {}
    assert j["headers"]["User-Agent"] == "weathercli/1"


def test_get_url_custom_ua():
    r = networking.get_url("https://httpbin.org/get", "test_ua")
    j = json.loads(r)
    assert j["headers"]["User-Agent"] == "test_ua"


def test_get_url_custom_headers():
    r = networking.get_url("https://httpbin.org/get", "test_ua", {"Foo": "bar"})
    j = json.loads(r)
    assert j["headers"]["Foo"] == "bar"


def test_get_urls():
    r = networking.get_urls(
        ["https://httpbin.org/get", "https://httpbin.org/user-agent"]
    )
    assert len(r) == 2
