from cli.local import settings


def test_settings():
    settings.store_key("FOO", "bar")
    assert settings.get_key("FOO", "not bar") == "bar"
    assert settings.get_key("FOOBAR", "BARFOO") == "BARFOO"
    settings.delete_key("FOO")
    settings.delete_key("FOOBAR")
