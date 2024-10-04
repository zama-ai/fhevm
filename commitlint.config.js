module.exports = {
    extends: ['@commitlint/config-conventional'],
    rules: {
      'header-max-length': [2, 'always', 100], // Enforce max length of commit message header
      'type-empty': [2, 'never'],              // Type must not be empty
      'subject-empty': [2, 'never'],           // Subject must not be empty
    },
  };
  