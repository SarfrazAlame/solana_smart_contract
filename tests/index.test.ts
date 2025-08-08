import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction } from '@solana/web3.js'
import { expect, test } from 'bun:test'
import { COUNTER_SIZE, schema } from './types'
import * as borsh from "borsh"

let adminAccount = Keypair.generate()
let dataAccount = Keypair.generate()

const programId = new PublicKey('DTfDw13TFTxFAra6roNz8E7n2FK9m4PNfoA6CPzVyeNU')

test('counter does increase', async () => {
    const connection = new Connection('http://127.0.0.1:8899')
    const txn = await connection.requestAirdrop(adminAccount.publicKey, 2 * LAMPORTS_PER_SOL)
    await connection.confirmTransaction(txn)

    // now creating data account for storing counter value
    const lamports = await connection.getMinimumBalanceForRentExemption(COUNTER_SIZE)
    const ix = SystemProgram.createAccount({
        fromPubkey: adminAccount.publicKey,
        newAccountPubkey: dataAccount.publicKey,
        lamports,
        programId,
        space:COUNTER_SIZE
    })

    const txn2 = new Transaction().add(ix)

    const signature = await connection.sendTransaction(txn2, [adminAccount, dataAccount])

    await connection.confirmTransaction(signature);

    // console.log(dataAccount.publicKey.toBase58()).
    const dataAccountInternalInfo = await connection.getAccountInfo(dataAccount.publicKey)
    const counter = borsh.deserialize(schema, dataAccountInternalInfo?.data)

    console.log(counter.count)
    expect(counter.count).toBe(0)
})
