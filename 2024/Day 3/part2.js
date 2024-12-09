const fs = require('fs');
const path = require('path');

const currentDir = path.dirname(__filename);

function readInput(filename) {
    try {
        const filePath = path.join(currentDir, filename);
        return fs.readFileSync(filePath, 'utf8');
    } catch (err) {
        console.error('Error reading file:', err);
        process.exit(1);
    }
}

function processInstructions(input) {
    const instructions = [];

    const mulRegex = /mul\((\d{1,3}),(\d{1,3})\)/g;
    let mulMatch;
    while ((mulMatch = mulRegex.exec(input)) !== null) {
        instructions.push({
            type: 'mul',
            position: mulMatch.index,
            x: parseInt(mulMatch[1]),
            y: parseInt(mulMatch[2])
        });
    }

    const doRegex = /do\(\)/g;
    const dontRegex = /don't\(\)/g;

    let doMatch;
    while ((doMatch = doRegex.exec(input)) !== null) {
        instructions.push({
            type: 'do',
            position: doMatch.index
        });
    }

    let dontMatch;
    while ((dontMatch = dontRegex.exec(input)) !== null) {
        instructions.push({
            type: 'dont',
            position: dontMatch.index
        });
    }

    return instructions.sort((a, b) => a.position - b.position);
}

function calculateSum(instructions) {
    let sum = 0;
    let isEnabled = true;

    for (const instruction of instructions) {
        switch (instruction.type) {
            case 'mul':
                if (isEnabled) {
                    sum += instruction.x * instruction.y;
                }
                break;
            case 'do':
                isEnabled = true;
                break;
            case 'dont':
                isEnabled = false;
                break;
        }
    }

    return sum;
}

function main() {
    const input = readInput('data.txt');
    const instructions = processInstructions(input);

    const result = calculateSum(instructions);
    console.log('The sum of all enabled multiplication results is:', result);
}

main();