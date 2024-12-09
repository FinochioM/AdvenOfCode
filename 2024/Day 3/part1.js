const fs = require('fs');

function readInput(filename) {
    try {
        return fs.readFileSync(filename, 'utf8');
    } catch (err) {
        console.error('Error reading file:', err);
        process.exit(1);
    }
}

function findValidMultiplications(input) {
    const mulRegex = /mul\((\d{1,3}),(\d{1,3})\)/g;

    const matches = [];
    let match;

    while ((match = mulRegex.exec(input)) !== null) {
        matches.push({
            x: parseInt(match[1]),
            y: parseInt(match[2])
        });
    }

    return matches;
}

function calculateSum(multiplications) {
    return multiplications.reduce((sum, { x, y }) => sum + (x * y), 0);
}

function main() {
    const input = readInput('Day 3/data.txt');

    const multiplications = findValidMultiplications(input);

    const result = calculateSum(multiplications);
    console.log('The sum of all multiplication results is:', result);
}

main();