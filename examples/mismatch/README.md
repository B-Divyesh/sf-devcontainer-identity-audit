# Bundled mismatch sample

This sample represents a workspace owned by host user `0:0` with mode `0755`.
The configured container process uses `1000:1000`, so direct Docker mapping can
read the directory but cannot write it.
