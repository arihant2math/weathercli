from cli.layout.layout import Layout


def test_text():
    layout_dict = {
        "version": 0,
        "layout": [
            [
                {"type": "text", "data": {"text": "foo"}},
                {"type": "text", "data": {"text": "bar"}},
            ]
        ],
    }
    layout = Layout(text=layout_dict)
    assert "foo" in (layout.to_string({}, False))
    assert "bar" in (layout.to_string({}, False))


def test_variable():
    layout_dict = {
        "version": 0,
        "layout": [[{"type": "variable", "data": {"name": "[0].[1]"}}]],
    }
    layout = Layout(text=layout_dict)
    assert "foobar" in layout.to_string([["foo", "foobar"], "bar"], False)
