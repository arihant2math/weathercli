build: .
	pyinstaller -F weather.py
docs: docs_templates/
	touch docs/index.html
	./jc index.html "./docs/index.html" --template-dir "./docs_templates"
	touch docs/index.json
	./jc index.json "./docs/index.json" --template-dir "./docs_templates"
	cp docs_templates/hero.png docs/hero.png
	cp docs_templates/weather.exe docs/weather.exe
	cp docs_templates/weather docs/weather
	cp docs_templates/updater.exe docs/updater.exe
	cp docs_templates/updater docs/updater
clean:
	rm -rf docs
	mkdir docs
