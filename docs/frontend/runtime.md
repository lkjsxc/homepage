# Runtime Behavior

- Homepage rendering is portfolio-first and content-driven.
- The `/` page always renders: profile, 5 featured projects, and social links.
- Frontend code validates the fixed five-project showcase at render time.
- Project and social links are rendered as external links.
- No Node process runs in production.
- Async job API helpers remain available for supporting API workflows, not as the primary homepage UI path.
