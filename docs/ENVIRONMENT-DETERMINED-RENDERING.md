# Environment-Determined Rendering — Numbers

This document assumes familiarity with INVARIANTS.md and API-SPEC.md.

This document defines how Numbers are rendered at view time. 

This document is non-normative.
It describes presentation consequences only and does not affect auction, settlement, inscription, or recognition semantics.

An environment may include a browser, operating system, wallet, indexer, or display context.

Numbers inscriptions contain the number only.
No appearance is specified at inscription time.

Rendering is determined entirely by the viewing environment at view time.

## Rendering Model

Numbers inscriptions encode textual numerals only.

Each environment renders the number using its own:
- default fonts
- text shaping and rasterization
- accessibility and display settings

The system does not specify or embed presentation details.

## Constraints

To preserve environment-determined rendering:

- Numbers are rendered as text, not images.
- No fonts are specified or embedded.
- No styling is applied at inscription time.
- Text must not be replaced with pixel-based renderings.
- The number itself is never animated.
- Text semantics and accessibility are delegated to the environment.

Environments may violate these constraints without affecting the validity or recognition of an inscription.

## Implications

Numbers have no canonical appearance.

The same number may appear differently across environments.
These differences are expected and accepted.

Rendering variation is not customization.
It is the consequence of deferring appearance to the environment.
No variation changes the recorded outcome or the system’s recognition of that outcome.