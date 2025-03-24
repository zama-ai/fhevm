import { web3 } from 'messages'
import { AppError, Task } from 'utils'

export interface ProducerService {
  sendMessage(message: web3.Web3Event): Task<void, AppError>
}
