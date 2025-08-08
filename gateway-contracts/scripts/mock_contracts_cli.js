const fs = require("fs");
const os = require("os");
const { Command } = require("commander");
const { exit } = require("process");
const path = require("path");
const parser = require("@solidity-parser/parser");
const { exec } = require("child_process");

const CONTRACTS_DIR = path.join(__dirname, "../contracts");
const INTERFACES_DIR = path.join(CONTRACTS_DIR, "/interfaces");
const MOCKS_DIR = path.join(CONTRACTS_DIR, "/mocks");
const SHARED_STRUCTS_FILE = "shared/Structs.sol";

// Logging functions
const logInfo = (msg) => console.log(`\x1b[34m[*]\x1b[0m ${msg}`);
const logSuccess = (msg) => console.log(`\x1b[32m[+]\x1b[0m ${msg}`);
const logError = (msg) => console.error(`\x1b[31m[-]\x1b[0m ${msg}`);
const logWarning = (msg) => console.warn(`\x1b[33m[!]\x1b[0m ${msg}`);

// Initialize the CLI
const program = new Command();
program
  .name("mock-contracts-cli")
  .description("A tool to check or update the mock contracts of the Gateway contracts.");

// Add "check" command
program
  .command("check")
  .description("Check if the mock contracts need to be updated.")
  .action(() => checkMocksUpToDate());

// Add "update" command
program
  .command("update")
  .description("Update the mock contracts.")
  .action(() => generateAllMockContracts());

// Parse the CLI arguments
program.parse(process.argv);

/**
 * @description Checks if the mock contracts are up-to-date.
 */
function checkMocksUpToDate() {
  logInfo("Checking if mock contracts are up-to-date...");

  // Create a temporary directory to store newly generated mock contracts
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "mock-contracts"));

  try {
    // Get all existing mock contract files
    const existingMockFiles = fs
      .readdirSync(MOCKS_DIR)
      .filter((file) => file.endsWith(".sol"))
      .map((file) => path.join(MOCKS_DIR, file));

    let outdated = false;

    for (const mockFile of existingMockFiles) {
      const contractName = path.basename(mockFile).replace("Mock", "");
      const contractFilePath = path.join(CONTRACTS_DIR, contractName);
      const interfaceFilePath = path.join(INTERFACES_DIR, `I${contractName}`);

      // Check if the corresponding contract or interface file exists
      if (!fs.existsSync(contractFilePath) || !fs.existsSync(interfaceFilePath)) {
        logError(`Contract or interface file not found for mock: ${mockFile}`);
        exit();
      }

      const contractContent = fs.readFileSync(contractFilePath, "utf8");
      const interfaceContent = fs.readFileSync(interfaceFilePath, "utf8");

      // Generate the mock contract in the temporary directory
      createMockContract(contractContent, interfaceContent, tempDir);

      // Compare the existing mock contract with the newly generated one
      const tempMockFile = path.join(tempDir, path.basename(mockFile));
      const existingContent = fs.readFileSync(mockFile, "utf-8").replace(/\s+/g, "");
      const generatedContent = fs.readFileSync(tempMockFile, "utf-8").replace(/\s+/g, "");

      if (existingContent !== generatedContent) {
        logWarning(`Mock contract ${path.basename(mockFile)} is outdated.`);
        outdated = true;
      }
    }

    if (outdated) {
      logError("ERROR: Some mock contracts are outdated. Run `update` to regenerate them.");
      logInfo("Run `node scripts/mock_contracts_cli.js update` to update the mocks.");
      exit();
    }

    logSuccess("All mock contracts are up-to-date!");
  } finally {
    // Clean up the temporary directory
    fs.rmSync(tempDir, { recursive: true, force: true });
  }
}

/**
 * @description Generates the mock contracts for all .sol files in CONTRACTS_DIR - no recursively.
 */
function generateAllMockContracts() {
  logInfo("Generating mock contracts...");

  if (!fs.existsSync(CONTRACTS_DIR)) {
    logError(`Contracts directory ${CONTRACTS_DIR} does not exist.`);
    exit();
  }

  if (!fs.existsSync(INTERFACES_DIR)) {
    logError(`Interfaces directory ${INTERFACES_DIR} does not exist.`);
    exit();
  }

  const contractFiles = fs.readdirSync(CONTRACTS_DIR, { withFileTypes: true });
  contractFiles.forEach((contractFile) => {
    if (contractFile.isFile() && contractFile.name.endsWith(".sol")) {
      // Read the contract file
      const contractFilePath = path.join(CONTRACTS_DIR, contractFile.name);
      const contractContent = fs.readFileSync(contractFilePath, "utf8");

      // Read the interface file
      const interfaceFilePath = path.join(INTERFACES_DIR, `I${contractFile.name}`);
      const interfaceContent = fs.readFileSync(interfaceFilePath, "utf8");

      createMockContract(contractContent, interfaceContent, MOCKS_DIR);
    }
  });

  // Run Prettier to format the generated mock contracts
  const prettierCommand = `npx prettier ${MOCKS_DIR} --write --log-level silent`;
  exec(prettierCommand, (error, _, stderr) => {
    if (error) {
      logError(error.message);
      return;
    }
    if (stderr) {
      logError(stderr);
      return;
    }
    logSuccess(`Mock generation completed for all contracts and saved to ${MOCKS_DIR} directory.`);
  });
}

