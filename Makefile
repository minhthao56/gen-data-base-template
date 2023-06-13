watch:
	trunk serve --open

serve:
	serve -s dist

build:
	trunk build

watch-css:
	npx tailwindcss -i ./input.css -o ./style/output.css --watch