## ADDED Requirements

### Requirement: Rewrite relative .md hyperlinks to .html
The system SHALL post-process rendered HTML to convert relative `.md` hyperlinks to `.html` so that markdown cross-references resolve to the correct rendered pages.

#### Scenario: Simple .md link rewritten
- **WHEN** markdown contains `[text](./other.md)`
- **THEN** rendered HTML contains `<a href="./other.html">text</a>`

#### Scenario: Nested .md link rewritten
- **WHEN** markdown contains `[text](sub/page.md)`
- **THEN** rendered HTML contains `<a href="sub/page.html">text</a>`

#### Scenario: Parent directory .md link rewritten
- **WHEN** markdown contains `[text](../README.md)`
- **THEN** rendered HTML contains `<a href="../README.html">text</a>`

#### Scenario: External URL not rewritten
- **WHEN** markdown contains `[text](https://example.com)`
- **THEN** rendered HTML contains `<a href="https://example.com">text</a>` (unchanged)

#### Scenario: Anchor link not rewritten
- **WHEN** markdown contains `[text](#heading)`
- **THEN** rendered HTML contains `<a href="#heading">text</a>` (unchanged)

#### Scenario: Image path not rewritten
- **WHEN** markdown contains `![alt](./image.png)`
- **THEN** rendered HTML contains `<img src="./image.png">` (unchanged, served via /source/)