/**
 * @description Parses a shared file and extracts its struct definitions
 * @param {string} sharedFilePath - Path to the shared file
 * @returns {BaseASTNode[]} - Array of struct definitions
 */
function parseSharedFile(sharedFilePath) {
  const sharedContent = fs.readFileSync(sharedFilePath, "utf8");
  const parsedShared = parser.parse(sharedContent);
  return parsedShared.children.filter((child) => child.type === "StructDefinition");
}

/**
 * @description Creates a mock contract based on the provided contract and interface contents.
 * @param {string} contractContent - The content of the contract file
 * @param {string} interfaceContent - The content of the interface file
 * @param {string} outputPath - The path to save the generated mock contract
 */
function createMockContract(contractContent, interfaceContent, outputPath) {
  // Parse the contract content and extract its definition
  const parsedContract = parser.parse(contractContent);
  const contractDefinition = parsedContract.children.find((child) => child.type === "ContractDefinition");

  // Parse the interface content and extract its definition
  const parsedInterface = parser.parse(interfaceContent);
  const interfaceDefinition = parsedInterface.children.find((child) => child.type === "ContractDefinition");

  // Parse shared files and extract their definitions
  const sharedStructsDefinitions = parseSharedFile(path.join(CONTRACTS_DIR, SHARED_STRUCTS_FILE));

  // Extract EventDefinitions from the interface definition
  const eventDefinitions = interfaceDefinition.subNodes.filter((node) => node.type === "EventDefinition");

  // Extract StructDefinitions from the interface definition
  const structDefinitions = interfaceDefinition.subNodes.filter((node) => node.type === "StructDefinition");

  // Extract FunctionDefinitions from the contract definition
  const functionDefinitions = contractDefinition.subNodes.filter((node) => node.type === "FunctionDefinition");

  // Generate mock event definitions
  const mockEvents = generateMockEvents(eventDefinitions);

  // Generate mock struct definitions
  const mockStruct = generateMockStructs(structDefinitions);

  // Generate mock counters
  const mockCounters = generateMockCounters(functionDefinitions);

  // Generate mock function definitions
  const mockFunctions = generateMockFunctions(
    functionDefinitions,
    eventDefinitions,
    structDefinitions.concat(sharedStructsDefinitions),
  );

  const spdxLine = "// SPDX-License-Identifier: BSD-3-Clause-Clear";
  const pragmaDirective = parsedContract.children.find((child) => child.type === "PragmaDirective");
  const pragmaLine = `pragma solidity ${pragmaDirective.value};`;

  // Import shared structs if needed
  const importsSharedStructs = parsedInterface.children.some(
    (child) => child.type === "ImportDirective" && child.path.includes(SHARED_STRUCTS_FILE),
  );

  let imports = [];
  if (importsSharedStructs) imports.push(`import "../${SHARED_STRUCTS_FILE}";`);
  const importsLines = imports.join("\n");

  // Build the mock contract
  const contractName = `${contractDefinition.name}Mock`;
  let mockContract = `${spdxLine}\n${pragmaLine}\n${importsLines}\n\ncontract ${contractName} {\n\n`;

  // Append struct lines
  mockContract += mockStruct + "\n\n";
  // Append event lines
  mockContract += mockEvents + "\n\n";
  // Append counter lines
  mockContract += mockCounters + "\n\n";
  // Append function lines
  mockContract += mockFunctions + "\n\n";
  // Close the contract
  mockContract += `}\n`;

  // Check if the output directory exists, if not create it
  if (!fs.existsSync(outputPath)) {
    fs.mkdirSync(outputPath, { recursive: true });
  }

  // Write the mock contract to a file
  fs.writeFileSync(path.join(outputPath, `${contractName}.sol`), mockContract, "utf8");
}

/**
 * @description Generates mock counter definitions based on the provided function definitions.
 * @param {BaseASTNode[]} functionDefinitions - Array of function definitions
 * @returns string - Generated mock counter definitions
 */
