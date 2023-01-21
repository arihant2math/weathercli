build: .
	pyinstaller -F weather.py
docs: docs_templates/
	touch docs/index.html
	./jc index.html "./docs/index.html" --template-dir "./docs_templates"

	touch docs/config.html
	./jc config.html "./docs/config.html" --template-dir "./docs_templates"

	touch docs/index.json
	./jc index.json "./docs/index.json" --template-dir "./docs_templates" --no-minify
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
	rm -rf build
	rm -rf dist
	rm -rf target
	rm -rf updater/target
