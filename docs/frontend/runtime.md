# Runtime Behavior

- Browser UI posts job creation requests.
- Browser polls job snapshots until terminal state.
- UI rendering is event-driven and async.
- No Node process runs in production.
