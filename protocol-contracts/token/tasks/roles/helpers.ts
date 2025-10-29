import { Contract } from 'ethers'

export type RoleKey = 'MINTER_ROLE' | 'PAUSING_MINTER_ROLE' | 'DEFAULT_ADMIN_ROLE'

export const ROLE_ACCESSORS: Record<RoleKey, string> = {
    MINTER_ROLE: 'MINTER_ROLE',
    PAUSING_MINTER_ROLE: 'MINTING_PAUSER_ROLE',
    DEFAULT_ADMIN_ROLE: 'DEFAULT_ADMIN_ROLE',
}

export const ROLE_TASK_SUFFIX: Record<RoleKey, string> = {
    MINTER_ROLE: 'minter_role',
    PAUSING_MINTER_ROLE: 'pausing_minter_role',
    DEFAULT_ADMIN_ROLE: 'default_admin_role',
}

export async function resolveRoleValue(zamaErc20: Contract, role: RoleKey): Promise<string> {
    const accessor = ROLE_ACCESSORS[role]
    const getter = zamaErc20[accessor]
    if (typeof getter !== 'function') {
        throw new Error(`Role accessor ${accessor}() not available on ZamaERC20 contract`)
    }
    return getter()
}
