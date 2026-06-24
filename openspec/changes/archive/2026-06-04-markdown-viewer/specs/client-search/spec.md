## ADDED Requirements

### Requirement: Client-side search
The system SHALL provide fuse.js-based client-side search across filenames and content.

#### Scenario: Search input visible
- **WHEN** page loads
- **THEN** search input visible in top navigation bar

#### Scenario: Search by filename
- **WHEN** user types search query
- **THEN** results include files matching query in filename

#### Scenario: Search by content
- **WHEN** user types search query
- **THEN** results include files containing query in content

#### Scenario: Search by headings
- **WHEN** user types search query
- **THEN** results include files matching query in headings

#### Scenario: Fuzzy search
- **WHEN** user types partial or misspelled query
- **THEN** results include fuzzy matches ranked by relevance

#### Scenario: Search results display
- **WHEN** search returns results
- **THEN** dropdown shows list of matching files with title and snippet

#### Scenario: Search result navigation
- **WHEN** user clicks search result
- **THEN** page navigates to that file

#### Scenario: No results
- **WHEN** search returns no results
- **THEN** dropdown shows "No results found" message

#### Scenario: Search index generation
- **WHEN** system generates HTML for folder mode
- **THEN** search index embedded in index.html with all filenames and content
