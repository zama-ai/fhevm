
# Initialize and update submodules
git submodule init || { echo >&2 "Failed to initialize submodules."; exit 1; }
git submodule update --remote || { echo >&2 "Failed to update submodules."; exit 1; }
