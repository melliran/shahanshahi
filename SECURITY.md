# Security

## Supported versions

Security fixes target the **latest published release** on [crates.io/crates/shahanshahi](https://crates.io/crates/shahanshahi) once publishing is active, and the **`main`** branch at the tip of this repository.

The crate is **pre-1.0**; see [`docs/ENGINEERING.md`](./docs/ENGINEERING.md) for versioning expectations. Backports to older release lines happen when maintainers can reasonably do so.

## Reporting a vulnerability

**Do not** open a **public** issue for an undisclosed security problem. That can put users at risk before a fix exists.

### How to report

1. Use GitHub **Security → [Report a vulnerability](https://github.com/melliran/shahanshahi/security)** so the report stays **private** to maintainers.  
   If you do not see that control, ask maintainers to enable [private vulnerability reporting](https://docs.github.com/en/code-security/security-advisories/guidance-on-reporting-and-writing/configuring-private-vulnerability-reporting-for-a-repository) for this repository, or coordinate over an agreed private channel.

2. Include, when possible:

   - Affected **version**, **tag**, or **commit**
   - **Impact** (what could go wrong for a dependent)
   - **Steps to reproduce** or a minimal proof of concept

### What to expect

- **Acknowledgement** within about **7 business days** (sooner when feasible).
- A **coordinated disclosure** plan: fix, release or advisory, then public detail when appropriate.

## Scope

**In scope** for this policy: issues in **this repository’s code** (the `shahanshahi` crate), its **build/publish pipeline** as configured here, and **dependency** vulnerabilities in the crate’s **published** dependency graph when used as documented.

**Out of scope** for a private security report: general calendar or spec discussion without a credible security impact — use regular [issues](https://github.com/melliran/shahanshahi/issues) and [`SPEC.md`](./SPEC.md) instead.

## Safe harbor

We welcome **good-faith** research that follows this process. Do not access third-party data or systems without permission, and do not perform disruptive testing against infrastructure you do not own.
