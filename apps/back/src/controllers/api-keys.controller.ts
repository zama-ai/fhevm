import type { Request, Response } from "express";
import { getLogger } from "../common/logger.context";
import { withSpan } from "../decorators/span";
import { KongProvisioningService } from "../services/kong-provisioning.service";
import { getUnifiedCustomerId } from "../services/commonUtils";

export class ApiKeysController {
  constructor(private provisioningService: KongProvisioningService) {}

  createApiKey = withSpan(
    {
      name: "createApiKey",
      extractAttributesFromArgs: (args) => ({
        email: args[0].user?.email,
      }),
    },
    async (req: Request, res: Response) => {
      const logger = getLogger();
      try {
        // We are behind AuthMiddleware, so user is defined
        const email = req.user!.email;

        const customerId = await getUnifiedCustomerId(req.user, email);
        if (!customerId) {
          throw new Error(
            `Customer Id unknown. Ensure you're subscribed to a plan. If you just subscribed, try again.`
          );
        }

        const apiKey = await this.provisioningService.createApiKey(
          customerId,
          email
        );
        res.status(201).send(apiKey);
      } catch (error) {
        logger.error(`Error creating key: ${error}`);
        res.status(500).json({ status: 500, message: "Failed to create key" });
      }
    }
  );

  listApiKeys = withSpan(
    {
      name: "listApiKeys",
      extractAttributesFromArgs: (args) => ({
        email: args[0].user?.email,
      }),
    },
    async (req: Request, res: Response) => {
      const logger = getLogger();
      try {
        // We are behind AuthMiddleware, so user is defined
        const email = req.user!.email;

        const customerId = await getUnifiedCustomerId(req.user, email);
        if (!customerId) {
          throw new Error(
            `Customer Id unknown. Ensure you're subscribed to a plan. If you just subscribed, try again.`
          );
        }

        const apiKeys = await this.provisioningService.listApiKeys(
          customerId,
          email
        );
        res.status(200).send(apiKeys);
      } catch (error) {
        logger.error(`Error listing key: ${error}`);
        res.status(500).json({ status: 500, message: "Failed to list keys" });
      }
    }
  );

  getApiKey = withSpan(
    {
      name: "getApiKey",
      extractAttributesFromArgs: (args) => ({
        email: args[0].user?.email,
        apiKeyId: args[0].params?.apiKeyId,
      }),
    },
    async (req: Request, res: Response) => {
      const logger = getLogger();
      try {
        // We are behind AuthMiddleware, so user is defined
        const email = req.user!.email;
        const apiKeyId = req.params.apiKeyId;

        const customerId = await getUnifiedCustomerId(req.user, email);
        if (!customerId) {
          throw new Error(
            `Customer Id unknown. Ensure you're subscribed to a plan. If you just subscribed, try again.`
          );
        }

        const apiKey = await this.provisioningService.getApiKey(
          customerId,
          email,
          apiKeyId
        );
        res.status(200).send(apiKey);
      } catch (error) {
        logger.error(`Error getting key`);
        res.status(500).json({ status: 500, message: "Failed to get key" });
      }
    }
  );

  deleteApiKey = withSpan(
    {
      name: "deleteApiKey",
      extractAttributesFromArgs: (args) => ({
        email: args[0].user?.email,
        apiKeyId: args[0].params?.apiKeyId,
      }),
    },
    async (req: Request, res: Response) => {
      const logger = getLogger();
      try {
        // We are behind AuthMiddleware, so user is defined
        const email = req.user!.email;
        const apiKeyId = req.params.apiKeyId;

        logger.info(`Deleting an API key for a user`);

        const customerId = await getUnifiedCustomerId(req.user, email);
        if (!customerId) {
          throw new Error(
            `Customer Id unknown. Ensure you're subscribed to a plan. If you just subscribed, try again.`
          );
        }

        await this.provisioningService.deleteApiKey(
          customerId,
          email,
          apiKeyId
        );
        res.status(204).end();
      } catch (error) {
        logger.error(`Error deleting key: ${error}`);
        res.status(500).json({ status: 500, message: "Failed to delete key" });
      }
    }
  );
}
