'use client'

import { useState, ChangeEvent } from 'react'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { toast } from 'sonner'
import { AlertCircle } from 'lucide-react'
import initializeAmmAccount from '@/lib/actions/initializeAmmAccount'
import createTokenMint from '@/lib/actions/createTokenMint'
import { useWalletUi, useWalletUiSigner } from '@wallet-ui/react'
import { useWalletUiSignAndSend } from '@wallet-ui/react-gill'
import {
    Dialog,
    DialogContent,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from "@/components/ui/dialog"
import { useRouter } from 'next/navigation'

const ADMIN_ADDRESS = 'GK5uAKRv4Abn4szsDuhvcBDoYp8cwAcggkkynMjmSZf3'

const dummyMarkets = [
    {
        id: 1,
        tokenAMint: '6pXNd5...fkMgd',
        tokenBMint: 'DB1xwN...iwpP',
        poolAmountA: 1000,
        poolAmountB: 2000,
        totalLpTokens: 5000
    },
    {
        id: 2,
        tokenAMint: 'SolanaMint',
        tokenBMint: 'USDCMint',
        poolAmountA: 500,
        poolAmountB: 10000,
        totalLpTokens: 2500
    }
]
export default function AdminPage() {
    const { connected } = useWalletUi()
    return connected ? <Admin /> : <>wallet not connected</>

}
function Admin() {
    const { connected, account } = useWalletUi()
    // const router = useRouter()
    // if (!account) {
    //     return router.push("/")
    // }
    const signer = useWalletUiSigner({ account: account! });
    const sender = useWalletUiSignAndSend();
    const [createMarketData, setCreateMarketData] = useState({
        token_a_mint_account: '6pXNd5iDL3kqz8MxGiqZu4hb9vB9KbrPknGJ5vXfkMgd',
        token_b_mint_account: 'DB1xwND2situnNxmSCF3XNHnXXayLPoNSHDKJEfNiwpP',
        admin_token_a_account: 'cfqVCaPpWougv7Kq8kamHk3oZRNYhrTK6QURcACe1xt',
        admin_token_b_account: 'ADE1HAFKUiNSvWyLUDoQmLsNCqUKm297m6HRfx9GPynn',
        trade_fee: '100',
        initial_token_a_liquidity: '100',
        initial_token_b_liquidity: '1',
    })
    const [createTokenMintData, setCreateTokenMintData] = useState({
        decimals: '9',
        name: '',
    })
    const [error, setError] = useState<string | null>(null)
    const [msg, setMsg] = useState<string | null>(null)
    const [isCreateMarketOpen, setIsCreateMarketOpen] = useState(false)
    const [isCreateTokenMintOpen, setIsCreateTokenMintOpen] = useState(false)

    const handleCreateMarketChange = (e: ChangeEvent<HTMLInputElement>) => {
        const { id, value } = e.target
        setCreateMarketData((prev) => ({ ...prev, [id]: value }))
    }

    const handleCreateTokenMintChange = (e: ChangeEvent<HTMLInputElement>) => {
        const { id, value } = e.target
        setCreateTokenMintData((prev) => ({ ...prev, [id]: value }))
    }

    const handleCreateMarketSubmit = async () => {
        const isAnyFieldEmpty = Object.values(createMarketData).some((value) => value === '')

        if (isAnyFieldEmpty) {
            const errorMessage = 'All fields are required. Please fill in all the details.'
            setError(errorMessage)
            toast.error(errorMessage)
            return
        }
        if (!account) {
            setError("Wallet not connected!");
            toast.error("Wallet not connected!");
            return;
        }
        setError(null)

        const instruction = await initializeAmmAccount({ ...createMarketData, admin_account: account.address });
        if (!account || !instruction) {
            return;
        }

        sender([instruction!], signer);
        toast.success("Market initialization requested!");
        setIsCreateMarketOpen(false);
    }

    const handleCreateTokenMintSubmit = async () => {
        const { decimals, name } = createTokenMintData
        if (!decimals || !name) {
            const errorMessage = 'All fields are required.'
            setMsg(errorMessage)
            toast.error(errorMessage)
            return
        }

        if (!account) {
            toast.error("Wallet not connected!");
            return;
        }

        await createTokenMint(createTokenMintData);
        toast.success(`Token Mint "${name}" creation requested!`);
        setIsCreateTokenMintOpen(false);
        setMsg(null)
    }

    if (!connected) {
        return (
            <div className="flex flex-col items-center justify-center min-h-screen p-4">
                <h1 className="text-2xl font-bold mb-4">Admin Access Required</h1>
                <p>Please connect your wallet to verify admin privileges.</p>
            </div>
        )
    }

    if (account?.address !== ADMIN_ADDRESS) {
        return (
            <div className="flex flex-col items-center justify-center min-h-screen p-4">
                <h1 className="text-2xl font-bold mb-4 text-red-600">Access Denied</h1>
                <p>Your wallet address {account?.address} is not authorized to view this page.</p>
            </div>
        )
    }

    return (
        <div className="container mx-auto p-4 max-w-4xl">
            <h1 className="text-3xl font-bold mb-8">Admin Dashboard</h1>

            <div className="flex flex-col gap-6">

                <div className="flex gap-4">
                    <Dialog open={isCreateMarketOpen} onOpenChange={setIsCreateMarketOpen}>
                        <DialogTrigger asChild>
                            <Button>Create Market</Button>
                        </DialogTrigger>
                        <DialogContent className="max-w-2xl max-h-[90vh] overflow-y-auto">
                            <DialogHeader>
                                <DialogTitle>Create New Market</DialogTitle>
                            </DialogHeader>
                            <div className="grid gap-6 py-4">
                                {error && (
                                    <Alert variant="destructive">
                                        <AlertCircle className="h-4 w-4" />
                                        <AlertTitle>Error</AlertTitle>
                                        <AlertDescription>{error}</AlertDescription>
                                    </Alert>
                                )}

                                <div className="grid gap-2">
                                    <Label htmlFor="token_a_mint_account">Token A Mint Account *</Label>
                                    <Input
                                        id="token_a_mint_account"
                                        value={createMarketData.token_a_mint_account}
                                        onChange={handleCreateMarketChange}
                                    />
                                </div>

                                <div className="grid gap-2">
                                    <Label htmlFor="token_b_mint_account">Token B Mint Account *</Label>
                                    <Input
                                        id="token_b_mint_account"
                                        value={createMarketData.token_b_mint_account}
                                        onChange={handleCreateMarketChange}
                                    />
                                </div>

                                <div className="grid gap-2">
                                    <Label htmlFor="admin_token_a_account">Admin Token A Account *</Label>
                                    <Input
                                        id="admin_token_a_account"
                                        value={createMarketData.admin_token_a_account}
                                        onChange={handleCreateMarketChange}
                                    />
                                </div>

                                <div className="grid gap-2">
                                    <Label htmlFor="admin_token_b_account">Admin Token B Account *</Label>
                                    <Input
                                        id="admin_token_b_account"
                                        value={createMarketData.admin_token_b_account}
                                        onChange={handleCreateMarketChange}
                                    />
                                </div>

                                <div className="grid gap-2">
                                    <Label htmlFor="trade_fee">Trade Fee *</Label>
                                    <Input
                                        id="trade_fee"
                                        type="number"
                                        value={createMarketData.trade_fee}
                                        onChange={handleCreateMarketChange}
                                    />
                                </div>

                                <div className="grid gap-2">
                                    <Label htmlFor="initial_token_a_liquidity">Initial Token A Liquidity *</Label>
                                    <Input
                                        id="initial_token_a_liquidity"
                                        type="number"
                                        value={createMarketData.initial_token_a_liquidity}
                                        onChange={handleCreateMarketChange}
                                    />
                                </div>

                                <div className="grid gap-2">
                                    <Label htmlFor="initial_token_b_liquidity">Initial Token B Liquidity *</Label>
                                    <Input
                                        id="initial_token_b_liquidity"
                                        type="number"
                                        value={createMarketData.initial_token_b_liquidity}
                                        onChange={handleCreateMarketChange}
                                    />
                                </div>

                                <Button className="w-full mt-4" onClick={handleCreateMarketSubmit}>
                                    Initialize Market
                                </Button>
                            </div>
                        </DialogContent>
                    </Dialog>

                    <Dialog open={isCreateTokenMintOpen} onOpenChange={setIsCreateTokenMintOpen}>
                        <DialogTrigger asChild>
                            <Button variant="secondary">Create Token Mint</Button>
                        </DialogTrigger>
                        <DialogContent>
                            <DialogHeader>
                                <DialogTitle>Create Token Mint Account</DialogTitle>
                            </DialogHeader>
                            <div className="grid gap-6 py-4">
                                {msg && (
                                    <Alert variant="destructive">
                                        <AlertCircle className="h-4 w-4" />
                                        <AlertTitle>Error</AlertTitle>
                                        <AlertDescription>{msg}</AlertDescription>
                                    </Alert>
                                )}
                                <div className="grid gap-2">
                                    <Label htmlFor="name">Token Name *</Label>
                                    <Input
                                        id="name"
                                        value={createTokenMintData.name}
                                        onChange={handleCreateTokenMintChange}
                                        placeholder="e.g. My Token"
                                    />
                                </div>
                                <div className="grid gap-2">
                                    <Label htmlFor="decimals">Decimals *</Label>
                                    <Input
                                        id="decimals"
                                        type="number"
                                        value={createTokenMintData.decimals}
                                        onChange={handleCreateTokenMintChange}
                                        placeholder="9"
                                    />
                                </div>
                                <Button className="w-full mt-4" onClick={handleCreateTokenMintSubmit}>
                                    Create Mint
                                </Button>
                            </div>
                        </DialogContent>
                    </Dialog>
                </div>

                <div className="bg-card text-card-foreground shadow-sm border rounded-lg p-6">
                    <h2 className="text-xl font-semibold mb-4">Market Overview</h2>
                    <div className="relative w-full overflow-auto">
                        <table className="w-full caption-bottom text-sm text-left">
                            <thead className="[&_tr]:border-b">
                                <tr className="border-b transition-colors hover:bg-muted/50 data-[state=selected]:bg-muted">
                                    <th className="h-12 px-4 align-middle font-medium text-muted-foreground">ID</th>
                                    <th className="h-12 px-4 align-middle font-medium text-muted-foreground">Token A Mint</th>
                                    <th className="h-12 px-4 align-middle font-medium text-muted-foreground">Token B Mint</th>
                                    <th className="h-12 px-4 align-middle font-medium text-muted-foreground text-right">Pool A</th>
                                    <th className="h-12 px-4 align-middle font-medium text-muted-foreground text-right">Pool B</th>
                                    <th className="h-12 px-4 align-middle font-medium text-muted-foreground text-right">Total LP</th>
                                </tr>
                            </thead>
                            <tbody className="[&_tr:last-child]:border-0">
                                {dummyMarkets.map((market) => (
                                    <tr key={market.id} className="border-b transition-colors hover:bg-muted/50 data-[state=selected]:bg-muted">
                                        <td className="p-4 align-middle">{market.id}</td>
                                        <td className="p-4 align-middle font-mono">{market.tokenAMint}</td>
                                        <td className="p-4 align-middle font-mono">{market.tokenBMint}</td>
                                        <td className="p-4 align-middle text-right">{market.poolAmountA}</td>
                                        <td className="p-4 align-middle text-right">{market.poolAmountB}</td>
                                        <td className="p-4 align-middle text-right">{market.totalLpTokens}</td>
                                    </tr>
                                ))}
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        </div>
    )
}
