// Local-only patch for the community builder image. See Dockerfile.builder.
// Fix 1: preventCrossOriginCookie requires sec-fetch-site: same-origin; Safari 27
//        does not send Sec-Fetch-* on form posts -> every builder API POST 403s.
//        Allow requests whose Host is localhost/127.0.0.1 through the origin check.
// Fix 2: cookies are hardcoded `secure: true` AND named with the `__Host-` prefix,
//        which RFC-compliant browsers reject unless the connection is https.
//        On plain http://localhost Safari therefore never stores the session and a
//        successful DEV_LOGIN redirects back to /login. When DEPLOYMENT_URL is http,
//        drop Secure and the `__Host-` prefix; keep them for https deployments.
const fs = require("fs");
const p = "/app/build/server/index.js";
let s = fs.readFileSync(p, "utf8");
let changed = "";

// Fix 1
const t1 = 'request2.headers.get("sec-fetch-site") === "same-origin"';
const add1 = ' || String(request2.headers.get("host")||"").indexOf("localhost") !== -1 || String(request2.headers.get("host")||"").indexOf("127.0.0.1") !== -1';
if (!s.includes(t1)) { console.error("target 1 missing"); process.exit(1); }
if (!s.includes(t1 + add1)) { s = s.split(t1).join(t1 + add1); changed += " origin:" + (s.split(t1).length - 1); }

// Fix 2a: secure flag follows https
const t2 = "secure: true";
const add2 = 'secure: process.env.DEPLOYMENT_URL ? process.env.DEPLOYMENT_URL.startsWith("https://") : false';
if (!s.includes(add2)) { s = s.split(t2).join(add2); changed += " secure-flag:" + (s.split(t2).length - 1); }

// Fix 2b: __Host- prefix only when https
const pref = '(process.env.DEPLOYMENT_URL&&process.env.DEPLOYMENT_URL.startsWith("https://")?"__Host-":"")';
const reps = [
  ["`__Host-_session_${getSessionCookieNameVersion()}`",
   pref + "+`_session_${getSessionCookieNameVersion()}`"],
  ["`__Host-_session_builder_session_${getSessionCookieNameVersion()}`",
   pref + "+`_session_builder_session_${getSessionCookieNameVersion()}`"],
  ["`__Host-_csrf_${getCsrfSessionCookieNameVersion()}`",
   pref + "+`_csrf_${getCsrfSessionCookieNameVersion()}`"],
  ['createCookie("__Host-_returnTo"',
   'createCookie(' + pref + '+"_returnTo"']
];
for (const [from, to] of reps) {
  const n = s.split(from).length - 1;
  if (n === 0) { console.error("target missing: " + from.slice(0, 40)); process.exit(1); }
  if (!s.includes(to)) { s = s.split(from).join(to); changed += " prefix:" + n; }
}


// Fix 3: parseBuilderUrl hardcodes the source origin to https (upstream assumes
//        production/preview domains are always TLS). On plain http://localhost
//        that makes the OAuth authorize endpoint https://localhost:3000/... which
//        no browser can open -> "Failed to open page" after project creation.
//        Inherit the scheme from the incoming request instead (http stays http,
//        real https deployments keep https).
const t3 = 'sourceUrl.protocol = "https";';
const add3 = "sourceUrl.protocol = url2.protocol;";
if (!s.includes(t3)) { console.error("target 3 missing"); process.exit(1); }
if (!s.includes(add3)) { s = s.split(t3).join(add3); changed += " scheme-inherit:1"; }

fs.writeFileSync(p, s);
console.log("patched:" + (changed || " already-patched"));
