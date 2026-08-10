# Local-only patch layer on the community builder image.
# The builder's preventCrossOriginCookie middleware requires a `sec-fetch-site:
# same-origin` header (or a navigate GET) on every request. Safari 27 on macOS
# does not send Sec-Fetch-* headers for form posts, which made the DEV_LOGIN
# POST to /auth/dev and every subsequent builder API call fail with 403 on a
# localhost deployment. This layer allows requests whose Host is localhost /
# 127.0.0.1 through the origin check. It changes nothing for remote hosts.
FROM ghcr.io/webstudio-community/builder:latest
COPY patch.js /patch.js
RUN node /patch.js
