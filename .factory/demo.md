# Mount Identity Audit demo sandbox

## Browser sample

- URL: `https://devcontainer-identity-audit.sociobot.in/demo/`
- Local URL: `http://127.0.0.1:4173/demo/` after `npm run build:site && npm run preview`
- Entry point: select **Try it with sample data** on the home page.
- Sample: workspace owner `1000:1000`, mode `0755`, remote user `1000:1000`, and rootless Podman's default subuid map.
- Expected first result: `FAIL`; the process maps to host `100999:100999` and cannot write the workspace.
- Reset: select **Reset demo** in the persistent banner.
- Exit: select **Start for real** to return to the normal calculator.
- Storage namespace: none. State exists only in the page DOM. Demo mode never reads or writes real browser storage.
- Offline: visit once, wait for the service worker, then reload `/demo/` offline.

## CLI sample

```sh
mount-identity-audit --demo
```

The shipped input is `examples/mismatch/`. The command copies it to
`$TMPDIR/mount-identity-audit-demo-<pid>-<nonce>/`, audits that copy with direct
Docker mapping, and prints the copy path. It does not read or write the current
project. The sample intentionally returns `FAIL` and exit code `1`.

Run every registered sandbox check with:

```sh
npm run test:claims
```
