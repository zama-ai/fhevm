// run_tests.js
// Simple example for testing FHEVM environment

function square(x) {
    return x * x;
}

function cube(x) {
    return x * x * x;
}

const numbers = [2, 3, 4, 5];

console.log("Running simple FHEVM computations...\n");

numbers.forEach(n => {
    console.log(Number: ${n}, Square: ${square(n)}, Cube: ${cube(n)});
});

console.log("\nAll computations finished!");
