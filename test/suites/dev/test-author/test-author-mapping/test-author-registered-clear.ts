import "@moonbeam-network/api-augment";
import { alith, BALTATHAR_SESSION_ADDRESS } from "@moonwall/util";
import { expect, describeSuite } from "@moonwall/cli";
import { getMappingInfo } from "../../../../helpers/common.js";

describeSuite({
  id: "D226",
  title: "Author Mapping - registered author can clear (de register)",
  foundationMethods: "dev",
  testCases: ({ context, log, it }) => {
    it({
      id: "T01",
      title: "should succeed in clearing an association",
      test: async function () {
        const api = context.polkadotJs({ type: "moon" });
        await context.createBlock(
          api.tx.authorMapping.addAssociation(BALTATHAR_SESSION_ADDRESS).signAsync(alith)
        );
        expect((await getMappingInfo(context, BALTATHAR_SESSION_ADDRESS)).account).to.eq(
          alith.address
        );

        const {
          result: { events },
        } = await context.createBlock(
          api.tx.authorMapping.clearAssociation(BALTATHAR_SESSION_ADDRESS)
        );
        //check events
        expect(events.length === 6);
        expect(api.events.balances.Unreserved.is(events[1].event)).to.be.true;
        expect(api.events.authorMapping.KeysRemoved.is(events[2].event)).to.be.true;
        expect(api.events.treasury.Deposit.is(events[4].event)).to.be.true;
        expect(api.events.system.ExtrinsicSuccess.is(events[6].event)).to.be.true;

        // check mapping
        expect(await getMappingInfo(context, BALTATHAR_SESSION_ADDRESS)).to.eq(null);
      },
    });
  },
});
