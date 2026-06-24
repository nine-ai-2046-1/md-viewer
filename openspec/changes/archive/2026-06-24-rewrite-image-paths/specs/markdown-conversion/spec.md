## ADDED Requirements

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
