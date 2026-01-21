import * as fs from 'fs';
import * as path from 'path';

interface InputItem {
  point: {
    eid: number;
    address: string;
  };
  data: string;
  description: string;
}

interface AragonAction {
  to: string;
  value: number;
  data: string;
}

function convertToAragonProposal(inputPath: string, outputPath: string): void {
  // Read input JSON
  const inputData: InputItem[] = JSON.parse(fs.readFileSync(inputPath, 'utf-8'));

  // Transform to Aragon proposal format
  const aragonProposal: AragonAction[] = inputData.map((item) => ({
    to: item.point.address,
    value: 0,
    data: item.data,
  }));

  // Write output JSON
  fs.writeFileSync(outputPath, JSON.stringify(aragonProposal, null, 2));

  console.log(`✅ Converted ${inputData.length} actions to Aragon proposal format`);
  console.log(`📄 Output saved to: ${outputPath}`);
}

// Main execution
const inputFile = process.argv[2] || 'output.json';
const outputFile = process.argv[3] || 'aragonProposal.json';

const inputPath = path.resolve(process.cwd(), inputFile);
const outputPath = path.resolve(process.cwd(), outputFile);

if (!fs.existsSync(inputPath)) {
  console.error(`❌ Input file not found: ${inputPath}`);
  process.exit(1);
}

convertToAragonProposal(inputPath, outputPath);
