'use server'
import {
  AccountMeta,
  AccountRole,
  AccountSignerMeta,
  address,
  appendTransactionMessageInstruction,
  BaseTransactionMessage,
  compileTransactionMessage,
  createKeyPairFromBytes,
  createKeyPairSignerFromBytes,
  createSolanaClient,
  createTransaction,
  createTransactionMessage,
  getAddressCodec,
  getAddressEncoder,
  getBase64Decoder,
  getBase64EncodedWireTransaction,
  getBase64Encoder,
  getEnumCodec,
  getProgramDerivedAddress,
  getStructCodec,
  getU64Codec,
  Instruction,
  pipe,
  sendAndConfirmTransactionFactory,
  setTransactionMessageFeePayer,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransaction,
  signTransactionMessageWithSigners,
  TransactionMessageWithFeePayer,
  TransactionSigner,
} from 'gill'
import { SwapTokens } from '../types'
import * as borsh from 'borsh'
import { get_lexicographical_token_pda } from '../utils'

export default async function swapTokens(data: SwapTokens) {
  'use server'
  try {
    // if (data.provider_account !== process.env.ADMIN_ACCOUNT) {
    //     return { error: 'Unauthorized' }
    // }

    const { rpc, rpcSubscriptions } = createSolanaClient({ urlOrMoniker: 'devnet' })

    //helper function to get all the required accounts
    const createAmmAccounts: (AccountMeta | AccountSignerMeta)[] = await AmmaccountsCreator(data)

    console.log('Account Count:', createAmmAccounts.length)
    createAmmAccounts.forEach((acc, i) => console.log(`Index ${i}: ${acc.address}`))

    //using codecs to serialize the instruction data
    const codec = getStructCodec([
      ['amount_in', getU64Codec()],
      ['minimum_amount_out', getU64Codec()],
      ['mint_address_in', getAddressCodec()],
      ['mint_address_out', getAddressCodec()],
    ])

    const createAmmData = codec.encode({
      amount_in: parseInt(data.max_amount_in),
      minimum_amount_out: parseInt(data.minimum_amount_out),
      mint_address_in: address(data.token_a_mint_account),
      mint_address_out: address(data.token_b_mint_account),
    })

    const finalAmmData = new Uint8Array(1 + createAmmData.length)
    finalAmmData.set([3])
    finalAmmData.set(createAmmData, 1)

    //addliquidity instruction creation
    const createAmmInstruction: Instruction = {
      accounts: createAmmAccounts,
      data: finalAmmData,
      programAddress: address(process.env.AMM_PROGRAM_ADDRESS!),
    }
    return createAmmInstruction;
    // const { value: blockhash } = await rpc.getLatestBlockhash().send()
    // // const baseTx = createTransaction({ version: 0 })
    // const createAmmTransaction = pipe(
    //   createTransactionMessage({ version: 'legacy' }),
    //   (tx) => setTransactionMessageFeePayer(address(data.swapper_account), tx),
    //   (tx) => setTransactionMessageLifetimeUsingBlockhash(blockhash, tx),
    //   (tx) => appendTransactionMessageInstruction(createAmmInstruction, tx),
    // )

    // // const compiledTransactionMessage = compileTransactionMessage(createAmmTransaction)
    // //  as BaseTransactionMessage & TransactionMessageWithFeePayer;
    // // const basetx = getbasetransactio
    // // import { createSolanaRpcSubscriptions, address } from '@solana/kit';

    // // const rpcSubscriptions = createSolanaRpcSubscriptions('wss://api.mainnet-beta.solana.com');
    // const myAddress = address(data.swapper_account)
    // const abortController = new AbortController()

    // async function subscribeToAccount() {
    //   const notifications = await rpcSubscriptions
    //     .accountNotifications(myAddress, { commitment: 'confirmed' })
    //     .subscribe({ abortSignal: abortController.signal })

    //   // The loop waits for new notifications automatically
    //   for await (const notification of notifications) {
    //     console.log('Account Updated! New balance:', notification.value.lamports)
    //     console.log('Slot:', notification.context.slot)
    //   }
    // }
    // async function subscribeToTransactions() {
    //   const logSubscription = await rpcSubscriptions
    //     .logsNotifications({ mentions: [myAddress] }, { commitment: 'confirmed' })
    //     .subscribe({ abortSignal: abortController.signal })

    //   for await (const log of logSubscription) {
    //     console.log('Transaction detected!')
    //     console.log('Signature:', log.value.signature)
    //     console.log('Logs:', log.value.logs)

    //     if (log.value.err) {
    //       console.error('Transaction failed:', log.value.err)
    //     }
    //   }
    // }
    // subscribeToAccount()
    // subscribeToTransactions()
    // // signTransaction([admin], compiledTransactionMessage)
    // const compiledTx = compileTransactionMessage(createAmmTransaction)
    // return compiledTx;
    // const signedTx = await signTransactionMessageWithSigners(createAmmTransaction)
    // const encoder64 = getBase64EncodedWireTransaction(signedTx)
    // const msg = encoder64.decode(sig)
    // const simulated = await rpc
    //   .simulateTransaction(encoder64, {
    //     encoding: 'base64',
    //     commitment: 'confirmed',
    //   })
    //   .send()
    // console.log(simulated.value.logs)
    // const sendAndConfirmTransaction = sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions })
    // console.log(signedTx)
    // const signature = await sendAndConfirmTransaction(signedTx, { commitment: 'confirmed' })
    // console.log(signature)
    // const { value: signature } = await factory(compiledTransactionMessage).send();
    // return { instruction: createAmmInstruction }
    // abortController.abort()
  } catch (err) {
    console.log(err)
  }
}
async function AmmaccountsCreator(data: SwapTokens): Promise<(AccountMeta | AccountSignerMeta)[]> {
  const accounts: (AccountMeta | AccountSignerMeta)[] = []
  const addressEncoder = getAddressEncoder()

  //0. swapper_account(signer, writable)
  // const admin = await createKeyPairSignerFromBytes(
  //   new Uint8Array([
  //     10, 92, 27, 184, 114, 49, 64, 68, 52, 212, 240, 162, 153, 140, 141, 89, 87, 248, 49, 41, 56, 143, 73, 183, 89,
  //     195, 22, 183, 89, 59, 194, 35, 227, 129, 76, 82, 217, 163, 19, 224, 13, 216, 217, 51, 54, 111, 34, 248, 164, 178,
  //     79, 111, 66, 116, 188, 189, 132, 117, 72, 191, 89, 77, 133, 134,
  //   ]),
  // )
  accounts.push({
    //address: address(data.admin_account),
    address: address(data.swapper_account),
    // signer: admin,
    role: AccountRole.WRITABLE_SIGNER,
  })

  //1. spl_token_account (readonly)
  accounts.push({
    address: address('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'),
    role: AccountRole.READONLY,
  })

  //2. amm_token_account(writable)
  const [amm_token_account_address] = await get_lexicographical_token_pda(
    address(data.token_a_mint_account),
    address(data.token_b_mint_account),
    address(process.env.AMM_PROGRAM_ADDRESS!),
  )
  accounts.push({
    address: address(amm_token_account_address),
    role: AccountRole.WRITABLE,
  })
  //3. amm_token_a_pool_account(writable)
  const [token_a_pool_account_address] = await getProgramDerivedAddress({
    programAddress: address(process.env.AMM_PROGRAM_ADDRESS!),
    seeds: [
      'pool',
      addressEncoder.encode(address(amm_token_account_address)),
      addressEncoder.encode(address(data.token_a_mint_account)),
    ],
  })
  console.log(
    'amm_token_a_pool_account',
    amm_token_account_address,
    data.token_a_mint_account,
    token_a_pool_account_address,
  )
  accounts.push({
    address: address(token_a_pool_account_address),
    role: AccountRole.WRITABLE,
  })
  //4. amm_token_b_pool_account(writable)
  const [token_b_pool_account_address] = await getProgramDerivedAddress({
    programAddress: address(process.env.AMM_PROGRAM_ADDRESS!),
    seeds: [
      'pool',
      addressEncoder.encode(address(amm_token_account_address)),
      addressEncoder.encode(address(data.token_b_mint_account)),
    ],
  })
  console.log(
    'amm_token_b_pool_account',
    amm_token_account_address,
    data.token_b_mint_account,
    token_b_pool_account_address,
  )
  accounts.push({
    address: address(token_b_pool_account_address),
    role: AccountRole.WRITABLE,
  })

  //5. swapper_token_a_account(writable)
  accounts.push({
    address: address(data.swapper_token_a_account),
    role: AccountRole.WRITABLE,
  })

  //6. swapper_token_b_account(writable)
  accounts.push({
    address: address(data.swapper_token_b_account),
    role: AccountRole.WRITABLE,
  })

  // //10.sysvar_rent_account (readonly)
  // accounts.push({
  //     address: address('SysvarRent111111111111111111111111111111111'),
  //     role: AccountRole.READONLY,
  // })

  //7. token_a_mint_account(read only)
  accounts.push({
    address: address(data.token_a_mint_account),
    role: AccountRole.READONLY,
  })

  //8. token_b_mint_account(read only)
  accounts.push({
    address: address(data.token_b_mint_account),
    role: AccountRole.READONLY,
  })

  //9. system_program_account(read only)
  accounts.push({
    address: address('11111111111111111111111111111111'),
    role: AccountRole.READONLY,
  })

  //10. amm_program_account (read only)
  accounts.push({
    address: address(process.env.AMM_PROGRAM_ADDRESS!),
    role: AccountRole.READONLY,
  })

  return accounts
}
