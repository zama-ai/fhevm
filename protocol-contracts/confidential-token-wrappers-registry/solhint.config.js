const rules = [
  'avoid-tx-origin',
  'const-name-snakecase',
  'contract-name-capwords',
  'event-name-capwords',
  'max-states-count',
  'explicit-types',
  'func-name-mixedcase',
  'func-param-name-mixedcase',
  'imports-on-top',
  'imports-order',
  'modifier-name-mixedcase',
  'no-console',
  'no-global-import',
  'no-unused-vars',
  'quotes',
  'use-forbidden-name',
  'var-name-mixedcase',
  'visibility-modifier-order',
  'interface-starts-with-i',
  'duplicated-imports',
];

module.exports = {
  rules: Object.fromEntries(rules.map(r => [r, 'error'])),
};
