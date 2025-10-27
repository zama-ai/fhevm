require('@rushstack/eslint-patch/modern-module-resolution');

module.exports = {
    root: true,
    extends: ['@layerzerolabs/eslint-config-next/recommended'],
    rules: {
        // @layerzerolabs/eslint-config-next defines rules for turborepo-based projects
        // that are not relevant for this particular project
        'turbo/no-undeclared-env-vars': 'off',
        'import/no-unresolved': 'warn', // Updated to warn to avoid erratic popping up of this particular error for '@layerzerolabs/metadata-tools' even when the package has been installed correctly
    },
};
