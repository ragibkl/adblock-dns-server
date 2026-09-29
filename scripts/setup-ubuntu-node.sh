#!/bin/sh
# Set up a DNS node on a fresh Ubuntu 24.04 droplet (for example right after a
# DigitalOcean "rebuild", which keeps the droplet's IPv4 and IPv6 addresses).
#
# Usage, as root on the new node:
#   ./setup-ubuntu-node.sh <node-name> [restore-dir]
#     node-name    short name in server-keys, e.g. sg-dns2 (keytree --name)
#     restore-dir  optional: a directory holding the old node's `env` (its
#                  EXAMPLES/default/.env) and `letsencrypt.tgz` (a tar of the
#                  letsencrypt volume), so DoT/DoH keep their certificate.
#
# Every step is idempotent: if something fails, fix it and run it again.
set -eu
NODE=${1:?usage: $0 <node-name> [restore-dir]}
RESTORE=${2:-}
REPO=https://github.com/ragibkl/adblock-dns-server.git
KEYS=https://raw.githubusercontent.com/ragibkl/server-keys/main/keytree.yaml
say() { echo "== $*"; }

# 1. Port 53 must be free for dnsdist: disable systemd-resolved and use a
#    static resolv.conf (see README, "Disabling systemd-resolve"). On Ubuntu the
#    resolved package can turn /etc/resolv.conf into a stub symlink, which
#    dangles once resolved is off, so always write a real file.
say "resolver"
systemctl disable --now systemd-resolved 2>/dev/null || true
if [ -L /etc/resolv.conf ] || ! grep -q '^nameserver 8.8.8.8' /etc/resolv.conf 2>/dev/null; then
  rm -f /etc/resolv.conf
  printf 'nameserver 8.8.8.8\nnameserver 8.8.4.4\n' > /etc/resolv.conf
fi
getent hosts github.com >/dev/null || { echo "name resolution still broken"; exit 1; }

# 2. Docker from Docker's own repository, with the Compose v2 plugin.
say "docker"
if ! docker compose version >/dev/null 2>&1; then
  export DEBIAN_FRONTEND=noninteractive
  install -m 0755 -d /etc/apt/keyrings
  curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
  chmod a+r /etc/apt/keyrings/docker.asc
  codename=$(. /etc/os-release && echo "$VERSION_CODENAME") # subshell: os-release sets NAME
  echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu $codename stable" \
    > /etc/apt/sources.list.d/docker.list
  apt-get update -qq
  apt-get install -y -qq docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin >/dev/null
fi
docker version --format 'docker {{.Server.Version}}'

# 3. This repo, and the old node's settings and certificates.
say "adblock-dns-server"
[ -d /root/adblock-dns-server/.git ] || git clone -q "$REPO" /root/adblock-dns-server
cd /root/adblock-dns-server/EXAMPLES/default
git pull -q
if [ -n "$RESTORE" ]; then
  [ -f .env ] || install -m 0644 "$RESTORE/env" .env
  if ! docker volume inspect default_letsencrypt >/dev/null 2>&1; then
    # Labels as Compose would set them, so it adopts the volume without a warning.
    docker volume create --label com.docker.compose.project=default \
      --label com.docker.compose.volume=letsencrypt default_letsencrypt >/dev/null
    docker run --rm -i -v default_letsencrypt:/d busybox tar xz -C /d < "$RESTORE/letsencrypt.tgz"
    echo "restored letsencrypt volume"
  fi
fi

# 4. Start the stack (start.sh copies sample.env if there is no .env).
say "start"
./start.sh

# 5. SSH keys.
say "keytree"
if ! grep -qx "name: $NODE" /etc/keytree/config.yaml 2>/dev/null; then
  curl -fsSL https://ragibkl.github.io/keytree/install | sh -s "$KEYS" --name "$NODE"
fi

say "done: $NODE"
docker ps --format '{{.Names}}: {{.Status}}'
