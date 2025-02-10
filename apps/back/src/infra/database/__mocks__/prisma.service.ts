import { PrismaClient } from '#prisma/client/index.js'
import { beforeEach } from 'vitest'
import { DeepMockProxy, mockDeep, mockReset } from 'vitest-mock-extended'

const prisma = mockDeep<PrismaClient>()

// prisma.dappStat.findMany.mockResolvedValue([])

beforeEach(() => {
  mockReset(prisma)
})

export class PrismaService {
  get user(): DeepMockProxy<PrismaClient['user']> {
    return prisma.user
  }

  get team(): DeepMockProxy<PrismaClient['team']> {
    return prisma.team
  }

  get invitation(): DeepMockProxy<PrismaClient['invitation']> {
    return prisma.invitation
  }

  get dapp(): DeepMockProxy<PrismaClient['dapp']> {
    return prisma.dapp
  }

  get dappStat(): DeepMockProxy<PrismaClient['dappStat']> {
    return prisma.dappStat
  }
}
