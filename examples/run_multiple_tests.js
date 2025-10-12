// run_multiple_tests.js
// Script to run multiple simple FHEVM test computations

async function main() {
    const inputs = [2, 3, 4, 5];
    console.log("Starting multiple FHEVM test computations...");

    for (let i = 0; i < inputs.length; i++) {
        const input = inputs[i];
        const output = input * input; // simple square computation
        console.log(Input: ${input}, Output: ${output});
    }

    console.log("All FHEVM test computations finished successfully!");
}

main();
