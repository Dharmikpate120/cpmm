'use client'

import { useState, ChangeEvent } from 'react'
// import DashboardFeature from '@/features/dashboard/dashboard-feature'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { toast } from 'sonner'
import { AlertCircle } from 'lucide-react'

import addLiquidity from '@/lib/actions/addLiquidity'
import withdrawLiquidity from '@/lib/actions/withdrawLiquidity'
import { AmmAccountData, AmmAddLiquidity, AmmWithdrawLiquidity, SwapTokens } from '@/lib/types'
import swapTokens from '@/lib/actions/swapTokens'
import { useSignAndSendTransaction, useWalletUi, useWalletUiSigner } from '@wallet-ui/react'
import { useWalletUiGill, useWalletUiSignAndSend } from '@wallet-ui/react-gill'
import { getTransactionEncoder } from 'gill'
import DashboardFeature from '@/features/dashboard/dashboard-feature'

export default function Home() {
  const { connected } = useWalletUi()
  return connected ? <ConnectedComponent /> : <DashboardFeature />


}
export const maxDuration = 200;
function ConnectedComponent() {
  const walletUi = useWalletUi()
  const client = useWalletUiGill()

  const signer = useWalletUiSigner({ account: walletUi.account! });
  const sender = useWalletUiSignAndSend();

  /* Create Market logic moved to /admin */

  const [addLiquidityData, setAddLiquidityData] = useState({
    token_a_mint_account: '6pXNd5iDL3kqz8MxGiqZu4hb9vB9KbrPknGJ5vXfkMgd',
    token_b_mint_account: 'DB1xwND2situnNxmSCF3XNHnXXayLPoNSHDKJEfNiwpP',
    provider_token_a_account: 'cfqVCaPpWougv7Kq8kamHk3oZRNYhrTK6QURcACe1xt',
    provider_token_b_account: 'ADE1HAFKUiNSvWyLUDoQmLsNCqUKm297m6HRfx9GPynn',
    amount_a_max: '100',
    amount_b_max: '100',
    minimum_lp_tokens: '1',
  })

  const [withdrawLiquidityData, setWithdrawLiquidityData] = useState({
    token_a_mint_account: '6pXNd5iDL3kqz8MxGiqZu4hb9vB9KbrPknGJ5vXfkMgd',
    token_b_mint_account: 'DB1xwND2situnNxmSCF3XNHnXXayLPoNSHDKJEfNiwpP',
    provider_token_a_account: 'cfqVCaPpWougv7Kq8kamHk3oZRNYhrTK6QURcACe1xt',
    provider_token_b_account: 'ADE1HAFKUiNSvWyLUDoQmLsNCqUKm297m6HRfx9GPynn',
    amount_a_min: '100',
    amount_b_min: '100',
    maximum_lp_tokens: '100000',
  })
  // amm_token_a_pool_account 8HQanjD9QxL2dDC3aSZMW6VoyeRkZTK43wKpzhEwhExJ 6pXNd5iDL3kqz8MxGiqZu4hb9vB9KbrPknGJ5vXfkMgd BngwetXKv5uZVHAdta3SoY2n7BNqdtMPHUH4doiRR6tT
  // amm_token_b_pool_account 8HQanjD9QxL2dDC3aSZMW6VoyeRkZTK43wKpzhEwhExJ DB1xwND2situnNxmSCF3XNHnXXayLPoNSHDKJEfNiwpP AkZGSWn8wZqwPQ5uuaUpvVDBSHf2b7RimiqrFUYWjYRd  
  const [swapTokensData, setSwapTokensData] = useState({
    swapper_token_a_account: 'cfqVCaPpWougv7Kq8kamHk3oZRNYhrTK6QURcACe1xt',
    swapper_token_b_account: 'ADE1HAFKUiNSvWyLUDoQmLsNCqUKm297m6HRfx9GPynn',
    // swapper_token_a_account: 'ADE1HAFKUiNSvWyLUDoQmLsNCqUKm297m6HRfx9GPynn',
    // swapper_token_b_account: 'cfqVCaPpWougv7Kq8kamHk3oZRNYhrTK6QURcACe1xt',

    token_a_mint_account: '6pXNd5iDL3kqz8MxGiqZu4hb9vB9KbrPknGJ5vXfkMgd',
    token_b_mint_account: 'DB1xwND2situnNxmSCF3XNHnXXayLPoNSHDKJEfNiwpP',
    // token_a_mint_account: 'DB1xwND2situnNxmSCF3XNHnXXayLPoNSHDKJEfNiwpP',
    // token_b_mint_account: '6pXNd5iDL3kqz8MxGiqZu4hb9vB9KbrPknGJ5vXfkMgd',

    max_amount_in: '10000',
    minimum_amount_out: '1',
  })

  const [error, setError] = useState<string | null>(null)

  const handleAddLiquidityChange = (e: ChangeEvent<HTMLInputElement>) => {
    const { id, value } = e.target
    setAddLiquidityData((prev) => ({ ...prev, [id]: value }))
  }

  const handleWithdrawLiquidityChange = (e: ChangeEvent<HTMLInputElement>) => {
    const { id, value } = e.target
    setWithdrawLiquidityData((prev) => ({ ...prev, [id]: value }))
  }

  const handleSwapTokensChange = (e: ChangeEvent<HTMLInputElement>) => {
    const { id, value } = e.target
    setSwapTokensData((prev) => ({ ...prev, [id]: value }))
  }

  const handleAddLiquidity = async () => {
    const isAnyFieldEmpty = Object.values(addLiquidityData).some((value) => value === '')
    if (isAnyFieldEmpty) {
      const errorMessage = 'All fields are required. Please fill in all the details.'
      setError(errorMessage)
      toast.error(errorMessage)
      return
    }
    if (!walletUi.account) {
      setError("Wallet not connected!");
      toast.error("Wallet not connected!");
      return;
    }
    setError(null)
    const instruction = await addLiquidity({ ...addLiquidityData, provider_account: walletUi.account.address });
    if (!walletUi.account || !instruction) {
      return;
    }
    const result = await sender([instruction!], signer);
    console.log(result);

  }

  const handleWithdrawLiquidity = async () => {
    const isAnyFieldEmpty = Object.values(withdrawLiquidityData).some((value) => value === '')
    if (isAnyFieldEmpty) {
      const errorMessage = 'All fields are required. Please fill in all the details.'
      setError(errorMessage)
      toast.error(errorMessage)
      return
    }
    if (!walletUi.account) {
      setError("Wallet not connected!");
      toast.error("Wallet not connected!");
      return;
    }
    setError(null)
    const instruction = await withdrawLiquidity({ ...withdrawLiquidityData, provider_account: walletUi.account.address });
    if (!walletUi.account || !instruction) {
      return;
    }
    const result = await sender([instruction!], signer);
    console.log(result);
  }

  const handleSwapTokens = async () => {
    const isAnyFieldEmpty = Object.values(swapTokensData).some((value) => value === '')
    if (isAnyFieldEmpty) {
      const errorMessage = 'All fields are required. Please fill in all the details.'
      setError(errorMessage)
      toast.error(errorMessage)
      return
    }
    if (!walletUi.account) {
      setError("Wallet not connected!");
      toast.error("Wallet not connected!");
      return;
    }
    setError(null)
    const instruction = await swapTokens({ ...swapTokensData, swapper_account: walletUi.account.address });

    if (!walletUi.account || !instruction) {
      return;
    }
    const result = await sender([instruction!], signer);
    console.log(result);
    // getTransactionEncoder(transaction)

    // signer.signAndSendTransactions(transaction)
  }

  return (
    <div className="container mx-auto p-4">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold">Dashboard</h1>
        <div className="flex gap-2">
          <Dialog>
            <DialogTrigger asChild>
              <Button>Add Liquidity</Button>
            </DialogTrigger>
            <DialogContent className="sm:max-w-[425px]">
              <DialogHeader>
                <DialogTitle>Add Liquidity</DialogTitle>
                <DialogDescription>
                  Fill in the details to add liquidity. Click save when you&apos;re done.
                </DialogDescription>
              </DialogHeader>

              <div className="grid gap-4 py-4">
                {error && (
                  <Alert variant="destructive">
                    <AlertCircle className="h-4 w-4" />
                    <AlertTitle>Error</AlertTitle>
                    <AlertDescription>{error}</AlertDescription>
                  </Alert>
                )}
                <div className="grid grid-cols-4 items-center gap-4">
                  <Label htmlFor="token_a_mint_account" className="text-right col-span-4">
                    Token A Mint Account *
                  </Label>
                  <Input
                    id="token_a_mint_account"
                    placeholder="Token A Mint Account *"
                    className="col-span-4"
                    value={addLiquidityData.token_a_mint_account}
                    onChange={handleAddLiquidityChange}
                  />
                </div>
                <div className="grid grid-cols-4 items-center gap-4">
                  <Label htmlFor="token_b_mint_account" className="text-right col-span-3">
                    Token B Mint Account *
                  </Label>
                  <Input
                    id="token_b_mint_account"
                    placeholder="Token B Mint Account *"
                    className="col-span-4"
                    value={addLiquidityData.token_b_mint_account}
                    onChange={handleAddLiquidityChange}
                  />
                </div>
                <div className="grid grid-cols-4 items-center gap-4">
                  <Label htmlFor="provider_token_a_account" className="text-right col-span-3">
                    provider token A account *
                  </Label>
                  <Input
                    id="provider_token_a_account"
                    placeholder="provider token A account *"
                    className="col-span-4"
                    value={addLiquidityData.provider_token_a_account}
                    onChange={handleAddLiquidityChange}
                  />
                </div>
                <div className="grid grid-cols-4 items-center gap-4">
                  <Label htmlFor="provider_token_b_account" className="text-right col-span-3">
                    provider token B account *
                  </Label>
                  <Input
                    id="provider_token_b_account"
                    placeholder="provider token B account *"
                    className="col-span-4"
                    value={addLiquidityData.provider_token_b_account}
                    onChange={handleAddLiquidityChange}
                  />
                </div>
                <div className="grid grid-cols-4 items-center gap-4">
                  <Label htmlFor="amount_a_max" className="text-right col-span-3">
                    Amount A Max *
                  </Label>
                  <Input
                    id="amount_a_max"
                    type="number"
                    placeholder="amount A max *"
                    className="col-span-4"
                    value={addLiquidityData.amount_a_max}
                    onChange={handleAddLiquidityChange}
                  />
                </div>
                <div className="grid grid-cols-4 items-center gap-4">
                  <Label htmlFor="amount_b_max" className="text-right col-span-3">
                    Amount B Max *
                  </Label>
                  <Input
                    id="amount_b_max"
                    type="number"
                    placeholder="Amount B Max *"
                    className="col-span-4"
                    value={addLiquidityData.amount_b_max}
                    onChange={handleAddLiquidityChange}
                  />
                </div>
                <div className="grid grid-cols-4 items-center gap-4">
                  <Label htmlFor="minimum_lp_tokens" className="text-right col-span-3">
                    Minimum Lp Tokens *
                  </Label>
                  <Input
                    id="minimum_lp_tokens"
                    type="number"
                    placeholder="Minimum Lp Tokens *"
                    className="col-span-4"
                    value={addLiquidityData.minimum_lp_tokens}
                    onChange={handleAddLiquidityChange}
                  />
                </div>
              </div>
              <DialogFooter>
                <Button type="submit" onClick={handleAddLiquidity}>
                  Add Liquidity
                </Button>
              </DialogFooter>
            </DialogContent>
          </Dialog>

          <Dialog>
            <DialogTrigger asChild>
              <Button>Withdraw Liquidity</Button>
            </DialogTrigger>
            <DialogContent className="sm:max-w-[425px]">
              <DialogHeader>
                <DialogTitle>Withdraw Liquidity</DialogTitle>
                <DialogDescription>
                  Fill in the details to withdraw liquidity. Click save when you&apos;re done.
                </DialogDescription>
              </DialogHeader>

              <div className="grid gap-4 py-4">
                {error && (
                  <Alert variant="destructive">
                    <AlertCircle className="h-4 w-4" />
                    <AlertTitle>Error</AlertTitle>
                    <AlertDescription>{error}</AlertDescription>
                  </Alert>
                )}
                <div className="grid grid-cols-4 items-center gap-4">
                  <Label htmlFor="token_a_mint_account" className="text-right col-span-4">
                    Token A Mint Account *
                  </Label>
                  <Input
                    id="token_a_mint_account"
                    placeholder="Token A Mint Account *"
                    className="col-span-4"
                    value={withdrawLiquidityData.token_a_mint_account}
                    onChange={handleWithdrawLiquidityChange}
                  />
                </div>
                <div className="grid grid-cols-4 items-center gap-4">
                  <Label htmlFor="token_b_mint_account" className="text-right col-span-3">
                    Token B Mint Account *
                  </Label>
                  <Input
                    id="token_b_mint_account"
                    placeholder="Token B Mint Account *"
                    className="col-span-4"
                    value={withdrawLiquidityData.token_b_mint_account}
                    onChange={handleWithdrawLiquidityChange}
                  />
                </div>
                <div className="grid grid-cols-4 items-center gap-4">
                  <Label htmlFor="provider_token_a_account" className="text-right col-span-3">
                    provider token A account *
                  </Label>
                  <Input
                    id="provider_token_a_account"
                    placeholder="provider token A account *"
                    className="col-span-4"
                    value={withdrawLiquidityData.provider_token_a_account}
                    onChange={handleWithdrawLiquidityChange}
                  />
                </div>
                <div className="grid grid-cols-4 items-center gap-4">
                  <Label htmlFor="provider_token_b_account" className="text-right col-span-3">
                    provider token B account *
                  </Label>
                  <Input
                    id="provider_token_b_account"
                    placeholder="provider token B account *"
                    className="col-span-4"
                    value={withdrawLiquidityData.provider_token_b_account}
                    onChange={handleWithdrawLiquidityChange}
                  />
                </div>
                <div className="grid grid-cols-4 items-center gap-4">
                  <Label htmlFor="amount_a_min" className="text-right col-span-3">
                    Amount A Min *
                  </Label>
                  <Input
                    id="amount_a_min"
                    type="number"
                    placeholder="amount A min *"
                    className="col-span-4"
                    value={withdrawLiquidityData.amount_a_min}
                    onChange={handleWithdrawLiquidityChange}
                  />
                </div>
                <div className="grid grid-cols-4 items-center gap-4">
                  <Label htmlFor="amount_b_min" className="text-right col-span-3">
                    Amount B Min *
                  </Label>
                  <Input
                    id="amount_b_min"
                    type="number"
                    placeholder="Amount B Min *"
                    className="col-span-4"
                    value={withdrawLiquidityData.amount_b_min}
                    onChange={handleWithdrawLiquidityChange}
                  />
                </div>
                <div className="grid grid-cols-4 items-center gap-4">
                  <Label htmlFor="maximum_lp_tokens" className="text-right col-span-3">
                    Maximum Lp Tokens *
                  </Label>
                  <Input
                    id="maximum_lp_tokens"
                    type="number"
                    placeholder="Maximum Lp Tokens *"
                    className="col-span-4"
                    value={withdrawLiquidityData.maximum_lp_tokens}
                    onChange={handleWithdrawLiquidityChange}
                  />
                </div>
              </div>
              <DialogFooter>
                <Button type="submit" onClick={handleWithdrawLiquidity}>
                  Withdraw Liquidity
                </Button>
              </DialogFooter>
            </DialogContent>
          </Dialog>

          <Dialog>
            <DialogTrigger asChild>
              <Button>Swap Tokens</Button>
            </DialogTrigger>
            <DialogContent className="sm:max-w-[425px]">
              <DialogHeader>
                <DialogTitle>Swap Tokens</DialogTitle>
                <DialogDescription>
                  Fill in the details to swap tokens. Click save when you&apos;re done.
                </DialogDescription>
              </DialogHeader>

              <div className="grid gap-4 py-4">
                {error && (
                  <Alert variant="destructive">
                    <AlertCircle className="h-4 w-4" />
                    <AlertTitle>Error</AlertTitle>
                    <AlertDescription>{error}</AlertDescription>
                  </Alert>
                )}
                <div className="grid grid-cols-4 items-center gap-4">
                  <Label htmlFor="swapper_token_a_account" className="text-right col-span-4">
                    Swapper Token A Account *
                  </Label>
                  <Input
                    id="swapper_token_a_account"
                    placeholder="Swapper Token A Account *"
                    className="col-span-4"
                    value={swapTokensData.swapper_token_a_account}
                    onChange={handleSwapTokensChange}
                  />
                </div>
                <div className="grid grid-cols-4 items-center gap-4">
                  <Label htmlFor="swapper_token_b_account" className="text-right col-span-3">
                    Swapper Token B Account *
                  </Label>
                  <Input
                    id="swapper_token_b_account"
                    placeholder="Swapper Token B Account *"
                    className="col-span-4"
                    value={swapTokensData.swapper_token_b_account}
                    onChange={handleSwapTokensChange}
                  />
                </div>
                <div className="grid grid-cols-4 items-center gap-4">
                  <Label htmlFor="token_a_mint_account" className="text-right col-span-3">
                    Token A Mint Account *
                  </Label>
                  <Input
                    id="token_a_mint_account"
                    placeholder="Token A Mint Account *"
                    className="col-span-4"
                    value={swapTokensData.token_a_mint_account}
                    onChange={handleSwapTokensChange}
                  />
                </div>
                <div className="grid grid-cols-4 items-center gap-4">
                  <Label htmlFor="token_b_mint_account" className="text-right col-span-3">
                    Token B Mint Account *
                  </Label>
                  <Input
                    id="token_b_mint_account"
                    placeholder="Token B Mint Account *"
                    className="col-span-4"
                    value={swapTokensData.token_b_mint_account}
                    onChange={handleSwapTokensChange}
                  />
                </div>
                <div className="grid grid-cols-4 items-center gap-4">
                  <Label htmlFor="max_amount_in" className="text-right col-span-3">
                    Max Amount In *
                  </Label>
                  <Input
                    id="max_amount_in"
                    type="number"
                    placeholder="Max Amount In *"
                    className="col-span-4"
                    value={swapTokensData.max_amount_in}
                    onChange={handleSwapTokensChange}
                  />
                </div>
                <div className="grid grid-cols-4 items-center gap-4">
                  <Label htmlFor="minimum_amount_out" className="text-right col-span-3">
                    Minimum Amount Out *
                  </Label>
                  <Input
                    id="minimum_amount_out"
                    type="number"
                    placeholder="Minimum Amount Out *"
                    className="col-span-4"
                    value={swapTokensData.minimum_amount_out}
                    onChange={handleSwapTokensChange}
                  />
                </div>
              </div>
              <DialogFooter>
                <Button type="submit" onClick={handleSwapTokens}>
                  Swap Tokens
                </Button>
              </DialogFooter>
            </DialogContent>
          </Dialog>
        </div>
      </div>
    </div >
  )
}
