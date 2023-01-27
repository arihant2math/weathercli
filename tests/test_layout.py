from cli.layout.layout import Layout


def test_text():
    layout_dict = {
        "layout": [
            [
                {"type": "text", "data": {"text": "foo"}},
                {"type": "text", "data": {"text": "bar"}},
            ]
        ]
    }
    layout = Layout(text=layout_dict)
    assert "foo" in (layout.to_string({}, False))
    assert "bar" in (layout.to_string({}, False))


def test_variable():
    layout_dict = {
        "layout": [
            [
                {
                    "type": "variable",
                    "data": {
                        "name": "[0]"
                    }
                }
            ]
        ]
    }
    layout = Layout(text=layout_dict)
    assert "hi" in layout.to_string(["hi", "bye"], False)
    assert "bye" not in layout.to_string(["hi", "bye"], False)
