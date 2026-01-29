const fs = require('fs');
const path = process.argv[2];
const oldStr = process.argv[3];
const newStr = process.argv[4];

if (!path || !oldStr || !newStr) {
    console.log("Usage: node replace.js <file> <old> <new>");
    process.exit(1);
}

if (!fs.existsSync(path)) {
    console.log(`File not found: ${path}`);
    // Don't exit with error if file not found, just skip (for qa-ci.yml if it doesn't exist)
    process.exit(0);
}

try {
    let content = fs.readFileSync(path, 'utf8');
    if (content.includes(oldStr)) {
        content = content.split(oldStr).join(newStr);
        fs.writeFileSync(path, content, 'utf8');
        console.log(`Updated ${path}`);
    } else {
        console.log(`String '${oldStr}' not found in ${path}`);
    }
} catch (e) {
    console.error(`Error processing ${path}: ${e}`);
    process.exit(1);
}
