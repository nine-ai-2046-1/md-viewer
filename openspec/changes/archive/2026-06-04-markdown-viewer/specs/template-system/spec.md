## ADDED Requirements

### Requirement: Docusaurus-inspired UI
The system SHALL provide a Docusaurus-inspired user interface with dark/light theme.

#### Scenario: Dark theme default
- **WHEN** user opens page
- **THEN** theme defaults to dark (data-theme="dark")

#### Scenario: Theme toggle
- **WHEN** user clicks theme toggle button
- **THEN** theme switches between dark and light
- **AND** preference saved to localStorage

#### Scenario: Theme persistence
- **WHEN** user reloads page
- **THEN** theme preference restored from localStorage

### Requirement: Collapsible sidebar
The system SHALL provide a collapsible sidebar for file navigation.

#### Scenario: Sidebar visible on desktop
- **WHEN** page loads on desktop viewport
- **THEN** sidebar is visible on left side

#### Scenario: Sidebar toggle
- **WHEN** user clicks sidebar toggle button
- **THEN** sidebar collapses/expands

#### Scenario: Sidebar state persistence
- **WHEN** user collapses/expands sidebar
- **THEN** state saved to localStorage
- **AND** restored on page reload

### Requirement: Table of contents
The system SHALL display auto-generated table of contents.

#### Scenario: TOC displayed
- **WHEN** markdown contains headings
- **THEN** TOC shows nested list of headings with anchor links

#### Scenario: TOC smooth scroll
- **WHEN** user clicks TOC item
- **THEN** page smooth scrolls to corresponding heading

#### Scenario: TOC active state
- **WHEN** user scrolls through document
- **THEN** current heading highlighted in TOC

### Requirement: File tree for folder mode
The system SHALL display file tree in folder mode.

#### Scenario: File tree displayed
- **WHEN** user uses --mds mode
- **THEN** sidebar shows recursive file tree with folder/file icons

#### Scenario: File tree navigation
- **WHEN** user clicks file in tree
- **THEN** page loads that markdown file

#### Scenario: Folder expand/collapse
- **WHEN** user clicks folder in tree
- **THEN** folder expands/collapses to show/hide contents

### Requirement: Welcome home page
The system SHALL display Docusaurus-style welcome page in folder mode.

#### Scenario: Home page content
- **WHEN** user visits index in folder mode
- **THEN** page shows welcome message, description, and file list

#### Scenario: Home page file list
- **WHEN** home page displays file list
- **THEN** files are clickable links to their HTML pages

### Requirement: Responsive design
The system SHALL be responsive across desktop and mobile viewports.

#### Scenario: Mobile viewport
- **WHEN** page loads on mobile viewport (< 768px)
- **THEN** sidebar collapses to hamburger menu

#### Scenario: Desktop viewport
- **WHEN** page loads on desktop viewport (>= 768px)
- **THEN** sidebar visible by default
