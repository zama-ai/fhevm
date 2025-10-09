import { Router } from "express";
import bodyParser from "body-parser";
import { authMiddleware } from "../services/authPlugin";
import { ApiKeysController } from "../controllers/api-keys.controller";
import provisioningService from "../services/kong-provisioning.service";

const jsonParser = bodyParser.json();

const controller = new ApiKeysController(provisioningService);

const router = Router();

router.post("/", authMiddleware, jsonParser, controller.createApiKey);
router.get("/", authMiddleware, controller.listApiKeys);
router.get("/:apiKeyId", authMiddleware, controller.getApiKey);
router.delete("/:apiKeyId", authMiddleware, controller.deleteApiKey);

export default router;
