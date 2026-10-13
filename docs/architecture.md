# Architecture

```text
MedicationRequest.json
  → meta.profile (us-core-medicationrequest)
  → status, intent
  → medicationCodeableConcept (RxNorm; flag NDC-only)
  → subject.reference
  → status/intent combination warnings
```

`medicationReference` is accepted without RxNorm checks in this small validator.
