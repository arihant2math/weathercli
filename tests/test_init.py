from click.testing import CliRunner

from weather import main


def test_main():
    runner = CliRunner()
    result = runner.invoke(main)
    assert not result.exception
    assert result.exit_code == 0
