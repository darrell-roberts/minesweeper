
PLATFORM := $(shell uname)

all: build

clean-dist:
	rm -rf dist/

check:
	cargo clippy

build: check
	cargo build --release  --bin minesweeper-iced

bundle-mac: clean-dist build
	# Create necessary directories
	mkdir -p "dist/dmg"
	mkdir -p "dist/MineSweeper.app/Contents/MacOS"
	mkdir -p "dist/MineSweeper.app/Contents/Resources"

	# Copy application files
	cp assets/icon.icns "dist/MineSweeper.app/Contents/Resources"
	cp assets/Info.plist "dist/MineSweeper.app/Contents"
	cp target/release/minesweeper-iced "dist/MineSweeper.app/Contents/MacOS"
	chmod +x "dist/MineSweeper.app/Contents/MacOS/minesweeper-iced"

	codesign --sign "MyApps" "dist/MineSweeper.app"

	# Copy app to DMG staging area
	cp -r "dist/MineSweeper.app" "dist/dmg"

	# Create temporary DMG
	hdiutil create -size 100m -fs HFS+ -volname "MineSweeper" -o "dist/temp.dmg"

	# Mount temporary DMG
	hdiutil attach "dist/temp.dmg" -mountpoint "/Volumes/MineSweeper"

	# Copy contents to DMG
	cp -r "dist/dmg/MineSweeper.app" "/Volumes/MineSweeper"

	# Create Applications shortcut
	ln -s /Applications "/Volumes/MineSweeper/Applications"

	# Unmount
	hdiutil detach "/Volumes/MineSweeper"

	# Convert to compressed DMG
	hdiutil convert "dist/temp.dmg" -format UDZO -imagekey zlib-level=9 -o "dist/MineSweeper.dmg"

	# Clean up
	rm "dist/temp.dmg"

linux-app-image: clean-dist build
	echo "Building linux app image"
	rm -rf dist/AppDir
	# create new AppDir
	linuxdeploy-x86_64.AppImage --appdir dist/AppDir

	# Copy contents into AppDir
	cp target/release/minesweeper-iced dist/AppDir/usr/bin
	cp assets/minesweeper.desktop dist/AppDir/usr/share/applications
	cp -r assets/icons dist/AppDir/usr/share

	# Create app image
	linuxdeploy-x86_64.AppImage --appdir dist/AppDir --output appimage

install-local-linux: build
	echo "Installing for linux"
	mkdir -p ~/.local/share/applications
	mkdir -p ~/.local/share/icons
	mkdir -p ~/.local/bin
	cp target/release/minesweeper-iced ~/.local/bin
	cp assets/minesweeper.desktop ~/.local/share/applications
	cp -r assets/icons ~/.local/share/icons

install:
ifeq ($(PLATFORM), Darwin)
	@echo "Installing for Mac"
	@$(MAKE) bundle-mac
	open "dist/MineSweeper.dmg"
else ifeq ($(PLATFORM), Linux)
	@echo "Installing for Linux"
	@$(MAKE) install-local-linux
else
	@echo "Unsupported platform for install: " $(PLATFORM)
endif

.PHONY: all clean-dist check build bundle-mac install-local-linux install
