// helpers.js
// Helper functions for FHEVM test computations

/**
 * Compute the square of a number
 * @param {number} x
 * @returns {number}
 */
function square(x) {
    return x * x;
}

/**
 * Compute the cube of a number
 * @param {number} x
 * @returns {number}
 */
function cube(x) {
    return x * x * x;
}

// Export functions for use in other test scripts
module.exports = { square, cube };
