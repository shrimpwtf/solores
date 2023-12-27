use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
};
use std::io::Read;
#[derive(Clone, Debug, PartialEq)]
pub enum ShankIxNoPrivilegeProgramIx {
    NoPrivilegedAccountIx(NoPrivilegedAccountIxIxArgs),
}
impl ShankIxNoPrivilegeProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        match maybe_discm {
            NO_PRIVILEGED_ACCOUNT_IX_IX_DISCM => Ok(Self::NoPrivilegedAccountIx(
                NoPrivilegedAccountIxIxArgs::deserialize(&mut reader)?,
            )),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm {:?} not found", maybe_discm),
            )),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::NoPrivilegedAccountIx(args) => {
                writer.write_all(&[NO_PRIVILEGED_ACCOUNT_IX_IX_DISCM])?;
                args.serialize(&mut writer)
            }
        }
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn invoke_instruction<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
    ix: &Instruction,
    accounts: A,
) -> ProgramResult {
    let account_info: [AccountInfo<'info>; N] = accounts.into();
    invoke(ix, &account_info)
}
pub fn invoke_instruction_signed<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
    ix: &Instruction,
    accounts: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let account_info: [AccountInfo<'info>; N] = accounts.into();
    invoke_signed(ix, &account_info, seeds)
}
pub const NO_PRIVILEGED_ACCOUNT_IX_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct NoPrivilegedAccountIxAccounts<'me, 'info> {
    pub b: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct NoPrivilegedAccountIxKeys {
    pub b: Pubkey,
}
impl From<NoPrivilegedAccountIxAccounts<'_, '_>> for NoPrivilegedAccountIxKeys {
    fn from(accounts: NoPrivilegedAccountIxAccounts) -> Self {
        Self { b: *accounts.b.key }
    }
}
impl From<NoPrivilegedAccountIxKeys> for [AccountMeta; NO_PRIVILEGED_ACCOUNT_IX_IX_ACCOUNTS_LEN] {
    fn from(keys: NoPrivilegedAccountIxKeys) -> Self {
        [AccountMeta {
            pubkey: keys.b,
            is_signer: false,
            is_writable: false,
        }]
    }
}
impl From<[Pubkey; NO_PRIVILEGED_ACCOUNT_IX_IX_ACCOUNTS_LEN]> for NoPrivilegedAccountIxKeys {
    fn from(pubkeys: [Pubkey; NO_PRIVILEGED_ACCOUNT_IX_IX_ACCOUNTS_LEN]) -> Self {
        Self { b: pubkeys[0] }
    }
}
impl<'info> From<NoPrivilegedAccountIxAccounts<'_, 'info>>
    for [AccountInfo<'info>; NO_PRIVILEGED_ACCOUNT_IX_IX_ACCOUNTS_LEN]
{
    fn from(accounts: NoPrivilegedAccountIxAccounts<'_, 'info>) -> Self {
        [accounts.b.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; NO_PRIVILEGED_ACCOUNT_IX_IX_ACCOUNTS_LEN]>
    for NoPrivilegedAccountIxAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; NO_PRIVILEGED_ACCOUNT_IX_IX_ACCOUNTS_LEN]) -> Self {
        Self { b: &arr[0] }
    }
}
pub const NO_PRIVILEGED_ACCOUNT_IX_IX_DISCM: u8 = 69u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NoPrivilegedAccountIxIxArgs {
    pub arg: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct NoPrivilegedAccountIxIxData(pub NoPrivilegedAccountIxIxArgs);
impl From<NoPrivilegedAccountIxIxArgs> for NoPrivilegedAccountIxIxData {
    fn from(args: NoPrivilegedAccountIxIxArgs) -> Self {
        Self(args)
    }
}
impl NoPrivilegedAccountIxIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != NO_PRIVILEGED_ACCOUNT_IX_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    NO_PRIVILEGED_ACCOUNT_IX_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(NoPrivilegedAccountIxIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[NO_PRIVILEGED_ACCOUNT_IX_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn no_privileged_account_ix_ix_with_program_id(
    program_id: Pubkey,
    keys: NoPrivilegedAccountIxKeys,
    args: NoPrivilegedAccountIxIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; NO_PRIVILEGED_ACCOUNT_IX_IX_ACCOUNTS_LEN] = keys.into();
    let data: NoPrivilegedAccountIxIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn no_privileged_account_ix_ix(
    keys: NoPrivilegedAccountIxKeys,
    args: NoPrivilegedAccountIxIxArgs,
) -> std::io::Result<Instruction> {
    no_privileged_account_ix_ix_with_program_id(crate::ID, keys, args)
}
pub fn no_privileged_account_ix_invoke(
    accounts: NoPrivilegedAccountIxAccounts<'_, '_>,
    args: NoPrivilegedAccountIxIxArgs,
) -> ProgramResult {
    let keys: NoPrivilegedAccountIxKeys = accounts.into();
    let ix = no_privileged_account_ix_ix(keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn no_privileged_account_ix_invoke_signed(
    accounts: NoPrivilegedAccountIxAccounts<'_, '_>,
    args: NoPrivilegedAccountIxIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: NoPrivilegedAccountIxKeys = accounts.into();
    let ix = no_privileged_account_ix_ix(keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn no_privileged_account_ix_verify_account_keys(
    accounts: NoPrivilegedAccountIxAccounts<'_, '_>,
    keys: NoPrivilegedAccountIxKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(accounts.b.key, &keys.b)] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
