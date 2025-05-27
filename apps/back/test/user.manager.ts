import { GraphQl } from './graphql.js'
import { SetupManager } from './setup.manager.js'

export class UserManager {
  constructor(private readonly manager: SetupManager) {}

  get httpServer() {
    return this.manager.httpServer
  }

  async changePassword({
    token,
    oldPassword,
    newPassword,
  }: {
    token: string
    oldPassword: string
    newPassword: string
  }) {
    return GraphQl.request<
      { changePassword: boolean },
      { oldPassword: string; newPassword: string }
    >(this.httpServer)
      .auth(token)
      .mutate(CHANGE_PASSWORD, { oldPassword, newPassword })
      .exec('changePassword')
  }
}

const CHANGE_PASSWORD = `
  mutation ChangePassword($oldPassword: String!, $newPassword: String!) {
    changePassword(input: {
      oldPassword: $oldPassword, 
      newPassword: $newPassword
    })
  }
`
