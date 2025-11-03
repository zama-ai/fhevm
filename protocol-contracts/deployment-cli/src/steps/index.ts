import type { DeploymentStep } from "./base-step.js";
import { Step01AragonDao } from "./step-01-aragon-dao.js";
import { Step02Safe } from "./step-02-safe.js";
import { Step03LayerzeroLink } from "./step-03-layerzero.js";
import { Step04TokenDeployment } from "./step-04-token.js";
import { Step05FeesBurner } from "./step-05-fees-burner.js";
import { Step06GatewayContracts } from "./step-06-gateway-contracts.js";
import { Step07HostContracts } from "./step-07-host-contracts.js";
import { Step08PauserSetWrapper } from "./step-08-pauser-set-wrapper.js";
import { Step09GatewayOwnership } from "./step-09-gateway-ownership.js";
import { Step10HostOwnership } from "./step-10-host-ownership.js";

export function getDeploymentSteps(): DeploymentStep[] {
    return [
        new Step01AragonDao(),
        new Step02Safe(),
        new Step03LayerzeroLink(),
        new Step04TokenDeployment(),
        new Step05FeesBurner(),
        new Step06GatewayContracts(),
        new Step07HostContracts(),
        new Step08PauserSetWrapper(),
        new Step09GatewayOwnership(),
        new Step10HostOwnership(),
    ];
}
