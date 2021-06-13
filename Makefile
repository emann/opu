define help-message
usage: make <target> [target ...]

TARGETS

Cargo Wrappers:
  These targets wrap built-in Cargo commands with default options.

  build:     build a debug executable
  check:     check for possible compiler warnings/errors - faster than <build>
  clean:     clean up build files
  clippy:    like check, but more thorough
  doc:       build documentation for this crate and open in default browser
  fmt:       auto-format the code in-place for consistency
  fmt-check: exit 1 if <fmt> would make any changes
  run:       build and run the crate
  test:      run all unit tests
  tree:      show a dependency tree for this crate
  update:    update Cargo.lock with the newest versions of crates compatible with Cargo.toml

External Cargo Commands:
  These targets wrap external Cargo subcommands and ensure they are installed
  before running them. They are separated into two groups: developer-facing and
  internal-only.

  Developer-facing:

  audit:           alias for <deny-advisories>
  check-unsafe:    alias for <geiger>
  deny-advisories: check dependencies against a database of security advisories
  deny-check:      run all cargo-deny checks against this crate's dependencies
  geiger:          check dependencies for use of `unsafe`
  license:         print a list of licenses and which dependencies use them
  outdated:        check for outdated dependencies
  update-bin:      update all cargo subcommands to their newest versions
                   including ones not installed by this Makefile

  Internal-Only:

  ensure-cargo-%:  a pattern rule for installing cargo subcommands
                   depending on ensure-cargo-foo installs subcommand cargo-foo

Local CI
  These targets use github.com/nektos/act to locally run the configured GitHub
  actions for this repository.

  ensure-act: prequisite target that makes sure `act` is installed
  ci:         run all CI jobs for this repository once
  ci-list:    list all CI jobs
  ci-watch:   run all CI jobs for this repository whenever files change
  ci-sync:    run each CI job synchronously
              may be a required workaround for <ci> attempting to use a
			  container multiple times
  ci-%:       run the given CI job
              ci-foo will run the job with ID foo
endef

.PHONY: help
help:
	$(info $(help-message))
	@# Avoid message "nothing to be done for: help"
	@true

################# LOCAL ACTIONS #############################

.PHONY: ensure-act
ensure-act:
	@if ! which act &> /dev/null; then\
		if which brew &> /dev/null; then\
			@echo Installing act using Homebrew;\
			brew install act;\
		else\
			@echo github.com/nektos/act is required to continue;\
			@exit 1;\
		fi;\
	fi

.PHONY: ci
ci: ensure-act
	act

.PHONY: ci-list
ci-list: ensure-act
	act -l

.PHONY: ci-watch
ci-watch: ensure-act
	act --watch

.PHONY: ci-sync
ci-sync: ensure-act
	for job in $$(act -l | tail -n+2 | cut -f1 -d' '); do\
		act -j $$job;\
	done

.PHONY: ci-%
ci-%: ensure-act
	act -j $* -v

################## BASIC CARGO COMMANDS #####################

.PHONY: build
build:
	cargo build
	@echo "You can find the build executable at target/debug/$(basename $(pwd))"

.PHONY: check
check:
	cargo check

.PHONY: clean
clean:
	cargo clean

.PHONY: clippy
clippy:
	cargo clippy

.PHONY: doc
doc:
	cargo doc --workspace --document-private-items --no-deps --open # --all-features

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: check-fmt
check-fmt:
	cargo fmt -- --check

.PHONY: run
run:
	cargo run

.PHONY: test
test: ensure-cargo-all-features
	cargo test-all-features

.PHONY: tree
tree:
	cargo tree

.PHONY: update
update:
	cargo update

################## EXTERNAL SUBCOMMANDS #####################

.PHONY: audit
audit: deny-advisories

.PHONY: check-unsafe
check-unsafe: geiger

.PHONY: deny-check
deny-check: ensure-cargo-deny
	cargo deny check

.PHONY: geiger
geiger: ensure-cargo-geiger
	cargo geiger

.PHONY: license
license: ensure-cargo-license
	cargo license

.PHONY: outdated
outdated: ensure-cargo-outdated
	cargo outdated

################ SUBCOMMANDS INSTALLATION ###################

.PHONY: ensure-cargo-%
ensure-cargo-%:
	@if [ -n "$*" ] && ! which cargo-$* &> /dev/null; then \
		cargo install cargo-$*; \
	fi

.PHONY: update-bin
update-bin: ensure-cargo-update
	cargo install-update -a
