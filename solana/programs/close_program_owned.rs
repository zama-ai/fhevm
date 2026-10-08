// The preview-env wipe (DD-051), shared by zama-host and the apps: closes each target this program
// owns, crediting its rent to `admin`, and skips any other account, including an already closed
// one. It is separate from `preview_cleanup.rs` because zama-host does not depend on `anchor-spl`.

fn close_program_owned<'info>(
    admin: &AccountInfo<'info>,
    targets: &[AccountInfo<'info>],
) -> Result<()> {
    for target in targets {
        if target.owner != &crate::ID {
            continue;
        }
        let rent = target.lamports();
        **target.try_borrow_mut_lamports()? = 0;
        **admin.try_borrow_mut_lamports()? = admin
            .lamports()
            .checked_add(rent)
            .ok_or(ProgramError::ArithmeticOverflow)?;
        target.resize(0)?;
        target.assign(&anchor_lang::solana_program::system_program::ID);
    }
    Ok(())
}
