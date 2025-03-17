import { registerAs } from '@nestjs/config'

export default registerAs('web3', () => ({
  chainIds: process.env.WEB3_CHAIN_IDS?.split(','),
  every: process.env.WEB3_CRON_EVERY,
}))
