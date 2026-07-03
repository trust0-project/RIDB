#!/usr/bin/env bash
set -e  # Exit on any error

# Function to test node environment
test_node() {
    wasm-pack --log-level error test --node -- --features node || { echo "wasm-pack test for node failed"; exit 1; }
}

# Ensure the chromedriver used by wasm-pack matches the installed Chrome.
#
# wasm-pack resolves `chromedriver` from PATH and ignores $CHROMEDRIVER. Chrome and
# ChromeDriver must share the same major version, otherwise the driver is killed
# (SIGKILL) immediately after launch and the whole suite fails. If the chromedriver
# already on PATH matches Chrome we keep it; otherwise we download a matching
# Chrome-for-Testing driver into a gitignored cache and prepend it to PATH.
ensure_matching_chromedriver() {
    local chrome_bin="" chrome_major="" driver_major="" install_out driver_dir candidate

    for candidate in \
        "$CHROME" \
        "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" \
        "$(command -v google-chrome 2>/dev/null || true)" \
        "$(command -v google-chrome-stable 2>/dev/null || true)" \
        "$(command -v chromium 2>/dev/null || true)"; do
        if [ -n "$candidate" ] && [ -x "$candidate" ]; then chrome_bin="$candidate"; break; fi
    done

    if [ -z "$chrome_bin" ]; then
        echo "Chrome not found; letting wasm-pack manage chromedriver."
        return 0
    fi

    chrome_major="$("$chrome_bin" --version 2>/dev/null | grep -oE '[0-9]+' | head -1)"
    if [ -z "$chrome_major" ]; then
        echo "Could not determine Chrome version; letting wasm-pack manage chromedriver."
        return 0
    fi

    if command -v chromedriver >/dev/null 2>&1; then
        driver_major="$(chromedriver --version 2>/dev/null | grep -oE '[0-9]+' | head -1)"
        if [ "$driver_major" = "$chrome_major" ]; then
            echo "chromedriver on PATH matches Chrome ${chrome_major}."
            return 0
        fi
        echo "chromedriver (${driver_major}) does not match Chrome (${chrome_major})."
    fi

    echo "Installing chromedriver@${chrome_major} to match Chrome ${chrome_major}..."
    install_out="$(npx -y @puppeteer/browsers install "chromedriver@${chrome_major}" --path "$(pwd)/.chromedriver" 2>/dev/null | tail -1)"
    driver_dir="$(dirname "$(echo "$install_out" | awk '{print $NF}')")"
    if [ -n "$driver_dir" ] && [ -x "$driver_dir/chromedriver" ]; then
        export PATH="$driver_dir:$PATH"
        echo "Using chromedriver at $driver_dir/chromedriver"
    else
        echo "Warning: could not install a matching chromedriver; proceeding with the default on PATH."
    fi
}

# Function to test browser environment
test_browser() {
    ensure_matching_chromedriver
    wasm-pack --log-level info test --headless --chrome -- --lib --features browser || { echo "wasm-pack test for browser failed"; exit 1; }
}

# Initialize variables
ENVIRONMENT=""

# Parse command-line options
while getopts e: option
do
    case "${option}" in
        e) ENVIRONMENT=${OPTARG};;
    esac
done

# Check required options
if [ -z "$ENVIRONMENT" ]; then
    echo "No environment specified. Testing both node and browser environments."
    cargo test
    test_node
    test_browser
    exit 0
fi

# Check which environment to test based on the options provided
case "$ENVIRONMENT" in
    node)
        cargo test
        test_node
        ;;
    browser)
        cargo test
        test_browser
        ;;
    *)
        echo "Error: Unknown environment specified. Please use '-e node' or '-e browser'."
        exit 1
        ;;
esac
