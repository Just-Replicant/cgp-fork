# `FinalizeExtract`

```text
`MatchRemainder` still has a variant left: not every variant of the enum is handled
```

`MatchWith` walks an extensible enum. A partial enum marks each variant
`IsPresent` or `IsVoid`. `FinalizeExtract` is implemented for the remainder
only when every variant is `IsVoid`. A leftover `IsPresent` variant means the
handler list skipped it.

Give each present variant a handler. The order of handlers follows the order of
the variants the extractor still contains.
