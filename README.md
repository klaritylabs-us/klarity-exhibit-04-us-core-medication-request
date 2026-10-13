# US Core MedicationRequest (FHIR R4) — Rust

Small library and CLI that sanity-check **US Core MedicationRequest** JSON before it leaves your integration layer: **RxNorm** on `medicationCodeableConcept`, **status**, **intent**, **subject**, and pharmacy exports that ship **NDC only**.

Synthetic fixtures only. Not a drug interaction checker, not ONC certification, not a substitute for the HL7 FHIR Validator or Inferno, and not legal/clinical advice.

Maintained by [Klarity Labs](https://klaritylabs.ai/) as open reference code for U.S. healthcare FHIR interoperability work.

**Exhibit 04** — [FHIR exhibit catalog](https://github.com/klaritylabs-us/klarity-fhir-exhibits) · [klaritylabs-us](https://github.com/klaritylabs-us)


## How this relates to other tools

| Tool class | Role |
|------------|------|
| **HL7 FHIR Validator**, **HAPI**, **Firely** | Full validation, servers, SDKs, IG packages |
| **Inferno**, **Touchstone**, vendor cert suites | **Conformance** against test plans |
| **This exhibit** | **Pre-flight construction** on synthetic fixtures—catch shape mistakes before wire traffic |

Conformance tools test whether your **API behaves**; exhibits focus on whether your **payloads and pipes are built** to survive real partners.

## Specifications cited

- [US Core MedicationRequest](http://hl7.org/fhir/us/core/STU6.1/StructureDefinition-us-core-medicationrequest.html)

## What it checks

- `meta.profile` includes [US Core MedicationRequest STU 6.1](http://hl7.org/fhir/us/core/STU6.1/StructureDefinition-us-core-medicationrequest.html)
- Medication coding includes **RxNorm** (`http://www.nlm.nih.gov/research/umls/rxnorm`) — not NDC-only or display-only
- `status` and `intent` populated (`order` vs `plan` hygiene)
- `subject.reference` present

| Problem | Typical source |
|--------|----------------|
| **NDC without RxNorm** | Pharmacy / eRx pipes map package NDC only |
| **Missing intent** | C-CDA or HL7 v2 med orders converted without intent |
| **plan vs order** | CDS plans mixed with active orders in one bundle |
| **Text-only medication** | Display string with no RxNorm code |

## Run it

Needs [Rust](https://www.rust-lang.org/tools/install).

```bash
cargo test
cargo run --bin validate-medication-request -- examples/valid-order.json
cargo run --bin validate-medication-request -- examples/invalid-ndc-only.json
```

Exit `0` if no errors, `1` if validation failed, `2` for bad usage or JSON.

## Sample output

One command:

```bash
cargo run --bin validate-medication-request -- examples/valid-order.json
```

**Pass** (`examples/valid-order.json`) — exit code `0`:

```json
[]
```

**Fail** (`examples/invalid-missing-intent.json`) — exit code `1` (issues array; more entries may appear):

```json
[
  {
    "severity": "error",
    "path": "MedicationRequest.intent",
    "detail": "intent is required (order, plan, proposal, etc.)",
    "rule_id": "intent-required"
  }
]
```


## Layout

- `src/validate.rs` — rules
- `examples/` — valid and broken payloads for tests and demos
- [docs/architecture.md](./docs/architecture.md) — short pipeline note

## Inferno vs this repo

Inferno exercises your **API** against CMS-style scenarios. This repo is about **payload shape** — medication rows that still fail US Core binding in middleware.

## License

MIT — see [LICENSE](./LICENSE).

**Need this on your payloads or API?** 20-minute FHIR Scan — sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)

## Contact

sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)
