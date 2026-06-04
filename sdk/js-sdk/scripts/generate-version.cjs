const fs = require('fs');
const path = require('path');

const pkgPath = path.resolve(__dirname, '../src/package.json');

const pkg = JSON.parse(fs.readFileSync(pkgPath, 'utf8'));
const version = pkg.version;
const sdkName = pkg.name;

if (typeof version !== 'string' || version.length === 0) {
  throw new Error(`Missing package version in ${pkgPath}`);
}

if (typeof sdkName !== 'string' || sdkName.length === 0) {
  throw new Error(`Missing package name in ${pkgPath}`);
}

const templatePath = path.resolve(__dirname, '../src/core/_version.ts.template');
const outputPath = path.resolve(__dirname, '../src/core/_version.ts');

const template = fs.readFileSync(templatePath, 'utf8');

if (!template.includes('@VERSION@')) {
  throw new Error(`Missing @VERSION@ placeholder in ${templatePath}`);
}

if (!template.includes('@SDK_NAME@')) {
  throw new Error(`Missing @SDK_NAME@ placeholder in ${templatePath}`);
}

const content = template.replaceAll('@VERSION@', version).replaceAll('@SDK_NAME@', sdkName);

fs.writeFileSync(outputPath, content, 'utf8');

console.log(`✅ Generated ${outputPath} (version: ${version})`);