function generateMockCounters(functionDefinitions) {
  const counterOperators = findCounterOperators(functionDefinitions);
  return counterOperators.map((counter) => `uint256 ${counter};`).join("\n");
}

/**
 * @description Generates mock structs based on the provided struct definitions.
 * @param {BaseASTNode[]} structDefinitions - Array of structs to generate
 * @returns string - Generated mock structs
 */
function generateMockStructs(structDefinitions) {
  return structDefinitions
    .map((struct) => {
      const structName = struct.name;
      const members = struct.members
        .map((member) => {
          return getParameterType(member.typeName) + ` ${member.name};`;
        })
        .join("\n");

      return `struct ${structName} {\n${members}\n}`;
    })
    .join("\n\n");
}

/**
 * @description Generates mock events based on the provided event definitions.
 * @param {BaseASTNode[]} eventDefinitions - Array of event definitions to generate
 * @returns string - Generated mock events
 */
function generateMockEvents(eventDefinitions) {
  return eventDefinitions
    .map((eventDef) => {
      const eventName = eventDef.name;
      const parameters = eventDef.parameters
        .map((parameter) => {
          const indexed = parameter.isIndexed ? " indexed" : "";
          const parameterType = getParameterType(parameter.typeName);
          return `${parameterType}${indexed} ${parameter.name}`;
        })
        .join(", ");

      return `event ${eventName}(${parameters});`;
    })
    .join("\n\n");
}

/**
 * @description Generates mock functions that emit events based on the provided function definitions.
 * @param {BaseASTNode[]} functionDefinitions - Array of function definitions to generate
 * @param {BaseASTNode[]} eventDefinitions - Array of event definitions
 * @param {BaseASTNode[]} structDefinitions - Array of struct definitions
 * @returns string - Generated mock functions
 */
function generateMockFunctions(functionDefinitions, eventDefinitions, structDefinitions) {
  return functionDefinitions
    .filter(
      (functionDef) =>
        ["public", "external"].includes(functionDef.visibility) &&
        !["view", "pure"].includes(functionDef.stateMutability),
    )
    .map((functionDef) => {
      // Get the function emit statements
      const emitStatements = findEmitStatements(functionDef.body.statements);
      if (emitStatements.length === 0) {
        return;
      }

      // Get the function parameters
      const functionParameters = functionDef.parameters
        .map((parameter) => {
          const location = parameter.storageLocation ? `${parameter.storageLocation} ` : "";
          const parameterType = getParameterType(parameter.typeName);

          // There is a case when the parameter name is commented to silence "unused variable" warnings.
          // In that case, we only set the parameter type and location.
          if (parameter.name === null) {
            parameter.name = "/* unusedVariable */"; // Default name for unused parameters
          }
          return `${parameterType} ${location}${parameter.name}`;
        })
        .join(", ");

      // Get the function ID assignments based on counters
      const counterOperators = findCounterOperators(functionDef.body.statements);
      const idCounterAssignments = findCounterIdAssignments(functionDef.body.statements, counterOperators);

      // Initialize the mock function's header
      let mockFunction = `function ${functionDef.name}(${functionParameters}) ${functionDef.visibility} {\n`;

      // Track declared parameters to avoid duplicates
      const declaredParameters = new Set();
      const allDeclarations = [];

      // Build the mock implementation for each emit statement
      emitStatements.forEach((emitStatement) => {
        const eventName = emitStatement.eventCall.expression.name;
        const eventDefinition = eventDefinitions.find((event) => event.name === eventName);
        const eventArguments = [];

        // Generate declarations only for parameters that haven't been declared yet
        eventDefinition.parameters.forEach((parameter) => {
          const parameterName = parameter.name;
          eventArguments.push(parameterName);

          // Skip the parameter if it is one of the function's input parameters
          const skipDeclaration = functionDef.parameters.some((p) => p.name === parameter.name);
          if (skipDeclaration) return "";

          if (!declaredParameters.has(parameterName)) {
            const parameterType = getParameterType(parameter.typeName);

            let declaration;
            const idCounterAssignment = idCounterAssignments.find((assignment) => assignment.idVar === parameterName);

            // Check if the parameter is a counter ID assignation variable and declare it
            if (idCounterAssignment) {
              declaration = `${idCounterAssignment.counterVar}++;\n${parameterType} ${parameterName} = ${idCounterAssignment.counterVar};`;
              // Else, check if the parameter type is an array and declare it in memory
            } else if (parameterType.endsWith("[]")) {
              declaration = `${parameterType} memory ${parameterName} = new ${parameterType}(1);`;
              // Else, check if the parameter type is a struct, string, or bytes and declare it in memory
            } else if (
              structDefinitions.some((structDef) => structDef.name === parameterType) ||
              ["string", "bytes"].includes(parameterType)
            ) {
              declaration = `${parameterType} memory ${parameterName};`;
              // Else, declare as local type in stack (i.e. uint, bool, address, etc.)
            } else {
              declaration = `${parameterType} ${parameterName};`;
            }

            allDeclarations.push(declaration);
            declaredParameters.add(parameterName);
          }
        });

        // Append the emit statement
        mockFunction += `emit ${eventName}(${eventArguments.join(", ")});\n\n`;
      });

      // Add all declarations at the beginning of the function
      if (allDeclarations.length > 0) {
        mockFunction = mockFunction.replace("{\n", `{\n${allDeclarations.join("\n")}\n\n`);
      }

      // Close the mock function
      mockFunction += `}\n`;

      return mockFunction;
    })
    .filter(Boolean)
    .join("\n\n");
}

