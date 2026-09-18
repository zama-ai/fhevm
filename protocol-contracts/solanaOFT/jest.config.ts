/** @type {import('ts-jest').JestConfigWithTsJest} */
module.exports = {
    reporters: [['github-actions', { silent: false }], 'default'],
    testEnvironment: 'node',
    testTimeout: 15000,
    moduleNameMapper: {
        '^@/(.*)$': '<rootDir>/src/$1',
    },
    transform: {
        '^.+\\.(t|j)sx?$': '@swc/jest',
    },
}
