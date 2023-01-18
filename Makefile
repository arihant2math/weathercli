build: .
	pyinstaller -F weather.py
docs: docs_templates/
	touch docs/index.html
	touch ./docs/index.html.tmp
	./jc index.html "./docs/index.html.tmp" --template-dir "./docs_templates"
	./minify --output ./docs/index.html --keep-closing-tags --minify-css ./docs/index.html.tmp
	rm ./docs/index.html.tmp

	touch docs/config.html
	touch ./docs/config.html.tmp
	./jc config.html "./docs/config.html.tmp" --template-dir "./docs_templates"
	./minify --output ./docs/config.html --keep-closing-tags --minify-css ./docs/config.html.tmp
	rm ./docs/config.html.tmp

	touch docs/index.json
	./jc index.json "./docs/index.json" --template-dir "./docs_templates"
	cp docs_templates/hero.png docs/hero.png
	cp docs_templates/logo.png docs/logo.png
	cp docs_templates/weather.exe docs/weather.exe
	cp docs_templates/weather docs/weather
	cp docs_templates/updater.exe docs/updater.exe
	cp docs_templates/updater docs/updater
	cp docs_templates/theme.js docs/theme.js
clean:
	rm -rf docs
	mkdir docs
