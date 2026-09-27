#!/bin/sh
set -eu

case "$(uname -m)" in
  x86_64|amd64)
    APPIMAGE_ARCH=x86_64
    ;;
  aarch64|arm64)
    APPIMAGE_ARCH=aarch64
    ;;
  *)
    echo "Unsupported architecture: $(uname -m)" >&2
    exit 1
    ;;
esac

LINUXDEPLOY="linuxdeploy-${APPIMAGE_ARCH}.AppImage"
VERSION="${GITHUB_REF_NAME:-dev}"
export VERSION

echo "Packaging AppImage for ${APPIMAGE_ARCH}"

"./${LINUXDEPLOY}" \
  -dwayvr.desktop \
  -iwayvr.png \
  --appdir="${APPDIR}" \
  --output appimage \
  --exclude-library '*libpipewire*'

mv "WayVR-${VERSION}-${APPIMAGE_ARCH}.AppImage" "WayVR-${APPIMAGE_ARCH}.AppImage"
