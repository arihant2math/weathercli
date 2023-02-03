from click.testing import CliRunner

from cli.local import settings
from weather import main


def test_main():
    runner = CliRunner()
    result = runner.invoke(main)
    assert not result.exception
    assert result.exit_code == 0


def test_settings():
    settings.store_key("FOO", "bar")
    assert settings.get_key("FOO", "not bar") == "bar"
    assert settings.get_key("FOOBAR", "BARFOO") == "BARFOO"
    settings.delete_key("FOO")
    settings.delete_key("FOOBAR")
