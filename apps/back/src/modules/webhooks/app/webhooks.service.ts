import { Injectable, Logger } from '@nestjs/common'
import { DeveloperPortalService } from './developer-portal.service.js'
import type { User } from '../core/types.js'
import type { ApplicationRegistered, UserRegistered } from './webhooks.types.js'
import * as ur from '../core/user-registered.machine.js'
import * as ar from '../core/application-registered.machine.js'
import { createActor } from 'xstate'

@Injectable()
export class WebhookService {
  private readonly logger = new Logger(WebhookService.name)
  constructor(private readonly developerPortal: DeveloperPortalService) {}

  async handleUserRegistered(user: UserRegistered) {
    return new Promise<void>((resolve, reject) => {
      const actor = createActor(
        ur.factory(this.developerPortal, {
          id: user.ID,
          name: `${user.First} ${user.Last}`.trim(),
          email: user.Email,
          orgId: user.OrgID,
          // the event contains the user's teams names, not their ids,
          // so we ignore it
          teamIds: [],
        } satisfies User),
      ).start()
      actor.subscribe({
        next: snapshot => {
          this.logger.debug(
            `[${snapshot.status}] ${JSON.stringify(snapshot.value)}: ${JSON.stringify(snapshot.context)}`,
          )
        },
        complete: () => {
          const snapshot = actor.getSnapshot()

          if (actor.getSnapshot().value === 'failed') {
            this.logger.warn(
              `Failed to register user ${user.ID}: ${snapshot.context.error}`,
            )
            reject(snapshot.context.error)
          } else {
            this.logger.log('actor completed')
            resolve()
          }
        },

        error: err => {
          this.logger.warn(`actor error: ${err}`)
          reject(err)
        },
      })
      actor.send({ type: 'START' })
    })
  }

  async handleApplicationRegistered(app: ApplicationRegistered) {
    return new Promise<void>((resolve, reject) => {
      const actor = createActor(
        ar.factory(this.developerPortal, app.ID),
      ).start()
      actor.subscribe({
        next: snapshot => {
          this.logger.debug(
            `[${snapshot.status}] ${JSON.stringify(snapshot.value)}: ${JSON.stringify(snapshot.context)}`,
          )
        },
        complete: () => {
          const snapshot = actor.getSnapshot()

          if (actor.getSnapshot().value === 'failed') {
            this.logger.warn(
              `Failed to register application ${app.ID}: ${snapshot.context.error}`,
            )
            reject(snapshot.context.error)
          } else {
            this.logger.log('actor completed')
            resolve()
          }
        },

        error: err => {
          this.logger.warn(`actor error: ${err}`)
          reject(err)
        },
      })
      actor.send({ type: 'START' })
    })
  }
}
