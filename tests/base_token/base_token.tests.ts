import {expect, expectRevert, setupContract} from '../helpers'

describe('BASE_TOKEN', () => {
    async function setup() {
        return setupContract('base_token', 'new', '1000')
    }

    it('Should assign initial balance', async () => {
        const { query, defaultSigner: sender } = await setup()

        await expect(query.psp22BalanceOf(sender.address)).to.have.output(1000)
    })

    it('Should transfer', async () => {
        const { contract: token, query, defaultSigner: sender, accounts: [receiver] } = await setup()

        // sender has 1000 tokens
        await expect(query.psp22BalanceOf(sender.address)).to.have.output(1000)
        // receiver has 0 tokens
        await expect(query.psp22BalanceOf(receiver.address)).to.have.output(0)

        // sender sends tokens to the receiver
        await expect(token.tx.psp22Transfer(receiver.address, 100, [])).to.eventually.be.fulfilled

        // sender has 900 tokens
        await expect(query.psp22BalanceOf(sender.address)).to.have.output(900)
        // receiver has 100 tokens
        await expect(query.psp22BalanceOf(receiver.address)).to.have.output(100)
    })

})
