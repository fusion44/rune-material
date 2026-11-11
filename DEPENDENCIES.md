# Dependencies and third-party material

Project source is Apache-2.0; see `LICENSE`. Dependency licenses remain their own.
Cargo.lock, flake.lock and devenv.lock are committed. No crates.io publication is
planned while Rune requires Git dependencies.

- **Rune:** [anomalyco/rune](https://github.com/anomalyco/rune), MIT, pinned in
  Cargo.toml/Cargo.lock along with its required downstream patches. Workspace
  patches must be repeated at a consumer's root; see README.md.
- **Material Symbols Rounded:** Google, Apache-2.0. The exact revision, update
  procedure and license are in `assets/material-symbols/`.
- **Fonts:** Rune embeds its own fallback fonts. See the pinned upstream source's
  font license files; system-installed fonts are not relicensed by this project.
- Rust dependency licenses and Git/registry origins are checked by
  `cargo deny check licenses sources`. The allowlist is in `deny.toml`.
- Known vulnerability checks use `cargo audit`. Audit Git-patched dependencies
  against their upstream changes too: registry advisory matching is not proof
  that a Git fork contains every security fix.

## Reviewed maintenance warnings (2026-11-16)

The library lockfile has no known vulnerabilities in the audit database checked
on this date. `zbus_names 4.3.5`, `zvariant 5.15.1`, and `zvariant_derive 5.15.1`
report RUSTSEC-2026-0371, RUSTSEC-2026-0370, and RUSTSEC-2026-0372 respectively:
the standalone/older crates have been superseded upstream. These are maintenance
notices, not vulnerability waivers. They remain visible in audit output; migration
depends on upstream consumers. Re-audit and review this list before every release.

Binary redistributors must ship required third-party copyright/license notices,
including embedded fonts, not only this project's LICENSE. See RELEASE.md.
