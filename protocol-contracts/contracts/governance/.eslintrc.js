require('@rushstack/eslint-patch/modern-module-resolution');

module.exports = {
    root: true,
    extends: ['@layerzerolabs/eslint-config-next/recommended'],
    rules: {
        // @layerzerolabs/eslint-config-next defines rules for turborepo-based projects
        // that are not relevant for this particular project
        'turbo/no-undeclared-env-vars': 'off',
        'import/no-unresolved': 'warn', // Updated to warn to as this was failing (only) CI even when @layerzerolabs/metadata-tools has been installed correctly
    },
};
