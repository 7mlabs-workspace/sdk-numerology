# 7mlabs Numerology SDK

Offline Pythagorean calculations from one Rust core, with Node.js/TypeScript, Python, .NET and C interfaces. MIT License — Copyright (c) 2026 7mlabs.

SDK version: 1.0.0. Registry availability and verified native targets are recorded in the release manifest. This is an independent SDK with a new JSON contract, not a drop-in replacement for the earlier HTTP service. It performs calculations directly inside the caller process, without an engine server or an Internet request.

## Calculate

```json
{"operation":"profile","system":"pythagorean","name":{"given":"An","middle":"Văn","family":"Nguyễn"},"birthDate":"2000-02-29","referenceDate":"2026-11-03"}
```

Name components are explicit so callers control cultural name order. `profile` returns name numbers, birth numbers, cycles, Essence and derived values. Use `name`, `birth`, `cycles` or `essence` independently to avoid unnecessary calculation and payload. Only `pythagorean` is implemented in this release; another system returns a structured error.

### Node.js / TypeScript

```js
const { calculate } = require('@7mlabs/numerology');
const result = calculate({ operation: 'birth', birthDate: '2000-02-29' });
console.log(result.data.birth.lifePath);
```

### Python

```python
from sevenmlabs_numerology import calculate
result = calculate({"operation": "birth", "birthDate": "2000-02-29"})
print(result["data"]["birth"]["lifePath"])
```

### .NET

```csharp
using SevenMLabs.Numerology;
var result = Numerology.Calculate("""{"operation":"birth","birthDate":"2000-02-29"}""");
Console.WriteLine(result.GetProperty("data").GetProperty("birth").GetProperty("lifePath"));
```

All wrappers expose a raw JSON envelope (`calculateJson`, `calculate_json`, `CalculateJson`), returning `ok` and either `result` or structured `error`. Convenience methods return the result and throw an error with a code/field on calculation failure. Success metadata includes engine/schema/rules versions and selected options. Inputs are not mutated. Results use the explicit date supplied by the caller and are deterministic across bindings. See [the complete rules](https://github.com/7mlabs-workspace/sdk-numerology/blob/main/docs/rules.md) before comparing calculators using different conventions.

## Build and verify

Rust 1.83+, Node.js 18+, Python 3.10+, .NET 8+ library target. Native binary availability is target-specific: release manifests enumerate targets actually built and tested; no Windows, WebAssembly, musl or other untested binary support is implied.

```sh
cargo test --workspace --locked
python3 scripts/collect-notices.py
python3 scripts/build-native.py
python3 scripts/make-conformance.py
node scripts/test-node.cjs
python3 scripts/test-python.py
dotnet run --project tests/dotnet/Consumer.csproj -- artifacts/conformance.json
```

`crates/` holds the Rust engine, C ABI, Node adapter and CLI; `bindings/` holds language wrappers; `include/` defines the C ABI. The private local legacy snapshot is excluded from version control and packages. Third-party notices ship with native packages. Native calls require valid C pointers as documented in the header; use the managed wrappers for ordinary input validation.
