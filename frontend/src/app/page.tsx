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
import initializeAmmAccount from '@/lib/actions/initializeAmmAccount'
import { AmmAccountData } from '@/lib/types'
import { useSignAndSendTransaction, useWalletUi, useWalletUiSigner } from '@wallet-ui/react'
import { useWalletUiGill, useWalletUiSignAndSend } from '@wallet-ui/react-gill'
import { getTransactionEncoder } from 'gill'
import DashboardFeature from '@/features/dashboard/dashboard-feature'

export default function Home() {
  const { connected } = useWalletUi()
  return connected ? <ConnectedComponent /> : <DashboardFeature />


}
export const maxDuration = 20000;
function ConnectedComponent() {
  const walletUi = useWalletUi()
  const client = useWalletUiGill()

  const signer = useWalletUiSigner({ account: walletUi.account! });
  const sender = useWalletUiSignAndSend();

  const [formData, setFormData] = useState({
    token_a_mint_account: '6pXNd5iDL3kqz8MxGiqZu4hb9vB9KbrPknGJ5vXfkMgd',
    token_b_mint_account: 'DB1xwND2situnNxmSCF3XNHnXXayLPoNSHDKJEfNiwpP',
    admin_token_a_account: 'cfqVCaPpWougv7Kq8kamHk3oZRNYhrTK6QURcACe1xt',
    admin_token_b_account: 'ADE1HAFKUiNSvWyLUDoQmLsNCqUKm297m6HRfx9GPynn',
    trade_fee: '100',
    initial_token_a_liquidity: '100',
    initial_token_b_liquidity: '1',
    // admin_account:''
  })
  const [error, setError] = useState<string | null>(null)

  const handleChange = (e: ChangeEvent<HTMLInputElement>) => {
    const { id, value } = e.target
    setFormData((prev) => ({ ...prev, [id]: value }))
  }

  const handleSubmit = async () => {
    const isAnyFieldEmpty = Object.values(formData).some((value) => value === '')
    // console.log(formData);
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
    // const { instruction } = 
    await initializeAmmAccount({ ...formData, admin_account: walletUi.account.address });
    // console.log(instruction);
    // console.log(instruction);
    // if (!walletUi.account || !instruction) {
    //   return;
    // }

    // const result = await sender([instruction!], signer);
    // console.log(result);
    // getTransactionEncoder(transaction)

    // signer.signAndSendTransactions(transaction);
    // transaction.sign([signer])
    // const legacyTransaction: LegacyTransaction =
    // client.sendAndConfirmTransaction(transaction);
    // console.log('Form Data:', formData)
  }

  return (
    <div className="container mx-auto p-4">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold">Dashboard</h1>
        <Dialog>
          <DialogTrigger asChild>
            <Button>create Market</Button>
          </DialogTrigger>
          <DialogContent className="sm:max-w-[425px]">
            <DialogHeader>
              <DialogTitle>Add New Item</DialogTitle>
              <DialogDescription>
                Fill in the details for the new item. Click save when you&apos;re done.
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
                  value={formData.token_a_mint_account}
                  onChange={handleChange}
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
                  value={formData.token_b_mint_account}
                  onChange={handleChange}
                />
              </div>
              <div className="grid grid-cols-4 items-center gap-4">
                <Label htmlFor="admin_token_a_account" className="text-right col-span-3">
                  admin token A account *
                </Label>
                <Input
                  id="admin_token_a_account"
                  placeholder="admin token A account *"
                  className="col-span-4"
                  value={formData.admin_token_a_account}
                  onChange={handleChange}
                />
              </div>
              <div className="grid grid-cols-4 items-center gap-4">
                <Label htmlFor="admin_token_b_account" className="text-right col-span-3">
                  admin token B account *
                </Label>
                <Input
                  id="admin_token_b_account"
                  placeholder="admin token B account *"
                  className="col-span-4"
                  value={formData.admin_token_b_account}
                  onChange={handleChange}
                />
              </div>
              <div className="grid grid-cols-4 items-center gap-4">
                <Label htmlFor="trade_fee" className="text-right col-span-3">
                  Trade Fee *
                </Label>
                <Input
                  id="trade_fee"
                  type="number"
                  placeholder="Trade Fee *"
                  className="col-span-4"
                  value={formData.trade_fee}
                  onChange={handleChange}
                />
              </div>
              <div className="grid grid-cols-4 items-center gap-4">
                <Label htmlFor="initial_token_a_liquidity" className="text-right col-span-3">
                  Initial Token A Liquidity *
                </Label>
                <Input
                  id="initial_token_a_liquidity"
                  type="number"
                  placeholder="Initial Token A Liquidity *"
                  className="col-span-4"
                  value={formData.initial_token_a_liquidity}
                  onChange={handleChange}
                />
              </div>
              <div className="grid grid-cols-4 items-center gap-4">
                <Label htmlFor="initial_token_b_liquidity" className="text-right col-span-3">
                  Initial Token B Liquidity *
                </Label>
                <Input
                  id="initial_token_b_liquidity"
                  type="number"
                  placeholder="Initial Token A Liquidity *"
                  className="col-span-4"
                  value={formData.initial_token_b_liquidity}
                  onChange={handleChange}
                />
              </div>
            </div>
            <DialogFooter>
              <Button type="submit" onClick={handleSubmit}>
                Save changes
              </Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>
      </div>
    </div >
  )
}
