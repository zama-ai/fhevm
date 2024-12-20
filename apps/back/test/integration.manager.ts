import { AuthManager } from './auth.manager'
import { SetupManager } from './setup.manager'
import { DappManager } from './dapp.manager'

export type { GraphQlResponse } from './setup.manager'
export type { User } from './auth.manager'
export type { DApp } from './dapp.manager'

export class IntegrationManager {
  readonly setup = new SetupManager()
  readonly auth = new AuthManager(this.setup)
  readonly dapp = new DappManager(this.setup, this.auth)

  get httpServer() {
    return this.setup.httpServer
  }
  async beforeAll() {
    await this.setup.beforeAll()
  }

  async afterAll() {
    await this.setup.afterAll()
  }

  async afterEach() {
    await this.setup.afterEach()
  }
}
