mkfile_path := $(abspath $(lastword $(MAKEFILE_LIST)))
mkfile_dir := $(dir $(mkfile_path))
target_dir := $(mkfile_dir)/target

pkg: touch_pkg_dir
	cd abuild && abuild -r -P $(target_dir)

touch_pkg_dir: cleanpkg
	mkdir -p ./target/gnirehtet-relay/x86_64

.PHONY: clean cleanpkg
clean:
	rm -rf target
cleanpkg:
	rm -rf target/gnirehtet-relay

