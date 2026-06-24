## Purpose

The markdown conversion module transforms markdown files into styled HTML pages with syntax highlighting, table of contents, and cross-reference link rewriting.

## Requirements

### Requirement: Markdown to HTML conversion
The system SHALL convert markdown files to HTML using pulldown-cmark parser with CommonMark support.

#### Scenario: Single file conversion
- **WHEN** user provides a markdown file path
- **THEN** system converts it to valid HTML with proper doctype, charset, and viewport meta tags

#### Scenario: CommonMark compliance
- **WHEN** markdown contains headings, lists, code blocks, links, images
- **THEN** system renders them according to CommonMark specification

#### Scenario: Syntax highlighting
- **WHEN** markdown contains fenced code blocks with language identifier
- **THEN** system applies Prism.js syntax highlighting with correct language class

#### Scenario: Table support
- **WHEN** markdown contains pipe-delimited tables
- **THEN** system renders them as HTML tables with proper thead/tbody structure

#### Scenario: Task list support
- **WHEN** markdown contains task list items (- [ ] or - [x])
- **THEN** system renders them as checkbox input elements

### Requirement: Table of contents generation
The system SHALL automatically generate a table of contents from heading elements.

#### Scenario: TOC generation from headings
- **WHEN** markdown contains h1, h2, h3 headings
- **THEN** system generates nested list TOC with anchor links to each heading

#### Scenario: TOC depth configuration
- **WHEN** markdown contains headings at multiple levels
- **THEN** system includes all heading levels in TOC (h1-h6)

### Requirement: HTML output format
The system SHALL generate complete, self-contained HTML files.

#### Scenario: Complete HTML structure
- **WHEN** system converts markdown to HTML
- **THEN** output contains <!DOCTYPE html>, <html>, <head>, <body> elements

#### Scenario: Responsive viewport
- **WHEN** system generates HTML
- **THEN** head contains viewport meta tag for mobile responsiveness

#### Scenario: Theme support
- **WHEN** system generates HTML
- **THEN** html element contains data-theme attribute (default: "dark")

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

### Requirement: Rewrite relative image paths with /source/ prefix
The system SHALL post-process rendered HTML to prefix relative image `src` attributes with `/source/` so that images referenced in markdown load correctly from the source directory via the `/source/*` HTTP route.

#### Scenario: Simple relative image path rewritten
- **WHEN** markdown contains `![alt](screenshots/photo.png)`
- **THEN** rendered HTML contains `<img src="/source/screenshots/photo.png" alt="alt" />`

#### Scenario: Dot-prefixed relative path rewritten
- **WHEN** markdown contains `![alt](./images/diagram.png)`
- **THEN** rendered HTML contains `<img src="/source/images/diagram.png" alt="alt" />`

#### Scenario: Parent directory relative path rewritten
- **WHEN** markdown contains `![alt](../shared/image.png)`
- **THEN** rendered HTML contains `<img src="/source/../shared/image.png" alt="alt" />`

#### Scenario: Nested subdirectory path rewritten
- **WHEN** markdown contains `![alt](a/b/c/deep.png)`
- **THEN** rendered HTML contains `<img src="/source/a/b/c/deep.png" alt="alt" />`

#### Scenario: Absolute URL not rewritten
- **WHEN** markdown contains `![alt](https://example.com/image.png)`
- **THEN** rendered HTML contains `<img src="https://example.com/image.png" alt="alt" />` (unchanged)

#### Scenario: Protocol-relative URL not rewritten
- **WHEN** markdown contains `![alt](//cdn.example.com/image.png)`
- **THEN** rendered HTML contains `<img src="//cdn.example.com/image.png" alt="alt" />` (unchanged)

#### Scenario: Data URI not rewritten
- **WHEN** markdown contains `![alt](data:image/png;base64,abc...)`
- **THEN** rendered HTML contains `<img src="data:image/png;base64,abc..." alt="alt" />` (unchanged)

#### Scenario: Path already prefixed not double-prefixed
- **WHEN** markdown contains `![alt](/source/screenshots/photo.png)`
- **THEN** rendered HTML contains `<img src="/source/screenshots/photo.png" alt="alt" />` (unchanged)
