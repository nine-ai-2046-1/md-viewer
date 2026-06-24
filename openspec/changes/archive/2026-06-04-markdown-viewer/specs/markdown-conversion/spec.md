## ADDED Requirements

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
