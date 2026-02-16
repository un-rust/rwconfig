---
layout: home

hero:
  name: "rwconfig"
  text: "Config with get/set, save when ready"
  tagline: Read config files, change values by path, write everything back in one save.
  actions:
    - theme: brand
      text: Guide
      link: /guide
    - theme: alt
      text: API Reference
      link: /api

features:
  - title: Get/set by path
    details: Use dot paths like "info.a" or "a.b.c" to read and write nested values. Parent keys are created automatically on set.
  - title: Dirty tracking
    details: Every set() marks the config dirty. save() writes to disk only when there are changes, and clears the dirty flag.
  - title: Multi-format
    details: JSON, JSON5, JSONC, YAML, and TOML. Format is inferred from the file extension.
---
