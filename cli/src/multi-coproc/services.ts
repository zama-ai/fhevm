import {
  COPROCESSOR_TEMPLATES,
  getServicesByGroup,
  type ServiceDefinition,
} from "../config/service-map";
import { DOCKER_PROJECT } from "../docker/types";

import type { CoprocessorInstance } from "./topology";

function asCoprocessorService(instance: CoprocessorInstance, suffix: string): string {
  return `${instance.servicePrefix}-${suffix}`;
}

function asComposeContainerName(serviceName: string): string {
  return `${DOCKER_PROJECT}-${serviceName}-1`;
}

export function generateInstanceServices(instance: CoprocessorInstance): ServiceDefinition[] {
  if (instance.index === 0) {
    return getServicesByGroup("coprocessor");
  }

  return COPROCESSOR_TEMPLATES.map((template) => {
    const name = asCoprocessorService(instance, template.suffix);

    return {
      name,
      group: "coprocessor",
      composeFile: instance.composeFile,
      envFile: instance.envFileName,
      versionVar: template.versionVar,
      containerName: asComposeContainerName(name),
      isOneShot: template.isOneShot,
      isBuildable: template.isBuildable,
      healthCheck: template.healthCheck,
      healthEndpoint:
        template.suffix === "gw-listener"
          ? `http://localhost:${instance.gwListenerPort}/liveness`
          : undefined,
      ports: template.suffix === "gw-listener" ? [instance.gwListenerPort] : undefined,
    };
  });
}

export function generateAllInstanceServices(instances: CoprocessorInstance[]): ServiceDefinition[] {
  return instances.flatMap((instance) => generateInstanceServices(instance));
}

export function getAllCoprocessorServiceNames(instances: CoprocessorInstance[]): string[] {
  return generateAllInstanceServices(instances).map((service) => service.name);
}
