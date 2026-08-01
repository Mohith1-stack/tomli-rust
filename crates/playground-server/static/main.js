import init, { parse_toml } from './pkg/tomli_wasm.js';

const tomlInput = document.getElementById('toml-input');
const jsonOutput = document.getElementById('json-output');
const statusBadge = document.getElementById('status-badge');
const runBtn = document.getElementById('run-btn');

let wasmLoaded = false;
let lastStatus = '';

async function loadWasm() {
    try {
        await init();
        wasmLoaded = true;
        
        // Initial parse
        handleInput();
        
        // Add event listener to the RUN button
        runBtn.addEventListener('click', handleInput);
        
        // Add a nice visual effect to the button when clicked
        runBtn.addEventListener('click', () => {
            runBtn.style.transform = 'scale(0.95)';
            setTimeout(() => runBtn.style.transform = '', 100);
        });

    } catch (e) {
        console.error("Failed to load WASM:", e);
        jsonOutput.innerHTML = `<span style="color:var(--error-color)">Failed to load WebAssembly module.</span>`;
        updateBadge("Error", "error");
    }
}

function updateBadge(text, className) {
    if (lastStatus !== text) {
        statusBadge.textContent = text;
        // Force reflow to re-trigger animation
        statusBadge.className = 'badge';
        void statusBadge.offsetWidth;
        statusBadge.className = `badge ${className}`;
        lastStatus = text;
    } else {
        // If it's the same status, still pop the badge to show it ran
        statusBadge.className = 'badge';
        void statusBadge.offsetWidth;
        statusBadge.className = `badge ${className}`;
    }
}

// Basic JSON syntax highlighter
function syntaxHighlight(jsonObj) {
    let json = JSON.stringify(jsonObj, null, 2);
    json = json.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
    return json.replace(/("(\\u[a-zA-Z0-9]{4}|\\[^u]|[^\\"])*"(\s*:)?|\b(true|false|null)\b|-?\d+(?:\.\d*)?(?:[eE][+\-]?\d+)?)/g, function (match) {
        let cls = 'json-number';
        if (/^"/.test(match)) {
            if (/:$/.test(match)) {
                cls = 'json-key';
            } else {
                cls = 'json-string';
            }
        } else if (/true|false/.test(match)) {
            cls = 'json-boolean';
        } else if (/null/.test(match)) {
            cls = 'json-null';
        }
        return '<span class="' + cls + '">' + match + '</span>';
    });
}

function handleInput() {
    if (!wasmLoaded) return;
    
    const text = tomlInput.value;
    
    try {
        const resultStr = parse_toml(text);
        const result = JSON.parse(resultStr);
        
        if (result.error) {
            jsonOutput.innerHTML = `<span style="color:var(--error-color)">${result.error}</span>`;
            updateBadge("Invalid TOML", "error");
        } else {
            jsonOutput.innerHTML = syntaxHighlight(result);
            updateBadge("Valid", "success");
        }
    } catch (e) {
        jsonOutput.innerHTML = `<span style="color:var(--error-color)">Fatal error parsing TOML:\n${e.toString()}</span>`;
        updateBadge("Crash", "error");
    }
}

loadWasm();
