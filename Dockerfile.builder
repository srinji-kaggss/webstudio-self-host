# Local-only patch layer on the community builder image (see patch.rs).
# The builder's preventCrossOriginCookie middleware requires a `sec-fetch-site:
# same-origin` header (or a navigate GET) on every request. Safari 27 on macOS
# does not send Sec-Fetch-* headers for form posts, which made the DEV_LOGIN
# POST to /auth/dev and every subsequent builder API call fail with 403 on a
# localhost deployment, and its Secure `__Host-` cookies never stick over http.
# patch.rs fixes both for localhost / http deployments and changes nothing for
# https ones. It runs in a throwaway Rust stage; only the patched bundle ships.
FROM ghcr.io/webstudio-community/builder:latest AS upstream

FROM rust:1.99-trixie AS patch
COPY patch.rs /patch.rs
COPY --from=upstream /app/build/server/index.js /index.js
RUN rustc --edition 2024 -O -o /patch /patch.rs && /patch /index.js

FROM upstream
COPY --from=patch --chown=remix:nodejs /index.js /app/build/server/index.js
