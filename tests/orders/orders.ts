import {expect, fromSigner, setupContract} from '../helpers'

describe('ORDERS', () => {
    async function setup() {
        // Setup the 3 token contracts
        const baseToken = await setupContract('base_token', 'new', '1000')
        const quotedToken = await setupContract('quoted_token', 'new', '1000')
        const phalanxToken = await setupContract('phalanx_token', 'new', '1000')

        const orders = await setupContract('orders', 'new',
                                                  baseToken.contract.address, 
                                                  quotedToken.contract.address, 
                                                  phalanxToken.contract.address)

        return { baseToken, quotedToken, phalanxToken, orders, 
            alice: baseToken.defaultSigner, 
            bob: quotedToken.defaultSigner, 
            charlie: phalanxToken.defaultSigner, 
            dave: orders.defaultSigner }
    }

    it('Should fill order queuen, then match orders', async () => {
        const {baseToken, quotedToken, phalanxToken, orders, alice, bob, charlie, dave} = await setup()

        // The balance of baseToken for Alice account should be 1000
        await expect(fromSigner(baseToken.contract, alice.address).query.psp22BalanceOf(alice.address)).to.have.output(1000)
        // The balance of quotedToken for Bob account should be 1000
        await expect(fromSigner(quotedToken.contract, bob.address).query.psp22BalanceOf(bob.address)).to.have.output(1000)
        // The balance of quotedToken for Bob account should be 1000
        await expect(fromSigner(phalanxToken.contract, charlie.address).query.psp22BalanceOf(charlie.address)).to.have.output(1000)

        // Allow orders contract to spend baseToken on behalf of Alice
        await expect(fromSigner(baseToken.contract, alice.address).tx.psp22Approve(orders.contract.address, 100)).to.eventually.be.fulfilled
        // Allow tradeTokens contract to spend quotedToken on behalf of Bob
        await expect(fromSigner(quotedToken.contract, bob.address).tx.psp22Approve(orders.contract.address, 200)).to.eventually.be.fulfilled

        // Alice enters a Bid Order (Side = 0) for 100 base tokens
         await expect(fromSigner(orders.contract, alice.address).tx.ordersOrder(0, 100)).to.eventually.be.fulfilled
        // Check queue status
        // Ask amount = 0
        // Bid amount = 100
        // Ask length = 0
        // Bid length = 1
        await expect(fromSigner(orders.contract, alice.address).query.ordersQueueGetTotalAmount(0)).to.have.output(100)
        await expect(fromSigner(orders.contract, alice.address).query.ordersQueueGetTotalAmount(1)).to.have.output(0)
        await expect(fromSigner(orders.contract, alice.address).query.ordersQueueGetLength(0)).to.have.output(1)
        await expect(fromSigner(orders.contract, alice.address).query.ordersQueueGetLength(1)).to.have.output(0)
        // Check orders by account
        // Alice should have Some order
        // Bob should have None order
        console.log("Test")
        await fromSigner(orders.contract, alice.address).query.ordersOrderGet(alice.address).then((queryOut) => console.log("Test:" + queryOut.output.toHuman()))
        // await expect(fromSigner(orders.contract, alice.address).query.ordersOrderGet(alice.address)).to.be.isSome
        // await expect(fromSigner(orders.contract, alice.address).query.ordersOrderGet(bob.address)).to.be.isNone


        // Bob enters a Bid Order (Side = 0) for 160 base tokens
        await expect(fromSigner(orders.contract, bob.address).tx.ordersOrder(0, 160)).to.eventually.be.fulfilled
        // Check queue status
        // Ask amount = 0
        // Bid amount = 260
        // Ask length = 0
        // Bid length = 2
        await expect(fromSigner(orders.contract, alice.address).query.ordersQueueGetTotalAmount(0)).to.have.output(260)
        await expect(fromSigner(orders.contract, alice.address).query.ordersQueueGetTotalAmount(1)).to.have.output(0)
        await expect(fromSigner(orders.contract, alice.address).query.ordersQueueGetLength(0)).to.have.output(2)
        await expect(fromSigner(orders.contract, alice.address).query.ordersQueueGetLength(1)).to.have.output(0)


        // Charlie enters a Ask Order (Side = 1) for 200 base tokens
        await expect(fromSigner(orders.contract, charlie.address).tx.ordersOrder(1, 200)).to.eventually.be.fulfilled
        // Check queue status
        // Ask amount = 200
        // Bid amount = 260
        // Ask length = 1
        // Bid length = 2
        await expect(fromSigner(orders.contract, alice.address).query.ordersQueueGetTotalAmount(0)).to.have.output(260)
        await expect(fromSigner(orders.contract, alice.address).query.ordersQueueGetTotalAmount(1)).to.have.output(200)
        await expect(fromSigner(orders.contract, alice.address).query.ordersQueueGetLength(0)).to.have.output(2)
        await expect(fromSigner(orders.contract, alice.address).query.ordersQueueGetLength(1)).to.have.output(1)

        // Dave enters a Ask Order (Side = 1) for 1 base tokens
        await expect(fromSigner(orders.contract, dave.address).tx.ordersOrder(1, 1)).to.eventually.be.fulfilled
        // Check queue status
        // Ask amount = 201
        // Bid amount = 260
        // Ask length = 2
        // Bid length = 2
        await expect(fromSigner(orders.contract, alice.address).query.ordersQueueGetTotalAmount(0)).to.have.output(260)
        await expect(fromSigner(orders.contract, alice.address).query.ordersQueueGetTotalAmount(1)).to.have.output(201)
        await expect(fromSigner(orders.contract, alice.address).query.ordersQueueGetLength(0)).to.have.output(2)
        await expect(fromSigner(orders.contract, alice.address).query.ordersQueueGetLength(1)).to.have.output(2)

        // Dave modifies his order
        await expect(fromSigner(orders.contract, dave.address).tx.ordersOrder(1, 100)).to.eventually.be.fulfilled
        // Check queue status
        // Ask amount = 300
        // Bid amount = 260
        // Ask length = 2
        // Bid length = 2
        await expect(fromSigner(orders.contract, alice.address).query.ordersQueueGetTotalAmount(0)).to.have.output(260)
        await expect(fromSigner(orders.contract, alice.address).query.ordersQueueGetTotalAmount(1)).to.have.output(300)
        await expect(fromSigner(orders.contract, alice.address).query.ordersQueueGetLength(0)).to.have.output(2)
        await expect(fromSigner(orders.contract, alice.address).query.ordersQueueGetLength(1)).to.have.output(2)
       
        // Dave cancels his order
        await expect(fromSigner(orders.contract, dave.address).tx.ordersOrderCancel()).to.eventually.be.fulfilled
        // Check queue status
        // Ask amount = 200
        // Bid amount = 260
        // Ask length = 1
        // Bid length = 2
        await expect(fromSigner(orders.contract, alice.address).query.ordersQueueGetTotalAmount(0)).to.have.output(260)
        await expect(fromSigner(orders.contract, alice.address).query.ordersQueueGetTotalAmount(1)).to.have.output(200)
        await expect(fromSigner(orders.contract, alice.address).query.ordersQueueGetLength(0)).to.have.output(2)
        await expect(fromSigner(orders.contract, alice.address).query.ordersQueueGetLength(1)).to.have.output(1)


        // After transferring of 100 tokens the balance of Alice account should be 900
        // await expect(fromSigner(usdToken.contract, alice.address).query.psp22BalanceOf(alice.address)).to.have.output(900)

        // // Check the amount of usd tokens of wrapped contract
        // await expect(usdToken.query.psp22BalanceOf(wrapperUsd.contract.address)).to.have.output(100)

        // // Check Alice's amount of wrapped
        // await expect(wrapperUsd.query.psp22BalanceOf(alice.address)).to.have.output(100)
    })


    // it('Should setup orders a trade of tokens', async () => {
    //     const {baseToken, quotedToken, phalanxToken, orders, alice, bob, charlie, dave} = await setup()

    //     // The balance of baseToken for Alice account should be 1000
    //     await expect(fromSigner(baseToken.contract, alice.address).query.psp22BalanceOf(alice.address)).to.have.output(1000)
    //     // The balance of quotedToken for Bob account should be 1000
    //     await expect(fromSigner(quotedToken.contract, bob.address).query.psp22BalanceOf(bob.address)).to.have.output(1000)
    //     // The balance of quotedToken for Bob account should be 1000
    //     await expect(fromSigner(phalanxToken.contract, charlie.address).query.psp22BalanceOf(charlie.address)).to.have.output(1000)

    //     // Allow tradeTokens contract to spend baseToken on behalf of Alice
    //     await expect(fromSigner(baseToken.contract, alice.address).tx.psp22Approve(tradeTokens.contract.address, 100)).to.eventually.be.fulfilled
    //     // Allow tradeTokens contract to spend quotedToken on behalf of Bob
    //     await expect(fromSigner(quotedToken.contract, bob.address).tx.psp22Approve(tradeTokens.contract.address, 200)).to.eventually.be.fulfilled

    //     // Let tradeTokens contract execute trade of 100 baseToken from Alice with 200 quotedToken from Bob (baseToken/quotedToken price = 2) 
    //     await expect(fromSigner(tradeTokens.contract, dave.address).tx.tradePSP22TokensTradeTokens(alice.address, bob.address, 100, 2)).to.eventually.be.fulfilled

        // // After transferring of 100 tokens the balance of Alice account should be 900
        // await expect(fromSigner(usdToken.contract, alice.address).query.psp22BalanceOf(alice.address)).to.have.output(900)

        // // Check the amount of usd tokens of wrapped contract
        // await expect(usdToken.query.psp22BalanceOf(wrapperUsd.contract.address)).to.have.output(100)

        // // Check Alice's amount of wrapped
        // await expect(wrapperUsd.query.psp22BalanceOf(alice.address)).to.have.output(100)
    // })

    // it('Should withdraw to signer', async () => {
    //     const {usdToken, wrapperUsd, alice} = await setup()

    //     // Check Alice balance of usd Token
    //     await expect(usdToken.query.psp22BalanceOf(usdToken.contract.signer)).to.have.output(1000)

    //     // Allow wrappedUst to spend token on behalf of Alice
    //     await expect(fromSigner(usdToken.contract, alice.address).tx.psp22Approve(wrapperUsd.contract.address, 100)).to.eventually.be.fulfilled

    //     // Deposit wrapped for Alice
    //     await expect(fromSigner(wrapperUsd.contract, alice.address).tx.wrappedPSP22DepositFor(usdToken.contract.signer, 100)).to.eventually.be.fulfilled

    //     // Alice withdraws usd token
    //     await expect(fromSigner(wrapperUsd.contract, alice.address).tx.wrappedPSP22WithdrawTo(usdToken.contract.signer, 100)).to.eventually.be.fulfilled

    //     // Check the amount of usd tokens of wrapped contract
    //     await expect(usdToken.query.psp22BalanceOf(wrapperUsd.contract.address)).to.have.output(0)

    //     // Check Alice's amount of wrapped token
    //     await expect(wrapperUsd.query.psp22BalanceOf(alice.address)).to.have.output(0)

    //     // Check Alice's balance of usd token
    //     await expect(usdToken.query.psp22BalanceOf(alice.address)).to.have.output(1000)
    // })

    // it('Bob can withdraw deposited tokens', async () => {
    //     const {usdToken, wrapperUsd, alice, bob} = await setup()

    //     // Check Alice balance of usd Token
    //     await expect(usdToken.query.psp22BalanceOf(usdToken.contract.signer)).to.have.output(1000)

    //     // Allow wrappedUst to spend token on behalf of Alice
    //     await expect(fromSigner(usdToken.contract, alice.address).tx.psp22Approve(wrapperUsd.contract.address, 100)).to.eventually.be.fulfilled

    //     // Deposit wrapped for Alice
    //     await expect(fromSigner(wrapperUsd.contract, alice.address).tx.wrappedPSP22DepositFor(usdToken.contract.signer, 100)).to.eventually.be.fulfilled

    //     // Alice transfers wrapped tokens to Bob
    //     await expect(fromSigner(wrapperUsd.contract, alice.address).tx.psp22Transfer(bob.address, 100, [])).to.eventually.be.fulfilled

    //     // Check Alice amount of wrapped tokens after transfer
    //     await expect(wrapperUsd.query.psp22BalanceOf(alice.address)).to.have.output(0)

    //     // Check Bob amount of wrapped tokens after transfer
    //     await expect(wrapperUsd.query.psp22BalanceOf(bob.address)).to.have.output(100)

    //     // Bob withdraws usd token
    //     await expect(fromSigner(wrapperUsd.contract, bob.address).tx.wrappedPSP22WithdrawTo(bob.address, 100)).to.eventually.be.fulfilled

    //     // Check Bob amount of wrapped
    //     await expect(wrapperUsd.query.psp22BalanceOf(bob.address)).to.have.output(0)

    //     // Check Bob balance of usd Token
    //     await expect(usdToken.query.psp22BalanceOf(bob.address)).to.have.output(100)
    // })
})
