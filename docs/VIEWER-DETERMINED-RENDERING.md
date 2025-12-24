# Viewer-Determined Rendering — Numbers

This document defines normative constraints for Numbers.

Numbers inscriptions contain the number only.

Rendering is determined entirely by the viewing environment.

## Constraints

- Numbers must be rendered as text, not images.
- No custom fonts are specified or embedded.
- No visual styling is applied at inscription time.
- Interfaces must not replace text with pixel based renderings. (e.g.Canvas or WebGL)
- Motion may be applied to containers, not to the glyph itself.
- Accessibility and text semantics must be preserved.

## Implications

Numbers do not carry a visual identity.

Each viewing environment renders the number according to:
- Its default font
- Its text shaping and rasterization
- Its accessibility and display settings

This is not a customization feature.
It is a refusal to author appearance.

## Rationale

Rendering neutrality ensures that Numbers remain:

- Portable across contexts
- Legible without interpretation
- Free from aesthetic authorship
- Stable over time and devices

The system defines content.
The viewer’s machine defines appearance.
