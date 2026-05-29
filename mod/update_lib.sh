#!/usr/bin/env bash
set -euo pipefail

GAME_DIR="${1:-}"

if [ -z "$GAME_DIR" ]; then
    echo "Usage: ./update_lib.sh \"/path/to/Spaceflight Simulator Game\""
    echo ""
    echo "Default Steam paths:"
    echo '  Windows: "C:/Program Files (x86)/Steam/steamapps/common/Spaceflight Simulator Game"'
    echo '  Windows: "D:/SteamLibrary/steamapps/common/Spaceflight Simulator Game"'
    exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
LIB_DIR="$SCRIPT_DIR/lib"

MANAGED="$GAME_DIR/Spaceflight Simulator_Data/Managed"
BEPINEX_CORE="$GAME_DIR/BepInEx/core"

if [ ! -d "$MANAGED" ]; then
    echo "ERROR: Managed/ not found at $MANAGED"
    echo "Is this the correct game directory?"
    exit 1
fi

mkdir -p "$LIB_DIR"

echo "Copying from Managed/..."
cp "$MANAGED/Assembly-CSharp.dll" "$LIB_DIR/"
cp "$MANAGED/Unity.TextMeshPro.dll" "$LIB_DIR/"
cp "$MANAGED/UnityEngine.CoreModule.dll" "$LIB_DIR/"
cp "$MANAGED/UnityEngine.dll" "$LIB_DIR/"
cp "$MANAGED/UnityEngine.UI.dll" "$LIB_DIR/"
cp "$MANAGED/UnityEngine.TextRenderingModule.dll" "$LIB_DIR/"
cp "$MANAGED/UnityEngine.TextCoreFontEngineModule.dll" "$LIB_DIR/"
cp "$MANAGED/UnityEngine.Physics2DModule.dll" "$LIB_DIR/"

if [ -d "$BEPINEX_CORE" ]; then
    echo "Copying from BepInEx/core/..."
    cp "$BEPINEX_CORE/0Harmony.dll" "$LIB_DIR/"
    cp "$BEPINEX_CORE/BepInEx.dll" "$LIB_DIR/"
    cp "$BEPINEX_CORE/BepInEx.Harmony.dll" "$LIB_DIR/"
else
    echo "NOTE: BepInEx/core/ not found at $BEPINEX_CORE"
    echo "Some DLLs (BepInEx.dll, BepInEx.Harmony.dll) may already be in Managed/"
    if [ -f "$MANAGED/0Harmony.dll" ]; then
        cp "$MANAGED/0Harmony.dll" "$LIB_DIR/"
        echo "Copied 0Harmony.dll from Managed/"
    fi
fi

echo ""
echo "Done. DLLs copied to $LIB_DIR"
echo "Now run: dotnet build mod/SFS_HAN_MOD.csproj -c Release"
echo "         dotnet build mod/SFS_FontFix.csproj -c Release"
