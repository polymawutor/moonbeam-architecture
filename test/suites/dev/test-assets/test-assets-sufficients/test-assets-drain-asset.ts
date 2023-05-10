import "@moonbeam-network/api-augment";
import { u128 } from "@polkadot/types";
import { BN } from "@polkadot/util";
import { describeSuite, beforeAll, expect } from "@moonwall/cli";
import { alith, baltathar, generateKeyringPair, GLMR } from "@moonwall/util";
import { mockAssetBalance } from "../../../../helpers/assets.js";
import type { PalletAssetsAssetAccount, PalletAssetsAssetDetails } from "@polkadot/types/lookup";

const ARBITRARY_ASSET_ID = 42259045809535163221576417993425387648n;
const ARBITRARY_TRANSFER_AMOUNT = 10000000000000n;

describeSuite({
  id: "D131",
  title: "Pallet Assets - Sufficient tests: is_sufficient to true",
  foundationMethods: "dev",
  testCases: ({ context, it, log }) => {
    let assetId: u128;
    const freshAccount = generateKeyringPair();
    let api;

    beforeAll(async () => {
      api = context.polkadotJs({ type: "moon" });
      assetId = api.createType("u128", ARBITRARY_ASSET_ID);
      // We need to mint units with sudo.setStorage, as we dont have xcm mocker yet
      // And we need relay tokens for issuing a transaction to be executed in the relay
      const balance = new BN("100000000000000");
      const assetBalance: PalletAssetsAssetAccount = api.createType("PalletAssetsAssetAccount", {
        balance: balance,
      });

      const assetDetails: PalletAssetsAssetDetails = api.createType("PalletAssetsAssetDetails", {
        supply: balance,
        isSufficient: true,
        minBalance: 1,
      });

      await mockAssetBalance(
        context,
        assetBalance,
        assetDetails,
        alith,
        assetId,
        alith.address,
        true
      );

      await context.createBlock();
      const alithBalance = await api.query.assets.account(assetId.toU8a(), alith.address);
      expect(alithBalance.unwrap().balance.toBigInt()).to.equal(100000000000000n);
    });

    it({
      id: "T01",
      title: "Send MOVR and assets to an account, then drain assets, dont drain MOVR",
      test: async function () {
        // We are going to use a fresh account here, since we mocked the asset balance
        // We transfer Assets to freshAccount, which should increase sufficients

        await context.createBlock(
          api.tx.assets.transfer(assetId, freshAccount.address, ARBITRARY_TRANSFER_AMOUNT)
        );

        expect((await api.query.system.account(freshAccount.address)).sufficients.toBigInt()).to.eq(
          1n
        );
        // Providers should still be 0
        expect((await api.query.system.account(freshAccount.address)).providers.toBigInt()).to.eq(
          0n
        );

        // Lets transfer it the native token. We want to transfer enough to cover for a future fee.
        const fee = (
          await api.tx.assets
            .transfer(assetId, baltathar.address, ARBITRARY_TRANSFER_AMOUNT)
            .paymentInfo(freshAccount)
        ).partialFee.toBigInt();

        await context.createBlock(api.tx.balances.transfer(freshAccount.address, fee));

        expect((await api.query.system.account(freshAccount.address)).sufficients.toBigInt()).to.eq(
          1n
        );
        // Providers should now be 1
        expect((await api.query.system.account(freshAccount.address)).providers.toBigInt()).to.eq(
          1n
        );

        // What happens now when we execute such transaction? both MOVR and Assets should be drained.
        await context.createBlock(
          api.tx.assets
            .transfer(assetId, baltathar.address, ARBITRARY_TRANSFER_AMOUNT)
            .signAsync(freshAccount)
        );

        const freshAccountBalance = await api.query.assets.account(
          assetId.toU8a(),
          freshAccount.address
        );
        expect(freshAccountBalance.isNone).to.equal(true);

        // Sufficients should go to 0
        expect((await api.query.system.account(freshAccount.address)).sufficients.toBigInt()).to.eq(
          0n
        );
        // Providers should be 1
        expect((await api.query.system.account(freshAccount.address)).providers.toBigInt()).to.eq(
          1n
        );

        // Nonce should be 1
        expect((await api.query.system.account(freshAccount.address)).providers.toBigInt()).to.eq(
          1n
        );

        // But balance of MOVR should be 0
        expect(
          (await api.query.system.account(freshAccount.address)).data.free.toBigInt() > 0n
        ).to.eq(true);
      },
    });
  },
});
