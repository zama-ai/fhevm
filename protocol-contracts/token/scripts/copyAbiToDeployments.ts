import * as fs from 'fs';
import * as path from 'path';

const ARTIFACTS_DIR = path.resolve(__dirname, '../artifacts/contracts');
const DEPLOYMENTS_DIR = path.resolve(__dirname, '../deployments');

interface ArtifactJson {
  abi: unknown[];
  [key: string]: unknown;
}

interface DeploymentJson {
  address: string;
  abi?: unknown[];
  [key: string]: unknown;
}

function getArtifactPath(contractName: string): string {
  return path.join(ARTIFACTS_DIR, `${contractName}.sol`, `${contractName}.json`);
}

function loadJson<T>(filePath: string): T {
  const content = fs.readFileSync(filePath, 'utf-8');
  return JSON.parse(content) as T;
}

function saveJson(filePath: string, data: unknown): void {
  fs.writeFileSync(filePath, JSON.stringify(data, null, 2) + '\n');
}

function isDeploymentFile(fileName: string): boolean {
  // Exclude non-deployment files like .chainId
  return fileName.endsWith('.json') && !fileName.startsWith('.');
}

function copyAbiToDeployments(): void {
  // Get all network directories
  const networks = fs.readdirSync(DEPLOYMENTS_DIR).filter((name) => {
    const fullPath = path.join(DEPLOYMENTS_DIR, name);
    return fs.statSync(fullPath).isDirectory();
  });

  console.log(`Found ${networks.length} network(s): ${networks.join(', ')}\n`);

  let updatedCount = 0;
  let skippedCount = 0;
  let errorCount = 0;

  for (const network of networks) {
    const networkDir = path.join(DEPLOYMENTS_DIR, network);
    console.log(`Processing network: ${network}`);

    // Get all deployment JSON files (excluding solcInputs folder and non-json files)
    const files = fs.readdirSync(networkDir).filter((file) => {
      const fullPath = path.join(networkDir, file);
      return fs.statSync(fullPath).isFile() && isDeploymentFile(file);
    });

    for (const file of files) {
      const contractName = path.basename(file, '.json');
      const deploymentPath = path.join(networkDir, file);
      const artifactPath = getArtifactPath(contractName);

      // Check if artifact exists
      if (!fs.existsSync(artifactPath)) {
        console.log(`  ⚠️  Skipped ${contractName}: artifact not found at ${artifactPath}`);
        skippedCount++;
        continue;
      }

      try {
        // Load artifact and deployment
        const artifact = loadJson<ArtifactJson>(artifactPath);
        const deployment = loadJson<DeploymentJson>(deploymentPath);

        if (!artifact.abi) {
          console.log(`  ⚠️  Skipped ${contractName}: no ABI in artifact`);
          skippedCount++;
          continue;
        }

        // Copy ABI to deployment
        deployment.abi = artifact.abi;

        // Save updated deployment
        saveJson(deploymentPath, deployment);
        console.log(`  ✅ Updated ${contractName}`);
        updatedCount++;
      } catch (error) {
        console.log(`  ❌ Error processing ${contractName}: ${error}`);
        errorCount++;
      }
    }

    console.log('');
  }

  console.log('='.repeat(50));
  console.log(`Summary: ${updatedCount} updated, ${skippedCount} skipped, ${errorCount} errors`);
}

// Main execution
copyAbiToDeployments();
