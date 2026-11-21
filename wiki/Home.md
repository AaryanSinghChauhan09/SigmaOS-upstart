# SigmaOS-upstart

SigmaOS integration component for **Upstart init system**.

## Commands

```bash
sigmaos-upstart install   # install package + deploy defaults
sigmaos-upstart deploy    # copy config templates to $HOME
sigmaos-upstart status    # binary/package/config report
sigmaos-upstart reset     # remove deployed config
sigmaos-upstart open      # launch the application
```

## Design

- Shell-based component manager (Omarchy-style per-app repos)
- Secure defaults; no secrets stored
- Deterministic `test.sh` with isolated `$HOME`
