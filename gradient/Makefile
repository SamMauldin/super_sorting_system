.PHONY: all

all:
	npm run build
	sudo rm -rf /usr/share/caddy/gradient
	sudo mv build /usr/share/caddy/gradient
	sudo chown root:root -R /usr/share/caddy/gradient
