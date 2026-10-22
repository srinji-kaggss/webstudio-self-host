//! Local-only patch for the community builder bundle. See Dockerfile.builder.
//! Every target must match exactly once; a miss exits 1 so upstream drift fails the build.
//!
//! Fix 1: preventCrossOriginCookie requires sec-fetch-site: same-origin; Safari 27
//!        does not send Sec-Fetch-* on form posts, so every builder API POST 403s.
//!        Requests whose Host is localhost/127.0.0.1 pass that origin check only
//!        (the bundle's other same-origin check stays as it is).
//! Fix 2: cookies are Secure and named with the `__Host-` prefix unless
//!        ALLOW_INSECURE_COOKIES=true, which upstream refuses when
//!        DEPLOYMENT_ENVIRONMENT=production (hardcoded by docker-compose.yml).
//!        Browsers reject such cookies over plain http, so on http://localhost
//!        Safari never stores the session and DEV_LOGIN loops to /login.
//!        SECURE_COOKIE follows the DEPLOYMENT_URL scheme instead, and production
//!        still refuses the explicit ALLOW_INSECURE_COOKIES flag.
//! The former Fix 3 (parseBuilderUrl forcing https) is fixed upstream: sourceUrl
//! now inherits the request origin's scheme.

use std::{
    env, fmt, fs,
    io::{self, Write},
    process::ExitCode,
};

const ORIGIN_FROM: &str = "const preventCrossOriginCookie = (request2, throwError = true) => {\n  if (request2.headers.get(\"sec-fetch-site\") === \"same-origin\"";
const ORIGIN_ADD: &str = " || String(request2.headers.get(\"host\")||\"\").indexOf(\"localhost\") !== -1 || String(request2.headers.get(\"host\")||\"\").indexOf(\"127.0.0.1\") !== -1";
const SECURE_FROM: &str = "SECURE_COOKIE: parseResult.data.ALLOW_INSECURE_COOKIES !== \"true\",";
const SECURE_TO: &str = "SECURE_COOKIE: parseResult.data.ALLOW_INSECURE_COOKIES !== \"true\" && String(process.env.DEPLOYMENT_URL || \"\").startsWith(\"https://\"),";
const GUARD_FROM: &str = "if (env$1.SECURE_COOKIE === false) {\n  if (env$1.DEPLOYMENT_ENVIRONMENT === \"production\") {";
const GUARD_TO: &str = "if (env$1.SECURE_COOKIE === false) {\n  if (env$1.DEPLOYMENT_ENVIRONMENT === \"production\" && env$1.ALLOW_INSECURE_COOKIES === \"true\") {";

#[derive(Debug)]
enum PatchError {
    Usage,
    Io(io::Error),
    Target { name: &'static str, found: usize },
}

impl fmt::Display for PatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usage => write!(f, "usage: patch <path/to/build/server/index.js>"),
            Self::Io(e) => write!(f, "io: {e}"),
            Self::Target { name, found } => {
                write!(f, "{name}: target found {found} times, expected 1")
            }
        }
    }
}

impl From<io::Error> for PatchError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

/// Replaces the single occurrence of `from` with `to`; a bundle already holding `to` is left alone.
fn apply(src: &mut String, name: &'static str, from: &str, to: &str) -> Result<bool, PatchError> {
    if src.contains(to) {
        return Ok(false);
    }
    let found = src.matches(from).count();
    if found != 1 {
        return Err(PatchError::Target { name, found });
    }
    *src = src.replacen(from, to, 1);
    Ok(true)
}

fn run() -> Result<Vec<&'static str>, PatchError> {
    let path = env::args().nth(1).ok_or(PatchError::Usage)?;
    let mut src = fs::read_to_string(&path)?;
    let origin_to = format!("{ORIGIN_FROM}{ORIGIN_ADD}");
    let patches = [
        ("origin", ORIGIN_FROM, origin_to.as_str()),
        ("secure-cookie", SECURE_FROM, SECURE_TO),
        ("prod-guard", GUARD_FROM, GUARD_TO),
    ];
    let mut changed = Vec::with_capacity(patches.len());
    for (name, from, to) in patches {
        if apply(&mut src, name, from, to)? {
            changed.push(name);
        }
    }
    fs::write(&path, src)?;
    Ok(changed)
}

/// Writes one line; a failed write turns the exit code into a failure.
fn report(mut out: impl Write, line: &str, code: ExitCode) -> ExitCode {
    match writeln!(out, "{line}") {
        Ok(()) => code,
        Err(_) => ExitCode::FAILURE,
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(changed) if changed.is_empty() => {
            report(io::stdout().lock(), "patched: already-patched", ExitCode::SUCCESS)
        }
        Ok(changed) => report(
            io::stdout().lock(),
            &format!("patched: {}", changed.join(" ")),
            ExitCode::SUCCESS,
        ),
        Err(e) => report(io::stderr().lock(), &e.to_string(), ExitCode::FAILURE),
    }
}
