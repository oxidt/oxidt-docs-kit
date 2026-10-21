Vendored copy of the [oxidt-ui](https://github.com/oxidt/oxidt-ui) theme at
the tag in `VERSION`: monochrome tokens and daisyUI-compatible component
classes built as Tailwind `@utility` rules, so the kits' stylesheets need no
daisyUI plugin. The kits import `tokens.css` and `components.css`.

Do not edit here: change oxidt-ui, tag a release, bump `TAG` in
`scripts/vendor-oxidt-ui.sh` and run `just oxidt-ui`. CI fails if this copy
differs from the pinned tag.
