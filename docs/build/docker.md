# Dockerfile Stages

1. `frontend-build`: install npm deps and transpile TypeScript.
2. `backend-build`: compile Rust binary and run tests.
3. `runtime`: copy Rust binary and frontend assets only.

The runtime stage excludes compilers and package managers.
