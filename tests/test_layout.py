from cli.layout.layout import Layout


def test_text():
    layout_dict = {
        "version": 3,
        "layout": [
            [
                {"type": "text", "value": "foo"},
                "bar",
            ]
        ],
    }
    layout = Layout(text=layout_dict)
    assert "foo" in (layout.to_string({}, False))
    assert "bar" in (layout.to_string({}, False))


def test_variable():
    layout_dict = {
        "version": 3,
        "layout": [[{"type": "variable", "value": "[0].[1]"}]],
    }
    layout = Layout(text=layout_dict)
    assert "buzz" in layout.to_string([["foo", "buzz"], "bar"], False)


def test_variable_shorthand():
    layout_dict = {
        "version": 3,
        "layout": [["@[0].[0]"]]
    }
    layout = Layout(text=layout_dict)
    assert "foo" in layout.to_string([["foo", "buzz"], "bar"], False)
