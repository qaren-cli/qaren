# Automation and CI/CD

Qaren is built from the ground up to be a native citizen in modern CI/CD pipelines. It utilizes deterministic exit codes to signal comparison states, making it incredibly simple to block pipelines, trigger alerts, or automate remediations when configuration drift is detected.

## Exit Code Semantics

Qaren adheres to a strict, predictable exit code policy designed for bash and pipeline integration:

| Exit Code | Meaning | Pipeline Action |
|-----------|---------|-----------------|
| `0` | No Differences | Pipeline continues smoothly. Configurations match. |
| `1` | Differences Found | Configuration drift detected. Pipeline should typically fail or require manual review. |
| `2` | Execution Error | Fatal error (e.g., I/O error, missing files, permission denied). |

## Bash Automation Example

You can easily wrap Qaren in shell scripts for automated environment checks.

```bash
#!/bin/bash

# Check if staging is missing keys that exist in production
qaren kv staging.env production.env --missing-only

if [ $? -eq 1 ]; then
    echo "CRITICAL: Staging is missing required keys from Production!"
    exit 1
elif [ $? -eq 2 ]; then
    echo "ERROR: Failed to run configuration check."
    exit 2
else
    echo "OK: Configurations are structurally sound."
fi
```

## GitHub Actions Example

Integrate Qaren directly into your GitHub Actions workflow to block PRs if `.env.example` drifts from your actual required configurations.

```yaml
name: Configuration Drift Detection

on:
  pull_request:
    branches: [ "main" ]

jobs:
  verify-config:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Qaren
        run: |
          curl -LO https://github.com/qaren-cli/qaren/releases/latest/download/qaren-linux-amd64.tar.gz
          tar -xzf qaren-linux-amd64.tar.gz
          sudo mv qaren /usr/local/bin/
          
      - name: Check KV Config Drift
        run: qaren kv .env.example .env.staging
```

## Secret Masking in CI

When running in CI environments, logs are often shipped to third-party aggregators. Qaren's built-in security features automatically mask detected secrets.

```bash
qaren kv staging.env production.env
# Output will highlight structural changes but replace values with ***
```

---
[Return to Index](../README.md) | [See Config Management](../commands/config.md)
