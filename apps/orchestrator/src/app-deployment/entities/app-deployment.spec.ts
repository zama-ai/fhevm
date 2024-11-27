import { beforeEach, describe, expect, it } from 'vitest';
import { AppDeployment } from './app-deployment';
import { randomUUID } from 'crypto';

import {
  completed,
  requested,
  scConfirmed,
  scDiscovered,
  scRegistered,
} from './app-deployment.events';
import { confirmSM, discoverSM, registerSM } from './app-deployment.commands';

const address = '0xa2dd817c2fdc3a2996f1a5174cf8f1aaed466e82';
const chainId = '1';

describe('AppDeployment', () => {
  let deployment: AppDeployment;
  let applicationId: string;

  beforeEach(() => {
    applicationId = randomUUID();
    deployment = AppDeployment.init(applicationId);
  });

  describe('when idle', () => {
    beforeEach(() => {
      expect(deployment.status).toBe('Idle');
    });

    it('should request SC discovery on deployment requested', () => {
      const messages = deployment.notify(
        requested({ applicationId, address, chainId }),
      );

      expect(messages).toEqual([
        discoverSM({ applicationId, address, chainId }),
      ]);
    });
  });

  describe('when discovering', () => {
    beforeEach(() => {
      deployment.notify(requested({ applicationId, address, chainId }));
      expect(deployment.status).toBe('Discovering');
    });

    it('should request SC confirmation on SC discovered', () => {
      const messages = deployment.notify(scDiscovered({ applicationId }));

      expect(messages).toEqual([confirmSM({ applicationId })]);
    });

    // it('should fail on SM discovery failed', () => {
    //   const messages = deployment.notify()
    // })
  });

  describe('when confirming', () => {
    beforeEach(() => {
      deployment.notify(requested({ applicationId, address, chainId }));
      deployment.notify(scDiscovered({ applicationId }));
      expect(deployment.status).toBe('Confirming');
    });

    it('should request SM registration on SM confirmation', () => {
      const messages = deployment.notify(scConfirmed({ applicationId }));

      expect(messages).toEqual([registerSM({ applicationId })]);
    });
  });

  describe('when registering', () => {
    beforeEach(() => {
      deployment.notify(requested({ applicationId, address, chainId }));
      deployment.notify(scDiscovered({ applicationId }));
      deployment.notify(scConfirmed({ applicationId }));
      expect(deployment.status).toBe('Registering');
    });

    it('should complete on SM registered', () => {
      const messages = deployment.notify(scRegistered({ applicationId }));
      expect(messages).toEqual([completed({ applicationId })]);
      expect(deployment.status).toBe('Active');
    });
  });
});
