# Migration and Upgrade Plan

This document outlines the upgrade plan for dependencies in the `dr-md` project and provides instructions on managing customizations to the vendored `egui_commonmark` crate.

## 1. Local Crate Customizations Patch

A patch file containing all current customizations to `egui_commonmark` (relative to the official `v0.15.0` version) is stored in the project repository:

📁 **Patch File Location:** [egui_commonmark_customizations.patch](file:///home/lnoxsian/lnox-files/project-rust/dr-md/patches/egui_commonmark_customizations.patch)

### How to Apply the Patch During Migration:
When upgrading to a newer version of `egui_commonmark`:
1. **Download/Vendor the new version** of `egui_commonmark` to `crates/egui_commonmark`.
2. **Apply the patch file** from the workspace root:
   ```bash
   git apply --directory=crates/egui_commonmark patches/egui_commonmark_customizations.patch
   ```
3. Resolve any conflicts in `crates/egui_commonmark/src/elements.rs` or `crates/egui_commonmark/src/lib.rs` if the upstream code has drifted.

---

## 2. Dependency Upgrade Plan

Upgrading the remaining dependencies should be done in structured phases to minimize compilation issues.

### Phase 1: Upgrading GUI Stack (Lockstep)
The `egui` ecosystem upgrades must be done together since minor version increments are often breaking and tightly coupled.

1. **Target Versions**:
   - `egui` & `eframe`: Upgrade from `0.27` to `0.28` (or the latest stable version).
   - `egui_extras`: Upgrade to match the `egui` version (e.g. `0.28`).
   - `egui_commonmark`: Upgrade the base vendored version (e.g. `0.16.x` or similar compatible with the chosen `egui` version).
2. **Cargo.toml Updates**:
   Update both the root [Cargo.toml](file:///home/lnoxsian/lnox-files/project-rust/dr-md/Cargo.toml) and the local [crates/egui_commonmark/Cargo.toml](file:///home/lnoxsian/lnox-files/project-rust/dr-md/crates/egui_commonmark/Cargo.toml) to reference the same version of `egui`.
3. **Code Changes**:
   - Review and update any API changes in [renderer.rs](file:///home/lnoxsian/lnox-files/project-rust/dr-md/src/editor/renderer.rs) and [tree.rs](file:///home/lnoxsian/lnox-files/project-rust/dr-md/src/explorer/tree.rs).

### Phase 2: Upgrading Markdown Parsing
- **pulldown-cmark**: Currently at `0.10`. Upgrading to `0.11+` will require matching version updates in `egui_commonmark` as well since it uses it under the hood.

### Phase 3: Upgrading Utility Crates (Safe & Independent)
These crates can be upgraded independently as they do not affect the GUI layer:
- `image`: Update to `0.25` patch releases.
- `notify`: Update to `6.1.1` (or latest `6.x` minor release).
- `directories`: Update to `5` latest patch release.
- `arboard`: Update to `3.6.1+` patch release.

---

## 3. Post-Upgrade Verification
After completing any upgrade steps, run the following to verify:
```bash
cargo check
cargo test
```
