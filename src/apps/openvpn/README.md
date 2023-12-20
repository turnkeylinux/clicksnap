# Envvars

`TKL_OPENVPN_PROFILE_URL`: the URL of the client profile QR code page. Must be defined for the test to work.

You can get a suitable value by:
- executing `aux.sh` in the test container via `docker cp` / `docker exec` or `podman cp` / `podman exec`
- `scp` and run for VMs
- copy and paste to a live `ssh` session to the OpenVPN appliance
