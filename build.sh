#!/bin/bash
#@Levithenn

BUILD="cargo build"
TARGET="release"


echo "[INFO] This build script was made by Leviathenn, and for the 5run program."


sleep 0.500

echo "[INFO] Starting build Process..."


sleep 0.500

echo "[BUILD] Checking for existing binary..."

sleep 0.600

if [ -f "./5run" ]; then
    echo "[BUILD] Binary Found! Removing binary..."
    sleep 0.500 
    rm -rf "./5run"
    echo "[BUILD] Binary removed. Verifing changes..."
    if [ -f "./5run" ]; then 
        echo "[BUILD] Binary did not successfully remove."
        exit 1
    else
        echo "[BUILD] Binary successfully remove. Changes have been Verified. Continuing Build Process..."
    fi
else
        echo "[BUILD] Existing Binary Not found! Starting build process for the first time..."

fi

echo "[BUILD] Selecting Target Profile..."

sleep 0.500

if [ -n "$1" ]; then
    TARGET="$1" # Set the target
fi

sleep 0.500

echo "[BUILD] Target Profile Selected as ""$TARGET"""

sleep 0.500

echo "[BUILD] Starting build with Profile..."

sleep 0.500

/bin/bash -c "$BUILD --$TARGET"

sleep 0.500

echo "[BUILD] Validing Build..."

sleep 0.500

if [ -f "target/$TARGET/fiverun" ]; then
    echo "[BUILD] Build has been validated. Finishing Build..."
    mv "target/$TARGET/fiverun" "./5run"
else
    echo "[BUILD] Build has not been validated. Please check console for any errors."
    exit 1
fi

sleep 0.500

echo "[BUILD] Editing Permissions for the Binary"

chmod +x ./5run

sleep 0.500

echo "[BUILD] Build Has Completeted Successfully."