/**
 * @description Gets the parameter type from the parameter's TypeName object
 * @param {TypeName} parameterTypeName - The parameter object
 * @returns {string} - The parameter type as a string
 */
function getParameterType(parameterTypeName) {
  switch (parameterTypeName.type) {
    case "ElementaryTypeName":
      return parameterTypeName.name;

    case "UserDefinedTypeName":
      return parameterTypeName.namePath;

    case "ArrayTypeName":
      return getParameterType(parameterTypeName.baseTypeName) + "[]";

    default:
      throw new Error(`Unsupported parameter type: ${parameterTypeName.type}`);
  }
}

/**
 * @description Finds Emit statements in the list of statements
 * @param {BaseASTNode[]} statements
 * @returns {EmitStatement[]} - Array of Emit statements
 */
function findEmitStatements(statements) {
  const emitStatements = [];

  for (const statement of statements) {
    switch (statement.type) {
      case "EmitStatement":
        emitStatements.push(statement);
        break;

      case "IfStatement":
        // Concat inner Emit statements in the If's statements
        emitStatements.push(...findEmitStatements(statement.trueBody?.statements || []));
        break;

      case "Block":
        // Concat inner Emit statements in the Block's statements
        emitStatements.push(...findEmitStatements(statement.statements || []));
        break;

      case "ForStatement":
      case "WhileStatement":
      case "DoWhileStatement":
        // Concat inner Emit statements in the For/While/DoWhile's statements
        emitStatements.push(...findEmitStatements(statement.body?.statements || []));
        break;

      default:
        break;
    }
  }

  return emitStatements;
}

/**
 * @description Finds counter operators in the list of nodes
 * @param {BaseASTNode[]} nodes - AST nodes (e.g., function bodies)
 * @returns string[] - List of counter operator names
 */
function findCounterOperators(nodes) {
  const counterOperators = [];

  for (const node of nodes) {
    if (
      node.type === "UnaryOperation" &&
      node.operator === "++" &&
      node.subExpression &&
      node.subExpression.type === "MemberAccess"
    ) {
      counterOperators.push(node.subExpression.memberName);
    } else {
      // Recursively check all object properties and array elements
      for (const key in node) {
        if (node[key] && typeof node[key] === "object") {
          const nodes = Array.isArray(node[key]) ? node[key] : [node[key]];
          counterOperators.push(...findCounterOperators(nodes));
        }
      }
    }
  }

  // Remove duplicates
  return [...new Set(counterOperators)];
}

/**
 * Finds the ID variables assigned from a counter variable after it's incremented.
 * @param {BaseASTNode[]} nodes - AST nodes (e.g., function bodies)
 * @param {string[]} counterNames - Names of counter variables
 * @returns {{ counterVar: string, idVar: string }[]} - Array of pairs
 */
function findCounterIdAssignments(nodes, counterNames) {
  const counterIdAssignments = [];

  for (const node of nodes) {
    // Check for VariableDeclarationStatement with initialValue from a counter
    if (
      node.type === "VariableDeclarationStatement" &&
      node.variables &&
      node.initialValue &&
      node.initialValue.type === "MemberAccess" &&
      counterNames.includes(node.initialValue.memberName)
    ) {
      // Get the variable name being assigned
      const idVar = node.variables[0]?.name;
      const counterVar = node.initialValue.memberName;
      if (idVar && counterVar) {
        counterIdAssignments.push({ counterVar, idVar });
      }
    }
    // Recursively check all object properties and array elements
    for (const key in node) {
      if (node[key] && typeof node[key] === "object") {
        const nodes = Array.isArray(node[key]) ? node[key] : [node[key]];
        counterIdAssignments.push(...findCounterIdAssignments(nodes, counterNames));
      }
    }
  }

  return counterIdAssignments;
}
